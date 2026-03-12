use std::fmt::Display;

use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
    traits::{
        processable::Processable,
        startable::{InitiationMode, Startable},
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
use bitvec::bitvec;
use ebi_activity_key::Activity;
use ebi_arithmetic::Fraction;

pub type TransitionIndex = usize;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BPMNMarking {
    pub(crate) element_index_2_sub_markings: Vec<BPMNSubMarking>,
    pub(crate) root_marking: BPMNRootMarking,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BPMNRootMarking {
    pub(crate) root_initial_choice_token: bool,
    pub(crate) message_flow_2_tokens: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BPMNSubMarking {
    pub(crate) sequence_flow_2_tokens: Vec<u64>,
    pub(crate) initial_choice_token: bool,
    pub(crate) element_index_2_tokens: Vec<u64>,
    pub(crate) element_index_2_sub_markings: Vec<Vec<BPMNSubMarking>>,
}

impl Display for BPMNMarking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", stringify!(self))
    }
}

impl BPMNSubMarking {
    pub(crate) fn new_empty() -> Self {
        Self {
            sequence_flow_2_tokens: vec![],
            initial_choice_token: false,
            element_index_2_tokens: vec![],
            element_index_2_sub_markings: vec![],
        }
    }
}

impl BusinessProcessModelAndNotation {
    /// Returns the initial marking, as specified by the BPMN 2.0.2 standard on page 238.
    /// Additionally, if the model is empty, it does not support any trace, and this function returns Ok(None).
    /// If the model is structurally correct, this method will always return Ok(..).
    /// If not, will return Err() but will not panic.
    pub fn get_initial_marking(&self) -> Result<Option<BPMNMarking>> {
        if self.elements.is_empty() {
            return Ok(None);
        }

        //gather the initiation mode
        let mut initiation_mode = InitiationMode::ParallelElements(vec![]);
        for element in &self.elements {
            if let BPMNElement::Process(process) = element {
                initiation_mode = initiation_mode + process.initiation_mode(self)?;
            }
        }

        if initiation_mode.is_choice_between_start_events() {
            let root_marking = BPMNRootMarking {
                message_flow_2_tokens: vec![0; self.message_flows.len()],
                root_initial_choice_token: true,
            };

            let mut element_index_2_sub_markings = Vec::with_capacity(self.elements.len());
            for element in self.elements.iter() {
                if let BPMNElement::Process(process) = element {
                    element_index_2_sub_markings.push(process.to_sub_marking(&initiation_mode)?);
                } else {
                    element_index_2_sub_markings.push(BPMNSubMarking::new_empty());
                }
            }

            Ok(Some(BPMNMarking {
                element_index_2_sub_markings,
                root_marking,
            }))
        } else {
            todo!()
        }
    }

    /// Updates the marking by executing the transition.
    /// By contract, will return Ok() if the model is structurally correct and the transition was enabled.
    /// May panic or return Err() otherwise.
    pub fn execute_transition(
        &self,
        marking: &mut BPMNMarking,
        mut transition_index: TransitionIndex,
    ) -> Result<()> {
        let transition_index_debug = transition_index;
        let BPMNMarking {
            element_index_2_sub_markings,
            root_marking,
        } = marking;
        for (element, sub_marking) in self
            .elements
            .iter()
            .zip(element_index_2_sub_markings.iter_mut())
        {
            let number_of_transitions = element.number_of_transitions(sub_marking);
            if transition_index < number_of_transitions {
                return element.execute_transition(
                    transition_index,
                    root_marking,
                    sub_marking,
                    self,
                    self,
                );
            }
            transition_index -= number_of_transitions;
        }
        Err(anyhow!(
            "transition {} is not enabled, as it is unknown",
            transition_index_debug
        ))
    }

    pub fn is_final_marking(&self, marking: &BPMNMarking) -> Result<bool> {
        Ok(self.get_enabled_transitions(marking)?.is_empty())
    }

    /// Returns `true` if the transition exists and is unlabelled, otherwise, returns false.
    /// Does not panic.
    pub fn is_transition_silent(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNMarking,
    ) -> bool {
        self.get_transition_activity(transition_index, marking)
            .is_none()
    }

    /// If the transition exists and is labelled, returns the label. Otherwise, returns None.
    pub fn get_transition_activity(
        &self,
        mut transition_index: TransitionIndex,
        marking: &BPMNMarking,
    ) -> Option<Activity> {
        for (element, sub_marking) in self
            .elements
            .iter()
            .zip(marking.element_index_2_sub_markings.iter())
        {
            let number_of_transitions = element.number_of_transitions(sub_marking);
            if transition_index < number_of_transitions {
                return element.transition_activity(transition_index, sub_marking);
            }
            transition_index -= number_of_transitions;
        }
        None
    }

    /// Returns the transitions that are enabled in the given `marking`.
    /// By contract, will return Ok() if the model is structurally correct. Otherwise, it will return Err() but will not panic.
    pub fn get_enabled_transitions(&self, marking: &BPMNMarking) -> Result<Vec<TransitionIndex>> {
        let mut result = bitvec![0;0];
        for (element, sub_marking) in self
            .elements
            .iter()
            .zip(marking.element_index_2_sub_markings.iter())
        {
            result.extend(element.enabled_transitions(
                &marking.root_marking,
                sub_marking,
                self,
                self,
            )?);
        }

        //transform to list of indices
        let mut result2 = Vec::new();
        for index in result.iter_ones() {
            result2.push(index);
        }
        Ok(result2)
    }

    /// Returns the number of transitions of the current marking. Note that not all of these transitions are necessarily enabled.
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

impl StochasticBusinessProcessModelAndNotation {
    /// BPMN 2.0.2 standard page 238
    /// By convention, if the model is empty, it does not support any trace, and this function returns Ok(None).
    /// If the model is structurally correct, this method will return Ok(..).
    pub fn get_initial_marking(&self) -> Result<Option<BPMNMarking>> {
        self.bpmn.get_initial_marking()
    }

    pub fn execute_transition(
        &self,
        marking: &mut BPMNMarking,
        transition_index: TransitionIndex,
    ) -> Result<()> {
        self.bpmn.execute_transition(marking, transition_index)
    }

    pub fn is_final_marking(&self, marking: &BPMNMarking) -> Result<bool> {
        self.bpmn.is_final_marking(marking)
    }

    pub fn is_transition_silent(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNMarking,
    ) -> bool {
        self.bpmn.is_transition_silent(transition_index, marking)
    }

    pub fn get_transition_activity(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNMarking,
    ) -> Option<Activity> {
        self.bpmn.get_transition_activity(transition_index, marking)
    }

    pub fn get_enabled_transitions(&self, marking: &BPMNMarking) -> Result<Vec<TransitionIndex>> {
        self.bpmn.get_enabled_transitions(marking)
    }

    pub fn number_of_transitions(&self, marking: &BPMNMarking) -> usize {
        self.bpmn.number_of_transitions(marking)
    }

    pub fn get_transition_weight(
        &self,
        mut transition_index: TransitionIndex,
        marking: &BPMNMarking,
    ) -> Option<Fraction> {
        for (element, sub_marking) in self
            .bpmn
            .elements
            .iter()
            .zip(marking.element_index_2_sub_markings.iter())
        {
            let number_of_transitions = element.number_of_transitions(sub_marking);
            if transition_index < number_of_transitions {
                return element.transition_weight(transition_index, sub_marking, &self.bpmn);
            }
            transition_index -= number_of_transitions;
        }
        None
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use ebi_arithmetic::{Fraction, One, f};

    use crate::{
        BusinessProcessModelAndNotation,
        semantics::{BPMNMarking, BPMNRootMarking, BPMNSubMarking},
        stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
    };
    use std::fs::{self};

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

        let mut marking = bpmn.get_initial_marking().unwrap().unwrap();
        assert_eq!(bpmn.number_of_transitions(&marking), 13);
        debug_transitions(&bpmn, &marking);

        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: true,
                    message_flow_2_tokens: vec![]
                },
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    initial_choice_token: false,
                    element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    element_index_2_sub_markings: vec![vec![]; 9],
                }],
            }
        );
        let enabled = bpmn.get_enabled_transitions(&marking).unwrap();
        assert_eq!(enabled, [0]);

        //execute start event
        bpmn.execute_transition(&mut marking, 0).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![]
                },
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    initial_choice_token: false,
                    element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    element_index_2_sub_markings: vec![vec![]; 9],
                }],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [1]);

        //execute task
        assert_eq!(
            bpmn.activity_key
                .deprocess_activity(&bpmn.get_transition_activity(1, &marking).unwrap()),
            "Register claim\n(2min)"
        );
        bpmn.execute_transition(&mut marking, 1).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![]
                },
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    initial_choice_token: false,
                    element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    element_index_2_sub_markings: vec![vec![]; 9],
                }],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [2, 3]);

        //execute XOR split
        bpmn.is_transition_silent(3, &marking);
        bpmn.execute_transition(&mut marking, 3).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![]
                },
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                    initial_choice_token: false,
                    element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    element_index_2_sub_markings: vec![vec![]; 9],
                }],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [5]);

        //execute task
        assert_eq!(
            bpmn.activity_key
                .deprocess_activity(&bpmn.get_transition_activity(5, &marking).unwrap()),
            "Check easy claim\n(5 min)"
        );
        bpmn.execute_transition(&mut marking, 5).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![]
                },
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                    initial_choice_token: false,
                    element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    element_index_2_sub_markings: vec![vec![]; 9],
                }],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [9, 10]);

        //execute XOR split
        bpmn.is_transition_silent(9, &marking);
        bpmn.execute_transition(&mut marking, 9).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![]
                },
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                    initial_choice_token: false,
                    element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    element_index_2_sub_markings: vec![vec![]; 9],
                }],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [7]);

        //execute XOR join
        bpmn.is_transition_silent(7, &marking);
        bpmn.execute_transition(&mut marking, 7).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![]
                },
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                    initial_choice_token: false,
                    element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    element_index_2_sub_markings: vec![vec![]; 9],
                }],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [8]);
        assert!(!bpmn.is_final_marking(&marking).unwrap());

        //execute end event
        bpmn.is_transition_silent(8, &marking);
        bpmn.execute_transition(&mut marking, 8).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![]
                },
                element_index_2_sub_markings: vec![BPMNSubMarking {
                    sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    initial_choice_token: false,
                    element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    element_index_2_sub_markings: vec![vec![]; 9],
                }],
            }
        );
        assert_eq!(
            bpmn.get_enabled_transitions(&marking).unwrap(),
            Vec::<usize>::new()
        );
        assert!(bpmn.is_final_marking(&marking).unwrap());
    }

    #[test]
    fn bpmn_lanes_semantics() {
        let fin = fs::read_to_string("testfiles/model-lanes.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        let mut marking = bpmn.get_initial_marking().unwrap().unwrap();
        debug_transitions(&bpmn, &marking);

        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: true,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![vec![]; 8],
                    }
                ],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [1, 6]);

        //execute start event
        bpmn.is_transition_silent(1, &marking);
        bpmn.execute_transition(&mut marking, 1).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![1, 0, 0, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![vec![]; 8],
                    }
                ],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [0]);

        //start expanded sub-process
        bpmn.is_transition_silent(0, &marking);
        bpmn.execute_transition(&mut marking, 0).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![BPMNSubMarking {
                                sequence_flow_2_tokens: vec![0, 0],
                                initial_choice_token: true,
                                element_index_2_tokens: vec![0, 0, 0],
                                element_index_2_sub_markings: vec![vec![]; 3]
                            }],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );
        debug_transitions(&bpmn, &marking);
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [2]);

        //execute start event
        bpmn.is_transition_silent(2, &marking);
        bpmn.execute_transition(&mut marking, 2).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![BPMNSubMarking {
                                sequence_flow_2_tokens: vec![1, 0],
                                initial_choice_token: false,
                                element_index_2_tokens: vec![0, 0, 0],
                                element_index_2_sub_markings: vec![vec![]; 3]
                            }],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [3]);

        // execute task
        assert_eq!(
            bpmn.activity_key
                .deprocess_activity(&bpmn.get_transition_activity(3, &marking).unwrap()),
            ""
        );
        assert!(!bpmn.is_transition_silent(3, &marking));
        bpmn.execute_transition(&mut marking, 3).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![BPMNSubMarking {
                                sequence_flow_2_tokens: vec![0, 1],
                                initial_choice_token: false,
                                element_index_2_tokens: vec![0, 0, 0],
                                element_index_2_sub_markings: vec![vec![]; 3]
                            }],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [4]);

        //execute end event
        assert!(bpmn.is_transition_silent(4, &marking));
        bpmn.execute_transition(&mut marking, 4).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![BPMNSubMarking {
                                sequence_flow_2_tokens: vec![0, 0],
                                initial_choice_token: false,
                                element_index_2_tokens: vec![0, 0, 0],
                                element_index_2_sub_markings: vec![vec![]; 3]
                            }],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [1]);

        //execute termination of sub-process
        assert!(bpmn.is_transition_silent(1, &marking));
        bpmn.execute_transition(&mut marking, 1).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 1, 1, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );
        debug_transitions(&bpmn, &marking);
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [2, 5]);

        //execute collapsed sub-process
        assert_eq!(
            bpmn.activity_key
                .deprocess_activity(&bpmn.get_transition_activity(5, &marking).unwrap()),
            ""
        );
        bpmn.execute_transition(&mut marking, 5).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 1, 0, 0, 0, 1],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [2, 8]);

        //execute message end event
        assert!(bpmn.is_transition_silent(8, &marking));
        bpmn.execute_transition(&mut marking, 8).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 1, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [2]);

        // execute collapsed sub-process
        assert_eq!(
            bpmn.activity_key
                .deprocess_activity(&bpmn.get_transition_activity(2, &marking).unwrap()),
            "collapsed subprocess"
        );
        bpmn.execute_transition(&mut marking, 2).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 1, 0, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [4]);

        //execute end event
        assert!(bpmn.is_transition_silent(4, &marking));
        bpmn.execute_transition(&mut marking, 4).unwrap();
        assert_eq!(
            marking,
            BPMNMarking {
                root_marking: BPMNRootMarking {
                    root_initial_choice_token: false,
                    message_flow_2_tokens: vec![0]
                },
                element_index_2_sub_markings: vec![
                    BPMNSubMarking::new_empty(),
                    BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0, 0, 0, 0, 0, 0, 0],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0, 0, 0, 0, 0, 0, 0, 0],
                        element_index_2_sub_markings: vec![
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                            vec![],
                        ],
                    }
                ],
            }
        );

        assert!(bpmn.is_final_marking(&marking).unwrap());
    }

    #[test]
    fn bpmn_or_import() {
        let fin = fs::read_to_string("testfiles/and-a-b-xor-c-or.sbpmn").unwrap();
        let bpmn = fin
            .parse::<StochasticBusinessProcessModelAndNotation>()
            .unwrap();

        let mut marking = bpmn.get_initial_marking().unwrap().unwrap();
        debug_transitions(&bpmn.bpmn, &marking);
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [0]);
        assert_eq!(
            bpmn.get_transition_weight(0, &marking).unwrap(),
            Fraction::one()
        );

        bpmn.execute_transition(&mut marking, 0).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [1]);
        assert_eq!(
            bpmn.get_transition_weight(1, &marking).unwrap(),
            Fraction::one()
        );

        bpmn.execute_transition(&mut marking, 1).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [2, 3]);
        assert_eq!(
            bpmn.get_transition_weight(3, &marking).unwrap(),
            Fraction::one()
        );
        assert_eq!(
            bpmn.get_transition_weight(2, &marking).unwrap(),
            Fraction::one()
        );

        bpmn.execute_transition(&mut marking, 2).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [3]);
        assert_eq!(
            bpmn.get_transition_weight(3, &marking).unwrap(),
            Fraction::one()
        );

        bpmn.execute_transition(&mut marking, 3).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [4, 5]);
        assert_eq!(bpmn.get_transition_weight(4, &marking).unwrap(), f!(1, 3));
        assert_eq!(bpmn.get_transition_weight(5, &marking).unwrap(), f!(2, 3));

        bpmn.execute_transition(&mut marking, 4).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [6, 7]);
        assert_eq!(
            bpmn.get_transition_weight(6, &marking).unwrap(),
            Fraction::one()
        );
        assert_eq!(
            bpmn.get_transition_weight(7, &marking).unwrap(),
            Fraction::one()
        );

        bpmn.execute_transition(&mut marking, 6).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [7, 9]);
        assert_eq!(
            bpmn.get_transition_weight(7, &marking).unwrap(),
            Fraction::one()
        );
        assert_eq!(
            bpmn.get_transition_weight(9, &marking).unwrap(),
            Fraction::one()
        );

        bpmn.execute_transition(&mut marking, 7).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [8, 9]);
        assert_eq!(
            bpmn.get_transition_weight(8, &marking).unwrap(),
            Fraction::one()
        );
        assert_eq!(
            bpmn.get_transition_weight(9, &marking).unwrap(),
            Fraction::one()
        );

        bpmn.execute_transition(&mut marking, 8).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [9]);
        assert_eq!(
            bpmn.get_transition_weight(9, &marking).unwrap(),
            Fraction::one()
        );

        bpmn.execute_transition(&mut marking, 9).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), [0; 0]);
        assert!(bpmn.is_final_marking(&marking).unwrap());
    }

    #[test]
    fn bpmn_eventbasedgateway() {
        let fin = fs::read_to_string("testfiles/eventbasedgateway.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        let mut marking = bpmn.get_initial_marking().unwrap().unwrap();
        debug_transitions(&bpmn, &marking);
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), vec![0]);

        bpmn.execute_transition(&mut marking, 0).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), vec![1]);

        bpmn.execute_transition(&mut marking, 1).unwrap();
        assert_eq!(
            bpmn.get_enabled_transitions(&marking).unwrap(),
            vec![2, 6, 7]
        );

        bpmn.execute_transition(&mut marking, 6).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), vec![3]);

        bpmn.execute_transition(&mut marking, 3).unwrap();
        assert_eq!(bpmn.get_enabled_transitions(&marking).unwrap(), vec![0; 0]);
        assert!(bpmn.is_final_marking(&marking).unwrap());
    }
}
