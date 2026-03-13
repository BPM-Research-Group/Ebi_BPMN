use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    elements::start_event::{enabled_transitions_start_event, execute_transition_start_event},
    parser::parser_state::GlobalIndex,
    semantics::{BPMNRootMarking, BPMNSubMarking, TransitionIndex},
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        transitionable::{
            Transitionable, execute_transition_parallel_split,
            transition_2_marked_sequence_flows_concurrent_split,
        },
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;
use ebi_arithmetic::{Fraction, One};

#[derive(Debug, Clone)]
pub struct BPMNTimerStartEvent {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) timer_marker_id: String,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNTimerStartEvent {
    fn add_incoming_sequence_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "timer start events cannot have incoming sequence flows"
        ))
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "timer start events cannot have incoming message flows"
        ))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "timer start events cannot have outgoing message flows"
        ))
    }

    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNTimerStartEvent {
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
        Ok(true)
    }

    fn is_end_event(&self) -> bool {
        false
    }

    fn incoming_sequence_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
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
        Ok(true)
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
        true
    }
}

impl Transitionable for BPMNTimerStartEvent {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        1
    }

    fn enabled_transitions(
        &self,
        root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        Ok(enabled_transitions_start_event!(
            self,
            root_marking,
            sub_marking,
            parent
        ))
    }

    fn execute_transition(
        &self,
        _transition_index: TransitionIndex,
        root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        execute_transition_start_event!(self, root_marking, sub_marking, parent);
        Ok(())
    }

    fn transition_activity(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        None
    }

    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        Some(format!(
            "timer start event `{}`; internal transition {}",
            self.id, transition_index
        ))
    }

    fn transition_weight(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        _parent: &dyn Processable,
    ) -> Option<Fraction> {
        Some(Fraction::one())
    }

    fn transition_2_marked_sequence_flows<'a>(
        &'a self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        parent: &'a dyn Processable,
    ) -> Option<Vec<GlobalIndex>> {
        transition_2_marked_sequence_flows_concurrent_split!(self, parent)
    }
}
