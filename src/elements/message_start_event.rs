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
pub struct BPMNMessageStartEvent {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) message_marker_id: String,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
    pub(crate) incoming_message_flow: Option<usize>,
}

impl BPMNElementTrait for BPMNMessageStartEvent {
    fn add_incoming_sequence_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "message start events cannot have incoming sequence flows"
        ))
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, flow_index: usize) -> Result<()> {
        if self.incoming_message_flow.is_some() {
            return Err(anyhow!("cannot add a second incoming message flow"));
        }
        self.incoming_message_flow = Some(flow_index);
        Ok(())
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "message start events cannot have outgoing message flows"
        ))
    }

    fn verify_structural_correctness(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        if self.incoming_message_flow.is_none() {
            return Err(anyhow!(
                "a message start event must have an incoming message flow"
            ));
        }
        Ok(())
    }
}

impl BPMNObject for BPMNMessageStartEvent {
    fn index(&self) -> usize {
        self.index
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn is_unconstrained_start_event(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        if let Some(message_flow_index) = self.incoming_message_flow {
            let source = bpmn.message_flow_index_2_source(message_flow_index)?;
            if source.outgoing_message_flows_always_have_tokens() {
                //a message from a collapsed pool is always there
                Ok(true)
            } else {
                //otherwise, the message must be there = the instance has already started
                Ok(false)
            }
        } else {
            //there is no constraining message, so this message start event can start a process instance
            Ok(true)
        }
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
        &self.incoming_message_flow.as_slice()
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn can_start_process_instance(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        self.is_unconstrained_start_event(bpmn)
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

impl Transitionable for BPMNMessageStartEvent {
    fn number_of_transitions(&self) -> usize {
        1
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        if let Some(message_flow_index) = self.incoming_message_flow {
            //Two cases apply:
            let source = bpmn.message_flow_index_2_source(message_flow_index)?;
            if source.outgoing_message_flows_always_have_tokens() {
                //1) the source of the message always has tokens
                //we are enabled when specifically enabled by the environment
                if marking.element_index_2_tokens[self.index] >= 1 {
                    //enabled
                    Ok(bitvec![1;1])
                } else {
                    //disabled
                    Ok(bitvec![0;1])
                }
            } else {
                //2) the source of the message is normal -> normal enablement
                // we are enabled if there is a message on the incoming message flow
                if marking.message_flow_2_tokens[message_flow_index] >= 1 {
                    //enabled
                    Ok(bitvec![1;1])
                } else {
                    //not enabled
                    Ok(bitvec![0;1])
                }
            }
        } else {
            //model does not have an incoming message flow; treat as a regular start event
            if marking.pre_initial_choice_token {
                //enabled by initial choice token
                Ok(bitvec![1;1])
            } else if marking.element_index_2_tokens[self.index] >= 1 {
                //enabled by element token
                Ok(bitvec![1;1])
            } else {
                //not enabled
                Ok(bitvec![0;1])
            }
        }
    }
}
