use std::{collections::HashMap, rc::Rc};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BPMNSubMarking {
    pub(crate) sequence_flow_2_tokens: Vec<u64>,
    pub(crate) initial_choice_token: bool,
    pub(crate) element_index_2_tokens: Vec<u64>,
    pub(crate) element_index_2_sub_markings: Vec<Vec<BPMNSubMarking>>,
    pub(crate) root_marking: Rc<BPMNMarking>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BPMNMarking {
    pub(crate) message_flow_2_tokens: Vec<u64>,
    pub(crate) element_index_2_sub_markings: Vec<BPMNSubMarking>,
    pub(crate) root_initial_choice_token: bool,
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

        if mode.is_choice_between_start_events() {
            Ok(BPMNMarking {
                message_flow_2_tokens: vec![0; self.message_flows.len()],
                element_index_2_sub_markings: vec![],
                root_initial_choice_token: true,
            })
        } else {
            todo!()
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
