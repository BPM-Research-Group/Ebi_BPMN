use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    semantics::{BPMNSubMarking, TransitionIndex},
    traits::processable::Processable,
};
use anyhow::Result;
use bitvec::{bitvec, prelude::Lsb0, vec::BitVec};
use ebi_activity_key::Activity;

/// A trait that provides semantics to BPMN elements, by means of transitions.
/// An element can involve any number of transitions, each of which has a deterministic effect on the marking.
pub trait Transitionable {
    /// the number of transitions that this element needs (recursive)
    fn number_of_transitions(&self, marking: &BPMNSubMarking) -> usize;

    /// Returns a BitVec with the transitions that are currently enabled.
    fn enabled_transitions(
        &self,
        marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec>;

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
    ) -> Option<String>;
}

impl Transitionable for Vec<BPMNElement> {
    fn number_of_transitions(&self, marking: &BPMNSubMarking) -> usize {
        self.iter().map(|x| x.number_of_transitions(marking)).sum()
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        let mut result = bitvec![];
        for element in self {
            result.extend(element.enabled_transitions(marking, parent, bpmn)?)
        }
        Ok(result)
    }

    fn transition_activity(
        &self,
        mut transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        for element in self.iter() {
            let number_of_transitions = element.number_of_transitions(marking);
            if transition_index < number_of_transitions {
                return element.transition_activity(transition_index, marking);
            }
            transition_index -= number_of_transitions;
        }
        None
    }

    fn transition_debug(
        &self,
        mut transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
    ) -> Option<String> {
        for element in self.iter() {
            let number_of_transitions = element.number_of_transitions(marking);
            if transition_index < number_of_transitions {
                return element.transition_debug(transition_index, marking);
            }
            transition_index -= number_of_transitions;
        }
        None
    }
}

#[macro_export]
macro_rules! number_of_transitions_xor_join_only {
    ($s:ident) => {
        $s.incoming_sequence_flows.len().max(1)
    };
}

#[macro_export]
macro_rules! enabledness_xor_join_only {
    ($s:ident, $marking:ident) => {
        {
            let mut result = bitvec![0;$s.incoming_sequence_flows.len().max(1)];
            if $s.incoming_sequence_flows.len() >= 1 {
                //we are in initiation mode 1
                for (transition_index, incoming_sequence_flow) in
                    $s.incoming_sequence_flows.iter().enumerate()
                {
                    if $marking.sequence_flow_2_tokens[*incoming_sequence_flow] >= 1 {
                        result.set(transition_index, true);
                    }
                }
            } else {
                //we are in initiation mode 2
                if $marking.element_index_2_tokens[$s.local_index] >= 1 {
                    //enabled
                    result.set(0, true);
                }
            }
            result
        }
    };
}
