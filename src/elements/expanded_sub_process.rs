use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    enabledness_xor_join_only, number_of_transitions_xor_join_only,
    semantics::{BPMNMarking, TransitionIndex},
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        startable::Startable,
        transitionable::Transitionable,
    },
    verify_structural_correctness_initiation_mode,
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

#[derive(Debug, Clone)]
pub struct BPMNExpandedSubProcess {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) elements: Vec<BPMNElement>,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNExpandedSubProcess {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> anyhow::Result<()> {
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

    fn verify_structural_correctness(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        //recurse on elements
        for element in &self.elements {
            element.verify_structural_correctness(bpmn)?
        }

        //verify initiation and termination
        verify_structural_correctness_initiation_mode!(self, bpmn);

        Ok(())
    }
}

impl BPMNObject for BPMNExpandedSubProcess {
    fn index(&self) -> usize {
        self.index
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
    fn number_of_transitions(&self) -> usize {
        //behaves like an XOR-join to start
        number_of_transitions_xor_join_only!(self)
        //one transition to end
        + 1
        //and its inner transitions
        + self.elements.number_of_transitions()
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        _parent_index: Option<usize>,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        //start transitions: like an xor join
        let mut result = enabledness_xor_join_only!(self, marking);

        //gather children transitions
        let children_enabled_transitions =
            self.elements
                .enabled_transitions(marking, Some(self.index), bpmn)?;

        //end transition
        if children_enabled_transitions.not_any() {
            //child has no enabled transitions -> terminated
            result.push(true);
        } else {
            //not enabled
            result.push(false);
        }

        //recurse
        result.extend(children_enabled_transitions);

        Ok(result)
    }

    fn transition_activity(&self, mut transition_index: TransitionIndex) -> Option<Activity> {
        //start transition
        if transition_index < self.incoming_sequence_flows.len().max(1) {
            return None;
        }
        transition_index -= self.incoming_sequence_flows.len().max(1);

        if transition_index == 0 {
            //end transition
            return None;
        }
        transition_index -= 1;

        //recurse on children
        self.elements.transition_activity(transition_index)
    }

    fn transition_debug(&self, mut transition_index: TransitionIndex) -> Option<String> {
        //start transition
        if transition_index < self.incoming_sequence_flows.len().max(1) {
            return Some(format!(
                "expanded sub-process `{}`; start internal transition {}",
                self.id, transition_index
            ));
        }
        transition_index -= self.incoming_sequence_flows.len().max(1);

        if transition_index == 0 {
            //end transition
            return Some(format!(
                "expanded sub-process `{}`; end transition",
                self.id
            ));
        }
        transition_index -= 1;

        //recurse on children
        self.elements.transition_debug(transition_index)
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
