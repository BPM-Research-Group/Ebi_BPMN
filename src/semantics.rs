use crate::{
    BusinessProcessModelAndNotation, objects_objectable::BPMNObject,
    objects_transitionable::Transitionable,
};
use anyhow::Result;
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

#[derive(Debug, PartialEq, Eq)]
pub struct BPMNMarking {
    pub(crate) sequence_flow_2_tokens: Vec<u64>,
    pub(crate) message_flow_2_tokens: Vec<u64>,

    /// in case multiple start events are present, a single place is added
    pub(crate) pre_initial_choice_token: bool,

    /// in case no start events are present, every eligible element without incoming sequence flows gets a token
    pub(crate) index_2_tokens: BitVec,
}
type TransitionIndex = usize;

impl BusinessProcessModelAndNotation {
    /// BPMN 2.0.2 standard page 238
    fn get_initial_state(&self) -> Option<BPMNMarking> {
        //find start events
        let applicable_start_events = self
            .all_elements_ref()
            .into_iter()
            .filter(|element| element.is_start_event() || element.is_timer_start_event())
            .collect::<Vec<_>>();

        //determine the initiation mode
        Some(if applicable_start_events.len() >= 1 {
            //initiation mode 1: through one or more start events
            BPMNMarking {
                sequence_flow_2_tokens: vec![0; self.number_of_sequence_flows()],
                message_flow_2_tokens: vec![0; self.number_of_message_flows()],
                pre_initial_choice_token: true,
                index_2_tokens: bitvec![0;0],
            }
        } else {
            //initiation mode 2: eligible elements without incoming sequence flows all get a token
            //add corresponding places to the marking
            todo!()

            //reminder: put message on message flow from collapsed pool to message start event
        })
    }

    fn execute_transition(
        &self,
        state: &mut BPMNMarking,
        transition: TransitionIndex,
    ) -> Result<()> {
        todo!()
    }

    fn is_final_state(&self, state: &BPMNMarking) -> bool {
        self.get_enabled_transitions(state).is_empty()
    }

    fn is_transition_silent(&self, transition: TransitionIndex) -> bool {
        todo!()
    }

    fn get_transition_activity(&self, transition: TransitionIndex) -> Option<Activity> {
        todo!()
    }

    fn get_enabled_transitions(&self, state: &BPMNMarking) -> Vec<TransitionIndex> {
        //recurse to elements
        let result = self.elements.enabled_transitions(state, self);

        //transform to list of indices
        let mut result2 = Vec::new();
        for index in result.iter_ones() {
            result2.push(index);
        }
        result2
    }

    fn get_number_of_transitions(&self) -> usize {
        self.elements.number_of_transitions()
    }
}

#[cfg(test)]
mod tests {
    use crate::{BusinessProcessModelAndNotation, bpmn::semantics::SemState};
    use std::fs::{self};

    #[test]
    fn bpmn_semantics() {
        let fin = fs::read_to_string("testfiles/model.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        assert_eq!(
            bpmn.get_initial_state(),
            Some(SemState {
                marking_sequence_flows: vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                marking_message_flows: vec![]
            })
        );
        assert_eq!(bpmn.get_number_of_transitions(), 12)
    }
}
