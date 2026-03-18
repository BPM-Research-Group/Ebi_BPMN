use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    marking::{BPMNRootMarking, BPMNSubMarking, Token},
    semantics::TransitionIndex,
    traits::processable::Processable,
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, prelude::Lsb0, vec::BitVec};
use ebi_activity_key::Activity;
use ebi_arithmetic::Fraction;

/// A trait that provides semantics to BPMN elements, by means of transitions.
/// An element can involve any number of transitions, each of which has a deterministic effect on the marking.
pub trait Transitionable {
    /// the number of transitions that this element needs (recursive)
    fn number_of_transitions(&self, marking: &BPMNSubMarking) -> usize;

    /// Returns a BitVec with the transitions that are currently enabled.
    fn enabled_transitions(
        &self,
        root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec>;

    /// Execute the transition
    fn execute_transition(
        &self,
        transition_index: TransitionIndex,
        root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()>;

    /// If the transition exists and is labelled, returns the label. Otherwise, returns None.
    fn transition_activity(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
    ) -> Option<Activity>;

    /// If the transition exists, returns debug information. Otherwise, returns None.
    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String>;

    /// If the transition exists, return the weight of the transition, that is, the probabilistic cost incurrend when calling it.
    /// Otherwise, return None.
    fn transition_probabilistic_penalty(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        parent: &dyn Processable,
    ) -> Option<Fraction>;

    /// Returns the tokens that are consumed when this transition is fired, or None if the transition does not exist.
    fn transition_2_consumed_tokens(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<Vec<Token>>;

    /// Returns the tokens that are produced when this transition is fired, or None if the transition does not exist.
    fn transition_2_produced_tokens(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<Vec<Token>>;
}

impl Transitionable for Vec<BPMNElement> {
    fn number_of_transitions(&self, marking: &BPMNSubMarking) -> usize {
        self.iter().map(|x| x.number_of_transitions(marking)).sum()
    }

    fn enabled_transitions(
        &self,
        root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        let mut result = bitvec![];
        for element in self {
            result.extend(element.enabled_transitions(root_marking, sub_marking, parent, bpmn)?)
        }
        Ok(result)
    }

    fn execute_transition(
        &self,
        mut transition_index: TransitionIndex,
        root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        for element in self.iter() {
            let number_of_transitions = element.number_of_transitions(sub_marking);
            if transition_index < number_of_transitions {
                return element.execute_transition(
                    transition_index,
                    root_marking,
                    sub_marking,
                    parent,
                    bpmn,
                );
            }
            transition_index -= number_of_transitions;
        }
        Err(anyhow!("transition not found"))
    }

    fn transition_activity(
        &self,
        mut transition_index: TransitionIndex,
        sub_marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        for element in self.iter() {
            let number_of_transitions = element.number_of_transitions(sub_marking);
            if transition_index < number_of_transitions {
                return element.transition_activity(transition_index, sub_marking);
            }
            transition_index -= number_of_transitions;
        }
        None
    }

    fn transition_debug(
        &self,
        mut transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        for element in self.iter() {
            let number_of_transitions = element.number_of_transitions(marking);
            if transition_index < number_of_transitions {
                return element.transition_debug(transition_index, marking, bpmn);
            }
            transition_index -= number_of_transitions;
        }
        None
    }

    fn transition_probabilistic_penalty(
        &self,
        mut transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        parent: &dyn Processable,
    ) -> Option<Fraction> {
        for element in self.iter() {
            let number_of_transitions = element.number_of_transitions(marking);
            if transition_index < number_of_transitions {
                return element.transition_probabilistic_penalty(transition_index, marking, parent);
            }
            transition_index -= number_of_transitions;
        }
        None
    }

    fn transition_2_consumed_tokens<'a>(
        &'a self,
        mut transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        parent: &'a dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<Vec<Token>> {
        for element in self.iter() {
            let number_of_transitions = element.number_of_transitions(marking);
            if transition_index < number_of_transitions {
                return element.transition_2_consumed_tokens(
                    transition_index,
                    marking,
                    parent,
                    bpmn,
                );
            }
            transition_index -= number_of_transitions;
        }
        None
    }

    fn transition_2_produced_tokens<'a>(
        &'a self,
        mut transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        parent: &'a dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<Vec<Token>> {
        for element in self.iter() {
            let number_of_transitions = element.number_of_transitions(marking);
            if transition_index < number_of_transitions {
                return element.transition_2_consumed_tokens(
                    transition_index,
                    marking,
                    parent,
                    bpmn,
                );
            }
            transition_index -= number_of_transitions;
        }
        None
    }
}

macro_rules! execute_transition_parallel_split {
    ($self:ident, $sub_marking:ident) => {
        for outgoing_sequence_flow in &$self.outgoing_sequence_flows {
            $sub_marking.sequence_flow_2_tokens[*outgoing_sequence_flow] += 1;
        }
    };
}
pub(crate) use execute_transition_parallel_split;

macro_rules! number_of_transitions_xor_join_only {
    ($s:ident) => {
        $s.incoming_sequence_flows.len().max(1)
    };
}
pub(crate) use number_of_transitions_xor_join_only;

macro_rules! enabledness_xor_join_only {
    ($self:ident, $sub_marking:ident) => {
        {
            let mut result = bitvec![0;$self.incoming_sequence_flows.len().max(1)];
            if $self.incoming_sequence_flows.len() >= 1 {
                //we are in initiation mode 1
                for (transition_index, incoming_sequence_flow) in
                    $self.incoming_sequence_flows.iter().enumerate()
                {
                    if $sub_marking.sequence_flow_2_tokens[*incoming_sequence_flow] >= 1 {
                        result.set(transition_index, true);
                    }
                }
            } else {
                //we are in initiation mode 2
                if $sub_marking.element_index_2_tokens[$self.local_index] >= 1 {
                    //enabled
                    result.set(0, true);
                }
            }
            result
        }
    };
}
pub(crate) use enabledness_xor_join_only;

macro_rules! execute_transition_xor_join_consume {
    ($self: ident, $sub_marking:ident, $transition_index:expr) => {
        if $self.incoming_sequence_flows.len() >= 1 {
            //we are in initiation mode 1
            $sub_marking.sequence_flow_2_tokens
                [$self.incoming_sequence_flows[$transition_index]] -= 1;
        } else {
            //we are in initiation mode 2
            $sub_marking.element_index_2_tokens[$self.local_index] -= 1;
        }
    };
}
pub(crate) use execute_transition_xor_join_consume;

macro_rules! execute_transition_message_produce {
    ($self:ident, $root_marking:ident, $bpmn:ident) => {
        if let Some(message_flow_index) = $self.outgoing_message_flow {
            {
                let target = $bpmn.message_flow_index_2_target(message_flow_index)?;
                if !target.incoming_messages_are_ignored() {
                    $root_marking.message_flow_2_tokens[message_flow_index] += 1;
                }
            }
        }
    };
}
pub(crate) use execute_transition_message_produce;

macro_rules! transition_2_consumed_tokens_xor_join {
    ($self:ident, $transition_index:expr, $parent:ident) => {
        if $self.incoming_sequence_flows.len() >= 1 {
            //we are in initiation mode 1
            let sequence_flow_index = $self.incoming_sequence_flows[$transition_index];
            vec![Token::SequenceFlow(
                $parent.sequence_flows_non_recursive()[sequence_flow_index].global_index,
            )]
        } else {
            //we are in initiation mode 2
            vec![Token::Element($self.global_index)]
        }
    };
}
pub(crate) use transition_2_consumed_tokens_xor_join;

macro_rules! transition_2_produced_tokens_concurrent_split {
    ($self:ident, $parent:ident) => {
        $self
            .outgoing_sequence_flows()
            .iter()
            .filter_map(|sequence_flow_id| {
                Some(Token::SequenceFlow(
                    $parent
                        .sequence_flows_non_recursive()
                        .get(*sequence_flow_id)?
                        .global_index,
                ))
            })
            .collect::<Vec<_>>()
    };
}
pub(crate) use transition_2_produced_tokens_concurrent_split;

macro_rules! transition_2_consumed_tokens_message {
    ($self:ident, $bpmn:ident) => {
        //check whether a message is present
        if let Some(message_flow_index) = $self.incoming_message_flow {
            //there is a connected message flow
            let source = $bpmn.message_flow_index_2_source(message_flow_index).ok()?;
            if !source.outgoing_message_flows_always_have_tokens() {
                //this message must actually be there
                if !source.outgoing_messages_cannot_be_removed() {
                    vec![Token::MessageFlow(
                        $bpmn.message_flows[message_flow_index].global_index,
                    )]
                } else {
                    vec![]
                }
            } else {
                //if the message flow has always tokens, we do not need to check the marking
                vec![]
            }
        } else {
            //if there is no incoming message flow, there is no restriction
            vec![]
        }
    };
}
pub(crate) use transition_2_consumed_tokens_message;

macro_rules! transition_2_produced_tokens_message {
    ($self:ident, $bpmn:ident) => {
        if let Some(message_flow_index) = $self.outgoing_message_flow {
            {
                let target = $bpmn.message_flow_index_2_target(message_flow_index).ok()?;
                if !target.incoming_messages_are_ignored() {
                    vec![Token::MessageFlow(
                        $bpmn.message_flows[message_flow_index].global_index,
                    )]
                } else {
                    vec![]
                }
            }
        } else {
            vec![]
        }
    };
}
pub(crate) use transition_2_produced_tokens_message;