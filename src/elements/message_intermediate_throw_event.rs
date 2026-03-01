use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    enabledness_xor_join_only, number_of_transitions_xor_join_only,
    parser::parser_state::GlobalIndex,
    semantics::{BPMNRootMarking, BPMNSubMarking, TransitionIndex},
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

#[derive(Debug, Clone)]
pub struct BPMNMessageIntermediateThrowEvent {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) message_marker_id: String,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
    pub(crate) outgoing_message_flow: Option<usize>,
}

impl BPMNElementTrait for BPMNMessageIntermediateThrowEvent {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> anyhow::Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> anyhow::Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("none events cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, flow_index: usize) -> Result<()> {
        if self.outgoing_message_flow.is_some() {
            return Err(anyhow!("cannot add a second outgoing message flow"));
        }
        self.outgoing_message_flow = Some(flow_index);
        Ok(())
    }

    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNMessageIntermediateThrowEvent {
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
        &self.incoming_sequence_flows
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &self.outgoing_sequence_flows
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        self.outgoing_message_flow.as_slice()
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

impl Transitionable for BPMNMessageIntermediateThrowEvent {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        number_of_transitions_xor_join_only!(self)
    }

    fn enabled_transitions(
        &self,
        _root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        Ok(enabledness_xor_join_only!(self, sub_marking))
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
    ) -> Option<String> {
        Some(format!(
            "message intermediate throw event `{}`; internal transition {}",
            self.id, transition_index
        ))
    }
}
