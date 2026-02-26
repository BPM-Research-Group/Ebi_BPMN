use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    elements::process::BPMNProcess,
    objects_objectable::BPMNObject,
    objects_startable::{InitiationMode, Startable},
    objects_transitionable::Transitionable,
};
use anyhow::Result;
use ebi_activity_key::Activity;

#[derive(Debug, PartialEq, Eq)]
pub struct BPMNMarking {
    pub(crate) sequence_flow_2_tokens: Vec<u64>,
    pub(crate) message_flow_2_tokens: Vec<u64>,

    /// in case multiple start events are present, a single place is added
    pub(crate) pre_initial_choice_token: bool,

    /// in case no start events are present, every eligible element without incoming sequence flows gets a token
    pub(crate) element_index_2_tokens: Vec<u64>,
}
type TransitionIndex = usize;

impl BusinessProcessModelAndNotation {
    /// BPMN 2.0.2 standard page 238
    pub fn get_initial_state(&self) -> Result<BPMNMarking> {
        //gather the initiation mode
        let mut mode = InitiationMode::ParallelElements(vec![]);
        for element in &self.elements {
            if let BPMNElement::Process(process) = element {
                mode = mode + process.initiation_mode(self)?;
            }
        }

        //determine the initiation mode
        if let InitiationMode::ParallelElements(elements) = mode {
            //initiation mode 2: eligible elements without incoming sequence flows all get a token

            let mut element_index_2_tokens = vec![0; self.number_of_elements()];
            for element in elements {
                element_index_2_tokens[element.index()] = 1;
            }

            Ok(BPMNMarking {
                sequence_flow_2_tokens: vec![0; self.number_of_sequence_flows()],
                message_flow_2_tokens: vec![0; self.number_of_message_flows()],
                pre_initial_choice_token: false,
                element_index_2_tokens,
            })
        } else {
            //initiation mode 1: through one or more start events
            Ok(BPMNMarking {
                sequence_flow_2_tokens: vec![0; self.number_of_sequence_flows()],
                message_flow_2_tokens: vec![0; self.number_of_message_flows()],
                pre_initial_choice_token: true,
                element_index_2_tokens: vec![],
            })
        }
    }

    fn execute_transition(
        &self,
        state: &mut BPMNMarking,
        transition: TransitionIndex,
    ) -> Result<()> {
        todo!()
    }

    fn is_final_state(&self, state: &BPMNMarking) -> Result<bool> {
        Ok(self.get_enabled_transitions(state)?.is_empty())
    }

    fn is_transition_silent(&self, transition: TransitionIndex) -> bool {
        todo!()
    }

    fn get_transition_activity(&self, transition: TransitionIndex) -> Option<Activity> {
        todo!()
    }

    fn get_enabled_transitions(&self, state: &BPMNMarking) -> Result<Vec<TransitionIndex>> {
        //recurse to elements
        let result = self.elements.enabled_transitions(state, self)?;

        //transform to list of indices
        let mut result2 = Vec::new();
        for index in result.iter_ones() {
            result2.push(index);
        }
        Ok(result2)
    }

    fn get_number_of_transitions(&self) -> usize {
        self.elements.number_of_transitions()
    }
}

#[cfg(test)]
mod tests {
    use crate::{BusinessProcessModelAndNotation, semantics::BPMNMarking};
    use std::fs::{self};

    #[test]
    fn bpmn_semantics() {
        let fin = fs::read_to_string("testfiles/model.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
        assert_eq!(bpmn.get_number_of_transitions(), 13);

        let state = bpmn.get_initial_state().unwrap();

        assert_eq!(
            state,
            BPMNMarking {
                sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                message_flow_2_tokens: vec![],
                pre_initial_choice_token: true,
                element_index_2_tokens: vec![]
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&state).unwrap().len(), 1);
    }

    
    fn bpmn_lanes_semantics() {
        let fin = fs::read_to_string("testfiles/model-lanes.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
        assert_eq!(bpmn.get_number_of_transitions(), 12);

        let state = bpmn.get_initial_state().unwrap();

        assert_eq!(
            state,
            BPMNMarking {
                sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                message_flow_2_tokens: vec![0],
                pre_initial_choice_token: true,
                element_index_2_tokens: vec![]
            }
        );
        println!("{:?}", bpmn.get_enabled_transitions(&state).unwrap());
        assert_eq!(bpmn.get_enabled_transitions(&state).unwrap().len(), 2);
    }
}
