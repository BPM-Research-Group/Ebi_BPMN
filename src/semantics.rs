use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    traits::{
        startable::{InitiationMode, Startable},
        transitionable::Transitionable,
    },
};
use anyhow::Result;
use bitvec::bitvec;
use ebi_activity_key::Activity;
use std::rc::Rc;

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

    pub fn is_transition_silent(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNMarking,
    ) -> bool {
        for (element, sub_marking) in self
            .elements
            .iter()
            .zip(marking.element_index_2_sub_markings.iter())
        {
            let x = element.transition_activity(transition_index, sub_marking);
            if x.is_some() {
                return false;
            }
        }
        true
    }

    pub fn get_transition_activity(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNMarking,
    ) -> Option<Activity> {
        for (element, sub_marking) in self
            .elements
            .iter()
            .zip(marking.element_index_2_sub_markings.iter())
        {
            let x = element.transition_activity(transition_index, sub_marking);
            if x.is_some() {
                return x;
            }
        }
        None
    }

    pub fn get_enabled_transitions(&self, marking: &BPMNMarking) -> Result<Vec<TransitionIndex>> {
        let mut result = bitvec![0;0];
        for (element, sub_marking) in self
            .elements
            .iter()
            .zip(marking.element_index_2_sub_markings.iter())
        {
            result.extend(element.enabled_transitions(sub_marking, self, self)?);
        }

        //transform to list of indices
        let mut result2 = Vec::new();
        for index in result.iter_ones() {
            result2.push(index);
        }
        Ok(result2)
    }

    pub fn number_of_transitions(&self, marking: &BPMNMarking) -> usize {
        let mut result = 0;
        for (element, sub_marking) in self
            .elements
            .iter()
            .zip(marking.element_index_2_sub_markings.iter())
        {
            result += element.number_of_transitions(sub_marking);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BusinessProcessModelAndNotation,
        semantics::{BPMNMarking, BPMNSubMarking},
        traits::transitionable::Transitionable,
    };
    use std::{
        collections::HashMap,
        fs::{self},
    };

    pub fn debug_transitions(bpmn: &BusinessProcessModelAndNotation, marking: &BPMNMarking) {
        println!("transitions");
        for transition_index in 0..bpmn.number_of_transitions(&marking) {
            println!(
                "\ttransition {} \t {}",
                transition_index,
                bpmn.transition_debug(transition_index, marking)
                    .unwrap_or("None".to_string())
            );
        }
    }

    #[test]
    fn bpmn_semantics() {
        let fin = fs::read_to_string("testfiles/model.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        let state = bpmn.get_initial_state().unwrap();
        assert_eq!(bpmn.number_of_transitions(&state), 13);

        assert_eq!(
            state,
            BPMNMarking {
                message_flow_2_tokens: vec![],
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0],
                    initial_choice_token: true,
                    element_index_2_tokens: vec![],
                    element_index_2_sub_markings: vec![],
                    root_marking: todo!()
                }],
                root_initial_choice_token: true,
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&state).unwrap().len(), 1);
    }

    // #[test]
    // fn bpmn_lanes_semantics() {
    //     let fin = fs::read_to_string("testfiles/model-lanes.bpmn").unwrap();
    //     let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    //     assert_eq!(bpmn.number_of_transitions(), 13);

    //     let state = bpmn.get_initial_state().unwrap();

    //     assert_eq!(
    //         state,
    //         BPMNMarking {
    //             sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    //             message_flow_2_tokens: vec![0],
    //             root_initial_choice_token: true,
    //             sub_initial_choice_tokens: HashMap::new(),
    //             element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    //         }
    //     );

    //     debug_transitions(&bpmn);

    //     println!("{:?}", bpmn.get_enabled_transitions(&state).unwrap());
    //     assert_eq!(bpmn.get_enabled_transitions(&state).unwrap().len(), 2);
    // }
}
