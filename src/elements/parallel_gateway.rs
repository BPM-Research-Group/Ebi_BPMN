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
use bitvec::{bitvec, prelude::BitVec};
use ebi_activity_key::Activity;

#[derive(Debug, Clone)]
pub struct BPMNParallelGateway {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNParallelGateway {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("gateways cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("gateways cannot have outgoing message flows"))
    }

    fn verify_structural_correctness(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNParallelGateway {
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

impl Transitionable for BPMNParallelGateway {
    fn number_of_transitions(&self) -> usize {
        1
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        _parent_index: Option<usize>,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        if self.incoming_sequence_flows.is_empty() {
            //if there are no sequence flows, then initiation mode 2 applies.
            //that is, look in the extra virtual sequence flow
            if marking.element_index_2_tokens[self.index] == 0 {
                //disabled
                return Ok(bitvec![0;1]);
            }
        } else {
            //otherwise, every incoming sequence flow must have a token
            for incoming_sequence_flow in &self.incoming_sequence_flows {
                if marking.sequence_flow_2_tokens[*incoming_sequence_flow] == 0 {
                    return Ok(bitvec![0;1]);
                }
            }
        }
        Ok(bitvec![1;1])
    }

    fn transition_activity(&self, _transition_index: TransitionIndex) -> Option<Activity> {
        None
    }

    fn transition_debug(&self, transition_index: TransitionIndex) -> Option<String> {
        Some(format!(
            "parallel gateway `{}`; internal transition {}",
            self.id,
            transition_index
        ))
    }
}
