use crate::{
    BusinessProcessModelAndNotation, element::BPMNElement, semantics::Token,
    traits::processable::Processable,
};
use anyhow::{Result, anyhow};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BPMNMarking {
    pub(crate) element_index_2_sub_markings: Vec<BPMNSubMarking>,
    pub(crate) root_marking: BPMNRootMarking,
}

impl BPMNMarking {
    pub fn new_empty(bpmn: &BusinessProcessModelAndNotation) -> Self {
        let mut marking = BPMNMarking {
            element_index_2_sub_markings: vec![],
            root_marking: BPMNRootMarking {
                root_initial_choice_token: false,
                message_flow_2_tokens: vec![0; bpmn.number_of_message_flows()],
            },
        };
        for element in bpmn.elements_non_recursive() {
            match element {
                BPMNElement::Process(process) => {
                    marking.element_index_2_sub_markings.push(BPMNSubMarking {
                        sequence_flow_2_tokens: vec![0; process.sequence_flows.len()],
                        initial_choice_token: false,
                        element_index_2_tokens: vec![0; process.elements.len()],
                        element_index_2_sub_markings: vec![vec![]; process.elements.len()],
                    })
                }
                _ => marking
                    .element_index_2_sub_markings
                    .push(BPMNSubMarking::new_empty()),
            }
        }
        marking
    }

    pub fn add_token(
        &mut self,
        token: &Token,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        match token {
            Token::SequenceFlow(sequence_flow_global_index) => {
                let (sequence_flow, parent) = bpmn
                    .global_index_2_sequence_flow_and_parent(*sequence_flow_global_index)
                    .ok_or_else(|| anyhow!("Sequence flow not found."))?;
                self.element_index_2_sub_markings
                    .get_mut(parent.local_index())
                    .ok_or_else(|| anyhow!("Parent not found."))?
                    .sequence_flow_2_tokens[sequence_flow.local_index] += 1;
            }
            Token::MessageFlow(message_flow_global_index) => {
                let message_flow = bpmn
                    .global_index_2_message_flow(*message_flow_global_index)
                    .ok_or_else(|| anyhow!("Message flow not found."))?;
                self.root_marking.message_flow_2_tokens[message_flow.local_index] += 1;
            }
            Token::Start { in_process } => todo!(),
            Token::ParallelElement(_) => todo!(),
        }
        Ok(())
    }
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
