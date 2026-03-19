use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    marking::{BPMNRootMarking, BPMNSubMarking, Token},
    parser::parser_state::GlobalIndex,
    semantics::TransitionIndex,
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        transitionable::{
            Transitionable, enabledness_xor_join_only, execute_transition_xor_join_consume,
            number_of_transitions_xor_join_only, transition_2_consumed_tokens_xor_join,
        },
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;
use ebi_arithmetic::{Fraction, One};

#[derive(Debug, Clone)]
pub struct BPMNEndEvent {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) incoming_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNEndEvent {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("end events cannot have outgoing sequence flows"))
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("none events cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("none events cannot have outgoing message flows"))
    }

    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNEndEvent {
    fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    fn local_index(&self) -> usize {
        self.local_index
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn activity(&self) -> Option<Activity> {
        None
    }

    fn is_unconstrained_start_event(
        &self,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<bool> {
        Ok(false)
    }

    fn is_end_event(&self) -> bool {
        true
    }

    fn incoming_sequence_flows(&self) -> &[usize] {
        &self.incoming_sequence_flows
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
        Ok(self.incoming_sequence_flows().len() == 0)
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
        true
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        false
    }
}

impl Transitionable for BPMNEndEvent {
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

    fn execute_transition(
        &self,
        transition_index: TransitionIndex,
        _root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        execute_transition_xor_join_consume!(self, sub_marking, transition_index);
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
            "end event `{}`; internal transition {}",
            self.id, transition_index
        ))
    }

    fn transition_probabilistic_penalty(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        _parent: &dyn Processable,
    ) -> Option<Fraction> {
        Some(Fraction::one())
    }

    fn transition_2_consumed_tokens(
        &self,
        transition_index: TransitionIndex,
        _root_marking: &BPMNRootMarking,
        _sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<crate::marking::Token>> {
        Ok(transition_2_consumed_tokens_xor_join!(
            self,
            transition_index,
            parent
        ))
    }

    fn transition_2_produced_tokens(
        &self,
        _transition_index: TransitionIndex,
        _root_marking: &BPMNRootMarking,
        _sub_marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<crate::marking::Token>> {
        Ok(vec![])
    }
}
