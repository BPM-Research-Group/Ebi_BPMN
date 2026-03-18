use crate::{
    BusinessProcessModelAndNotation, element::BPMNElementTrait, elements::task::task_consumed_tokens, marking::{BPMNRootMarking, BPMNSubMarking, Token}, parser::parser_state::GlobalIndex, semantics::TransitionIndex, traits::{
        objectable::BPMNObject,
        processable::Processable,
        transitionable::{
            Transitionable, enabledness_xor_join_only, execute_transition_message_produce,
            execute_transition_parallel_split, execute_transition_xor_join_consume,
            number_of_transitions_xor_join_only, transition_2_consumed_tokens_message,
            transition_2_consumed_tokens_xor_join, transition_2_produced_tokens_concurrent_split,
            transition_2_produced_tokens_message,
        },
    }
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;
use ebi_arithmetic::{Fraction, One};

#[derive(Debug, Clone)]
pub struct BPMNManualTask {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub activity: Activity,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
    pub(crate) incoming_message_flow: Option<usize>,
    pub(crate) outgoing_message_flow: Option<usize>,
}

impl BPMNElementTrait for BPMNManualTask {
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

impl BPMNObject for BPMNManualTask {
    fn local_index(&self) -> usize {
        self.local_index
    }

    fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    fn activity(&self) -> Option<Activity> {
        Some(self.activity)
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
        if let Some(message_flow_index) = self.incoming_message_flow {
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
        true
    }
}

impl Transitionable for BPMNManualTask {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        number_of_transitions_xor_join_only!(self)
    }

    fn enabled_transitions(
        &self,
        root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        //check whether a message is present
        if let Some(message_flow_index) = self.incoming_message_flow {
            //there is a connected message flow
            let source = bpmn.message_flow_index_2_source(message_flow_index)?;
            if !source.outgoing_message_flows_always_have_tokens() {
                //this message must actually be there

                if root_marking.message_flow_2_tokens[message_flow_index] == 0 {
                    //message is not present; all transitions are not enabled
                    return Ok(bitvec![0;self.number_of_transitions(sub_marking)]);
                }
            } else {
                //if the message flow has always tokens, we do not need to check the marking
            }
        } else {
            //if there is no incoming message flow, there is no restriction
        }

        Ok(enabledness_xor_join_only!(self, sub_marking))
    }

    fn execute_transition(
        &self,
        transition_index: TransitionIndex,
        root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        //consume token
        if let Some(sequence_flow_index) = self.incoming_sequence_flows.iter().next() {
            let sequence_flow = &parent.sequence_flows_non_recursive()[*sequence_flow_index];
            let source = &parent.elements_non_recursive()[sequence_flow.source_local_index];
            if source.is_event_based_gateway() {
                //special case: source is an event-based gateway

                //remove a token from all outgoing sequence flows of the event-based gateway
                for outgoing_sequence_flow in source.outgoing_sequence_flows() {
                    sub_marking.sequence_flow_2_tokens[*outgoing_sequence_flow] -= 1;
                }
            } else {
                //not a special case
                execute_transition_xor_join_consume!(self, sub_marking, transition_index);
            }
        } else {
            //not a special case
            execute_transition_xor_join_consume!(self, sub_marking, transition_index);
        }

        //consume message
        {
            //check whether a message is present
            if let Some(message_flow_index) = self.incoming_message_flow {
                //there is a connected message flow
                let source = bpmn.message_flow_index_2_source(message_flow_index)?;
                if !source.outgoing_message_flows_always_have_tokens() {
                    //this message must actually be there
                    if !source.outgoing_messages_cannot_be_removed() {
                        root_marking.message_flow_2_tokens[message_flow_index] -= 1;
                    }
                } else {
                    //if the message flow has always tokens, we do not need to check the marking
                }
            } else {
                //if there is no incoming message flow, there is no restriction
            }
        }

        //produce
        execute_transition_parallel_split!(self, sub_marking);
        execute_transition_message_produce!(self, root_marking, bpmn);

        Ok(())
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
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        Some(format!(
            "task `{}`; internal transition {}; label `{}`",
            self.id,
            transition_index,
            bpmn.activity_key.deprocess_activity(&self.activity)
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

    fn transition_2_consumed_tokens<'a>(
        &'a self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        parent: &'a dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<Vec<Token>> {
        let mut result = task_consumed_tokens!(self, transition_index, parent);
        result.append(&mut transition_2_consumed_tokens_message!(self, bpmn));
        Some(result)
    }

    fn transition_2_produced_tokens(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<Vec<Token>> {
        let mut result = transition_2_produced_tokens_concurrent_split!(self, parent);
        result.append(&mut transition_2_produced_tokens_message!(self, bpmn));
        Some(result)
    }
}
