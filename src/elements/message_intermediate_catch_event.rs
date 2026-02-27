use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    enabledness_xor_join_only, number_of_transitions_xor_join_only,
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
pub struct BPMNMessageIntermediateCatchEvent {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) message_marker_id: String,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
    pub(crate) incoming_message_flow: Option<usize>,
}

impl BPMNElementTrait for BPMNMessageIntermediateCatchEvent {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
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
            "message intermediate catch events cannot have outgoing message flows"
        ))
    }

    fn verify_structural_correctness(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNMessageIntermediateCatchEvent {
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
        self.incoming_message_flow.as_slice()
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn can_start_process_instance(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
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

impl Transitionable for BPMNMessageIntermediateCatchEvent {
    fn number_of_transitions(&self) -> usize {
        number_of_transitions_xor_join_only!(self)
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        _parent_index: Option<usize>,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        //check whether the message is present
        if let Some(message_flow_index) = self.incoming_message_flow {
            //there is a connected message flow
            let source = bpmn.message_flow_index_2_source(message_flow_index)?;
            if !source.outgoing_message_flows_always_have_tokens() {
                //this message must actually be there

                if marking.message_flow_2_tokens[message_flow_index] == 0 {
                    //message is not present; all transitions are not enabled
                    return Ok(bitvec![0; self.incoming_sequence_flows.len()]);
                }
            } else {
                //if the message flow has always tokens, we do not need to check them
            }
        } else {
            //if there is no incoming message flow, we assume there is always a message
        }

        //see which transitions are enabled
        Ok(enabledness_xor_join_only!(self, marking))
    }

    fn transition_activity(&self, _transition_index: TransitionIndex) -> Option<Activity> {
        None
    }

    fn transition_debug(&self, transition_index: TransitionIndex) -> Option<String> {
        Some(format!(
            "message intermediate catch event `{}`; internal transition {}",
            self.id, transition_index
        ))
    }
}
