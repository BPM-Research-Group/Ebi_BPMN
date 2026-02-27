use std::collections::HashMap;

use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    traits::{
        objectable::BPMNObject,
        startable::{InitiationMode, Startable},
        transitionable::Transitionable,
    },
};
use anyhow::Result;
use ebi_activity_key::Activity;

pub type TransitionIndex = usize;

#[derive(Debug, PartialEq, Eq)]
pub struct BPMNMarking {
    pub(crate) sequence_flow_2_tokens: Vec<u64>,
    pub(crate) message_flow_2_tokens: Vec<u64>,

    /// in case multiple start events are present, a single root token is added
    pub(crate) root_initial_choice_token: bool,

    // in case multiple start events are present in a sub-process, root tokens are added
    pub(crate) sub_initial_choice_tokens: HashMap<usize, usize>,

    /// in case no start events are present, every eligible element without incoming sequence flows gets a token
    pub(crate) element_index_2_tokens: Vec<u64>,
}

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
                root_initial_choice_token: false,
                sub_initial_choice_tokens: HashMap::new(),
                element_index_2_tokens,
            })
        } else {
            //initiation mode 1: through one or more start events
            Ok(BPMNMarking {
                sequence_flow_2_tokens: vec![0; self.number_of_sequence_flows()],
                message_flow_2_tokens: vec![0; self.number_of_message_flows()],
                root_initial_choice_token: true,
                sub_initial_choice_tokens: HashMap::new(),
                element_index_2_tokens: vec![0; self.number_of_elements()],
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

    pub fn is_final_state(&self, state: &BPMNMarking) -> Result<bool> {
        Ok(self.get_enabled_transitions(state)?.is_empty())
    }

    pub fn is_transition_silent(&self, transition_index: TransitionIndex) -> bool {
        self.elements
            .transition_activity(transition_index)
            .is_none()
    }

    pub fn get_transition_activity(&self, transition_index: TransitionIndex) -> Option<Activity> {
        self.elements.transition_activity(transition_index)
    }

    pub fn get_enabled_transitions(&self, state: &BPMNMarking) -> Result<Vec<TransitionIndex>> {
        //recurse to elements
        let result = self.elements.enabled_transitions(state, None, self)?;

        //transform to list of indices
        let mut result2 = Vec::new();
        for index in result.iter_ones() {
            result2.push(index);
        }
        Ok(result2)
    }

    pub fn number_of_transitions(&self) -> usize {
        self.elements.number_of_transitions()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BusinessProcessModelAndNotation, semantics::BPMNMarking,
        traits::transitionable::Transitionable,
    };
    use std::{
        collections::HashMap,
        fs::{self},
    };

    pub fn debug_transitions(bpmn: &BusinessProcessModelAndNotation) {
        println!("transitions");
        for transition_index in 0..bpmn.number_of_transitions() {
            println!(
                "\ttransition {} \t {}",
                transition_index,
                bpmn.elements
                    .transition_debug(transition_index)
                    .unwrap_or("None".to_string())
            );
        }
    }

    #[test]
    fn bpmn_semantics() {
        let fin = fs::read_to_string("testfiles/model.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
        assert_eq!(bpmn.number_of_transitions(), 13);

        let state = bpmn.get_initial_state().unwrap();

        assert_eq!(
            state,
            BPMNMarking {
                sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                message_flow_2_tokens: vec![],
                root_initial_choice_token: true,
                sub_initial_choice_tokens: HashMap::new(),
                element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&state).unwrap().len(), 1);
    }

    //#[test]
    fn bpmn_lanes_semantics() {
        let fin = fs::read_to_string("testfiles/model-lanes.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
        assert_eq!(bpmn.number_of_transitions(), 13);

        let state = bpmn.get_initial_state().unwrap();

        assert_eq!(
            state,
            BPMNMarking {
                sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                message_flow_2_tokens: vec![0],
                root_initial_choice_token: true,
                sub_initial_choice_tokens: HashMap::new(),
                element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }
        );

        debug_transitions(&bpmn);

        println!("{:?}", bpmn.get_enabled_transitions(&state).unwrap());
        assert_eq!(bpmn.get_enabled_transitions(&state).unwrap().len(), 2);
    }
}
