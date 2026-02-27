use std::rc::Rc;

use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    enabledness_xor_join_only, number_of_transitions_xor_join_only,
    parser::parser_state::GlobalIndex,
    semantics::{BPMNMarking, BPMNSubMarking, TransitionIndex},
    sequence_flow::BPMNSequenceFlow,
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        startable::{InitiationMode, Startable},
        transitionable::Transitionable,
    },
    verify_structural_correctness_initiation_mode,
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

#[derive(Debug, Clone)]
pub struct BPMNExpandedSubProcess {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) name: Option<String>,
    pub(crate) elements: Vec<BPMNElement>,
    ///internal sequence flows
    pub(crate) sequence_flows: Vec<BPMNSequenceFlow>,

    //external sequence flows
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNExpandedSubProcess {
    pub(crate) fn start_process_instance(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
        root_marking: Rc<BPMNMarking>,
    ) -> Result<BPMNSubMarking> {
        let initiation_mode = self.initiation_mode(bpmn)?;
        self.to_sub_marking(initiation_mode, root_marking)
    }
}

impl BPMNElementTrait for BPMNExpandedSubProcess {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> anyhow::Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "expanded sub-processes cannot have incoming message flows"
        ))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "expanded sub-processes cannot have outgoing message flows"
        ))
    }

    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        //recurse on elements
        for element in &self.elements {
            element.verify_structural_correctness(self, bpmn)?
        }

        //verify initiation and termination
        verify_structural_correctness_initiation_mode!(self, bpmn);

        Ok(())
    }
}

impl BPMNObject for BPMNExpandedSubProcess {
    fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn local_index(&self) -> usize {
        self.local_index
    }

    fn is_unconstrained_start_event(
        &self,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<bool> {
        Ok(false)
    }

    fn is_end_event(&self) -> bool {
        false
    }

    fn incoming_sequence_flows(&self) -> &[usize] {
        &self.incoming_sequence_flows
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &self.outgoing_sequence_flows
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn can_start_process_instance(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        Ok(self.incoming_sequence_flows().len() == 0)
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        false
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        true
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        true
    }
}

impl Transitionable for BPMNExpandedSubProcess {
    fn number_of_transitions(&self, marking: &BPMNSubMarking) -> usize {
        //behaves like an XOR-join to start
        let mut result = number_of_transitions_xor_join_only!(self);

        for sub_marking in &marking.element_index_2_sub_markings[self.local_index] {
            // one transition to end the instantiation
            result += 1;
            // and the transitions within us
            result += self.elements.number_of_transitions(sub_marking);
        }

        result
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        //start transitions: like an xor join
        let mut result = enabledness_xor_join_only!(self, marking);

        //gather sub-process instantations transitions
        for sub_marking in &marking.element_index_2_sub_markings[self.local_index] {
            let sub_marking_enabled_transitions =
                self.elements.enabled_transitions(sub_marking, self, bpmn)?;

            //end transition
            if sub_marking_enabled_transitions.not_any() {
                result.push(true);
            } else {
                result.push(false);
            }

            //transitions from this instantiation
            result.extend(sub_marking_enabled_transitions);
        }

        Ok(result)
    }

    fn transition_activity(
        &self,
        mut transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        //start transition
        if transition_index < number_of_transitions_xor_join_only!(self) {
            return None;
        }
        transition_index -= number_of_transitions_xor_join_only!(self);

        for sub_marking in &marking.element_index_2_sub_markings[self.local_index] {
            if transition_index == 0 {
                //end transition
                return None;
            }
            transition_index -= 1;

            //own transitions
            let sub_number_of_transitions = self.elements.number_of_transitions(&sub_marking);
            if transition_index < sub_number_of_transitions {
                return self
                    .elements
                    .transition_activity(transition_index, &sub_marking);
            }
            transition_index -= sub_number_of_transitions;
        }
        None
    }

    fn transition_debug(
        &self,
        mut transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
    ) -> Option<String> {
        //start transition
        if transition_index < self.incoming_sequence_flows.len().max(1) {
            return Some(format!(
                "expanded sub-process `{}`; start internal transition {}",
                self.id, transition_index
            ));
        }
        transition_index -= self.incoming_sequence_flows.len().max(1);

        //instantiations
        for (i, sub_marking) in marking.element_index_2_sub_markings[self.local_index]
            .iter()
            .enumerate()
        {
            if transition_index == 0 {
                //end transition
                return Some(format!(
                    "expanded sub-process `{}`; instantiation {}, end transition",
                    self.id, i
                ));
            }
            transition_index -= 1;

            //own transitions
            let sub_number_of_transitions = self.elements.number_of_transitions(&sub_marking);
            if transition_index < sub_number_of_transitions {
                return self
                    .elements
                    .transition_debug(transition_index, &sub_marking);
            }
            transition_index -= sub_number_of_transitions;
        }
        None
    }
}

impl Startable for BPMNExpandedSubProcess {
    fn unconstrained_start_events_without_recursing(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<&BPMNElement>> {
        self.elements
            .unconstrained_start_events_without_recursing(bpmn)
    }

    fn end_events_without_recursing(&self) -> Vec<&BPMNElement> {
        self.elements.end_events_without_recursing()
    }

    fn start_elements_without_recursing(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<&BPMNElement>> {
        self.elements.start_elements_without_recursing(bpmn)
    }
}

#[macro_export]
macro_rules! to_sub_marking {
    ($self:ident, $initiation_mode:ident, $root_marking:ident) => {
        match $initiation_mode {
            InitiationMode::ChoiceBetweenStartEvents() => {
                //initiation mode 1: through one or more start events
                Ok(BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0; $self.sequence_flows_non_recursive().len()],
                    initial_choice_token: true,
                    element_index_2_tokens: vec![0; $self.elements_non_recursive().len()],
                    element_index_2_sub_markings: vec![
                        vec![];
                        $self.elements_non_recursive().len()
                    ],
                    root_marking: $root_marking,
                })
            }
            InitiationMode::ParallelElements(elements) => {
                let mut element_index_2_tokens = vec![0; $self.elements_non_recursive().len()];
                for element in elements {
                    element_index_2_tokens[element.local_index()] = 1;
                }

                Ok(BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0; $self.sequence_flows_non_recursive().len()],
                    initial_choice_token: false,
                    element_index_2_tokens,
                    element_index_2_sub_markings: vec![
                        vec![];
                        $self.elements_non_recursive().len()
                    ],
                    root_marking: $root_marking,
                })
            }
        }
    };
}

impl Processable for BPMNExpandedSubProcess {
    fn elements_non_recursive(&self) -> &Vec<BPMNElement> {
        &self.elements
    }

    fn sequence_flows_non_recursive(&self) -> &Vec<BPMNSequenceFlow> {
        &self.sequence_flows
    }

    fn to_sub_marking(
        &self,
        initiation_mode: InitiationMode,
        root_marking: Rc<BPMNMarking>,
    ) -> Result<BPMNSubMarking> {
        to_sub_marking!(self, initiation_mode, root_marking)
    }
}
