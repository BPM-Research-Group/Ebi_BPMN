use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    objects_objectable::{BPMNObject, EMPTY_FLOWS},
    objects_transitionable::Transitionable,
    semantics::BPMNMarking,
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};

#[derive(Debug, Clone)]
pub struct BPMNIntermediateCatchEvent {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNIntermediateCatchEvent {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("none events cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("none events cannot have outgoing message flows"))
    }

    fn verify_structural_correctness(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNIntermediateCatchEvent {
    fn index(&self) -> usize {
        self.index
    }

    fn id(&self) -> &str {
        &self.id
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

    fn can_have_incoming_sequence_flows(&self) -> bool {
        true
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        false
    }
}

impl Transitionable for BPMNIntermediateCatchEvent {
    fn number_of_transitions(&self) -> usize {
        self.incoming_sequence_flows.len()
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> BitVec {
        let mut result = bitvec![0;self.incoming_sequence_flows.len()];

        for (transition_index, incoming_sequence_flow) in
            self.incoming_sequence_flows.iter().enumerate()
        {
            if marking.sequence_flow_2_tokens[*incoming_sequence_flow] >= 1 {
                result.set(transition_index, true);
            }
        }

        result
    }
}
