use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    enabledness_xor_join_only, number_of_transitions_xor_join_only,
    parser::parser_state::GlobalIndex,
    semantics::{BPMNSubMarking, TransitionIndex},
    traits::{objectable::BPMNObject, processable::Processable, transitionable::Transitionable},
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

#[derive(Debug, Clone)]
pub struct BPMNTask {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub activity: Activity,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
    pub(crate) incoming_message_flow: Option<usize>,
    pub(crate) outgoing_message_flow: Option<usize>,
}

impl BPMNElementTrait for BPMNTask {
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

impl BPMNObject for BPMNTask {
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
        &self.incoming_message_flow.as_slice()
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &self.outgoing_message_flow.as_slice()
    }

    fn can_start_process_instance(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        if self.outgoing_sequence_flows().len() == 0 {
            Ok(false)
        } else if let Some(message_flow_index) = self.incoming_message_flow {
            let source = bpmn.message_flow_index_2_source(message_flow_index)?;
            if source.is_collapsed_pool() {
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

impl Transitionable for BPMNTask {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        number_of_transitions_xor_join_only!(self)
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        //check whether a message is present
        if let Some(message_flow_index) = self.incoming_message_flow {
            //there is a connected message flow
            let source = bpmn.message_flow_index_2_source(message_flow_index)?;

            if !source.outgoing_message_flows_always_have_tokens() {
                //this message must actually be there

                if marking.root_marking.message_flow_2_tokens[message_flow_index] == 0 {
                    //message is not present; all transitions are not enabled
                    return Ok(bitvec![0;self.number_of_transitions(marking)]);
                }
            } else {
                //if the message flow has always tokens, we do not need to check the marking
            }
        } else {
            //if there is no incoming message flow, there is no restriction
        }

        Ok(enabledness_xor_join_only!(self, marking))
    }

    fn transition_activity(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        Some(self.activity)
    }

    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
    ) -> Option<String> {
        Some(format!(
            "task `{}`; internal transition {}",
            self.id, transition_index
        ))
    }
}
