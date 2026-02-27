use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    parser::parser_state::GlobalIndex,
    semantics::{BPMNSubMarking, TransitionIndex},
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

#[derive(Clone, Debug)]
pub struct BPMNCollapsedPool {
    pub global_index: GlobalIndex,
    pub id: String,
    pub local_index: usize,
    pub name: Option<String>,
    pub incoming_message_flows: Vec<usize>,
    pub outgoing_message_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNCollapsedPool {
    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        _bpmn: &crate::BusinessProcessModelAndNotation,
    ) -> Result<()> {
        Ok(())
    }

    fn add_incoming_sequence_flow(&mut self, _flow_index: usize) -> Result<()> {
        return Err(anyhow!("cannot add sequence flow to collapsed pool"));
    }

    fn add_outgoing_sequence_flow(&mut self, _flow_index: usize) -> Result<()> {
        return Err(anyhow!("cannot add sequence flow from collapsed pool"));
    }

    fn add_incoming_message_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_message_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_message_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_message_flows.push(flow_index);
        Ok(())
    }
}

impl BPMNObject for BPMNCollapsedPool {
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
        &EMPTY_FLOWS
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &self.incoming_message_flows
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &self.outgoing_message_flows
    }

    fn can_start_process_instance(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        Ok(self.incoming_sequence_flows().len() == 0)
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        true
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        false
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        false
    }
}

impl Transitionable for BPMNCollapsedPool {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        0
    }

    fn enabled_transitions(
        &self,
        _marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        Ok(bitvec![0;0])
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
            "collapsed pool `{}`; internal transition {}",
            self.id, transition_index
        ))
    }
}
