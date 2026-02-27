use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    semantics::{BPMNMarking, TransitionIndex},
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

#[derive(Debug, Clone)]
pub struct BPMNStartEvent {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNStartEvent {
    fn add_incoming_sequence_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("start events cannot have incoming sequence flows"))
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("start events cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("start events cannot have outgoing message flows"))
    }

    fn verify_structural_correctness(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNStartEvent {
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

    fn can_have_incoming_sequence_flows(&self) -> bool {
        false
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        true
    }
}

#[macro_export]
macro_rules! enabled_transitions_start_event {
    ($self:ident, $marking:ident,$parent_index:ident) => {
        if $parent_index.is_none() && $marking.root_initial_choice_token {
            //enabled by root initial choice token
            bitvec![1;1]
        } else if let Some(parent_index) = $parent_index
            && *$marking
                .sub_initial_choice_tokens
                .get(&parent_index)
                .unwrap_or(&0)
                >= 1
        {
            //enabled by sub-process initial choice token
            bitvec![1;1]
        } else if $marking.element_index_2_tokens[$self.index] >= 1 {
            //enabled by element token
            bitvec![1;1]
        } else {
            //not enabled
            bitvec![0;1]
        }
    };
}

impl Transitionable for BPMNStartEvent {
    fn number_of_transitions(&self) -> usize {
        1
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        parent_index: Option<usize>,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        Ok(enabled_transitions_start_event!(
            self,
            marking,
            parent_index
        ))
    }

    fn transition_activity(&self, _transition_index: TransitionIndex) -> Option<Activity> {
        None
    }

    fn transition_debug(&self, transition_index: TransitionIndex) -> Option<String> {
        Some(format!(
            "start event `{}`; internal transition {}",
            self.id, transition_index
        ))
    }
}
