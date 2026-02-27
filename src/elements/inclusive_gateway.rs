use std::collections::VecDeque;

use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    semantics::BPMNMarking,
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};

#[derive(Debug, Clone)]
pub struct BPMNInclusiveGateway {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}
impl BPMNElementTrait for BPMNInclusiveGateway {
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

impl BPMNObject for BPMNInclusiveGateway {
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

impl Transitionable for BPMNInclusiveGateway {
    fn number_of_transitions(&self) -> usize {
        2usize.pow(self.outgoing_sequence_flows.len() as u32) - 1
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        if self.incoming_sequence_flows.len() == 0 {
            //if there are no sequence flows, then initiation mode 2 applies.
            //that is, look in the extra virtual sequence flow
            if marking.element_index_2_tokens[self.index] >= 1 {
                //enabled
                return Ok(bitvec![1;self.number_of_transitions()]);
            } else {
                //not enabled
                return Ok(bitvec![0;self.number_of_transitions()]);
            }
        } else {
            //gather a list of incoming sequence flows that do not have a token
            let mut empty_sequence_flows = bitvec![0;bpmn.number_of_sequence_flows()];
            for sequence_flow in &self.incoming_sequence_flows {
                if marking.sequence_flow_2_tokens[*sequence_flow] == 0 {
                    empty_sequence_flows.set(*sequence_flow, true);
                }
            }

            if empty_sequence_flows.count_ones() == self.incoming_sequence_flows.len() {
                //not enabled as there is no token
                return Ok(bitvec![0;self.number_of_transitions()]);
            }

            //perform a backwards search to find tokens that may still come to the gateway
            let mut queue = VecDeque::new();
            queue.extend(empty_sequence_flows.iter_ones());
            let mut seen_sequence_flows = empty_sequence_flows;
            while let Some(sequence_flow) = queue.pop_front() {
                if marking.sequence_flow_2_tokens[sequence_flow] >= 1 {
                    //we encountered a token on our search, so the gateway is not enabled
                    return Ok(bitvec![0;self.number_of_transitions()]);
                }

                let source_index = bpmn.sequence_flows[sequence_flow].source_index;
                let source = bpmn
                    .index_2_element(source_index)
                    .ok_or_else(|| anyhow!("source not found"))?;
                for next_sequence_flow in source.incoming_sequence_flows() {
                    if !seen_sequence_flows[*next_sequence_flow] {
                        queue.push_back(*next_sequence_flow);
                        seen_sequence_flows.set(*next_sequence_flow, false);
                    }
                }
            }

            //enabled
            return Ok(bitvec![1;self.number_of_transitions()]);
        }
    }
}
