use crate::{BusinessProcessModelAndNotation, element::BPMNElement, semantics::BPMNMarking};
use bitvec::{bitvec, prelude::Lsb0, vec::BitVec};

/// A trait that provides semantics to BPMN elements, by means of transitions.
/// An element can involve any number of transitions, each of which has a deterministic effect on the marking.
pub trait Transitionable {
    /// the number of transitions supported
    fn number_of_transitions(&self) -> usize;

    /// Returns a BitVec with the transitions that are currently enabled.
    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> BitVec;
}

impl Transitionable for Vec<BPMNElement> {
    fn number_of_transitions(&self) -> usize {
        self.iter().map(|x| x.number_of_transitions()).sum()
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> BitVec {
        let mut result = bitvec![];
        for element in self {
            result.extend(element.enabled_transitions(marking, bpmn))
        }
        result
    }
}
