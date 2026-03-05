use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    parser::parser_state::GlobalIndex,
    semantics::{BPMNRootMarking, BPMNSubMarking, TransitionIndex},
    sequence_flow::BPMNSequenceFlow,
    to_sub_marking,
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        searchable::Searchable,
        startable::{InitiationMode, Startable},
        transitionable::Transitionable,
    },
    verify_structural_correctness_initiation_mode,
};
use anyhow::{Result, anyhow};
use bitvec::prelude::BitVec;
use ebi_activity_key::Activity;
use ebi_arithmetic::Fraction;

/// more common name: pool
#[derive(Clone, Debug)]
pub struct BPMNProcess {
    pub global_index: GlobalIndex,
    pub id: String,
    pub local_index: usize,
    pub elements: Vec<BPMNElement>,
    pub sequence_flows: Vec<BPMNSequenceFlow>,
}

impl Searchable for BPMNProcess {
    fn index_2_object(&self, index: GlobalIndex) -> Option<&dyn BPMNObject> {
        if self.global_index == index {
            return Some(self);
        }
        self.elements.index_2_object(index)
    }

    fn id_2_pool_and_global_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)> {
        if self.id == id {
            Some((Some(self.local_index), self.global_index))
        } else {
            if let Some((_, index)) = self.elements.id_2_pool_and_global_index(id) {
                Some((Some(self.local_index), index))
            } else {
                None
            }
        }
    }

    fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, &dyn Processable)> {
        for sequence_flow in &self.sequence_flows {
            if sequence_flow.global_index == sequence_flow_global_index {
                return Some((sequence_flow, self));
            }
        }
        None
    }

    fn id_2_local_index(&self, id: &str) -> Option<usize> {
        self.elements.id_2_local_index(id)
    }

    fn all_elements_ref(&self) -> Vec<&BPMNElement> {
        self.elements.all_elements_ref()
    }

    fn parent_of(&self, global_index: GlobalIndex) -> (Option<&dyn Processable>, bool) {
        if self.global_index == global_index {
            (None, true)
        } else {
            let x = self.elements.parent_of(global_index);
            if x.1 && x.0.is_none() {
                (Some(self), true)
            } else if x.1 {
                x
            } else {
                (None, false)
            }
        }
    }

    fn all_sequence_flows_ref(&self) -> Vec<&BPMNSequenceFlow> {
        let mut result: Vec<&BPMNSequenceFlow> = self.sequence_flows.iter().collect();
        result.extend(self.elements.all_sequence_flows_ref());
        result
    }

    fn global_index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement> {
        self.elements.global_index_2_element(index)
    }

    fn global_index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement> {
        self.elements.global_index_2_element_mut(index)
    }

    fn local_index_2_element_mut(&mut self, index: usize) -> Option<&mut BPMNElement> {
        self.elements.local_index_2_element_mut(index)
    }
}

impl BPMNElementTrait for BPMNProcess {
    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        //check children individually
        for element in &self.elements {
            element.verify_structural_correctness(self, bpmn)?
        }

        //verify initiation and termination
        verify_structural_correctness_initiation_mode!(self, bpmn);

        Ok(())
    }

    fn add_incoming_sequence_flow(&mut self, _flow_index: usize) -> anyhow::Result<()> {
        Err(anyhow!("processes cannot have incoming sequence flows"))
    }

    fn add_outgoing_sequence_flow(&mut self, _flow_index: usize) -> anyhow::Result<()> {
        Err(anyhow!("processes cannot have outgoing sequence flows"))
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> anyhow::Result<()> {
        Err(anyhow!("processes cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> anyhow::Result<()> {
        Err(anyhow!("processes cannot have outgoing message flows"))
    }
}

impl BPMNObject for BPMNProcess {
    fn local_index(&self) -> usize {
        self.local_index
    }

    fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    fn id(&self) -> &str {
        &self.id
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
        &EMPTY_FLOWS
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn can_start_process_instance(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        Ok(false)
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        false
    }

    fn outgoing_messages_cannot_be_removed(&self) -> bool {
        false
    }

    fn incoming_messages_are_ignored(&self) -> bool {
        false
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        false
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        false
    }
}

impl Transitionable for BPMNProcess {
    fn number_of_transitions(&self, marking: &BPMNSubMarking) -> usize {
        self.elements.number_of_transitions(marking)
    }

    fn enabled_transitions(
        &self,
        root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        self.elements
            .enabled_transitions(root_marking, sub_marking, parent, bpmn)
    }

    fn execute_transition(
        &self,
        mut transition_index: TransitionIndex,
        root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        _parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        for element in &self.elements {
            let number_of_transitions = element.number_of_transitions(sub_marking);
            if transition_index < number_of_transitions {
                return element.execute_transition(
                    transition_index,
                    root_marking,
                    sub_marking,
                    self,
                    bpmn,
                );
            }
            transition_index -= number_of_transitions;
        }
        Ok(())
    }

    fn transition_activity(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        self.elements.transition_activity(transition_index, marking)
    }

    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        self.elements
            .transition_debug(transition_index, marking, bpmn)
    }

    fn transition_weight(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        _parent: &dyn Processable,
    ) -> Option<Fraction> {
        self.elements
            .transition_weight(transition_index, marking, self)
    }
}

impl Startable for BPMNProcess {
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

impl Processable for BPMNProcess {
    fn elements_non_recursive(&self) -> &Vec<BPMNElement> {
        &self.elements
    }

    fn sequence_flows_non_recursive(&self) -> &Vec<BPMNSequenceFlow> {
        &self.sequence_flows
    }

    fn to_sub_marking(&self, initiation_mode: &InitiationMode) -> Result<BPMNSubMarking> {
        let result: Result<BPMNSubMarking> = to_sub_marking!(self, initiation_mode);
        let mut result = result?;
        result.initial_choice_token = false; //handled by root; should not be accessed; change to false for clarity
        Ok(result)
    }

    fn is_sub_process(&self) -> bool {
        false
    }
}
