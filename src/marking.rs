use crate::{
    BusinessProcessModelAndNotation, GlobalIndex,
    element::BPMNElement,
    if_not::IfNot,
    traits::{objectable::BPMNObject, processable::Processable, searchable::Searchable},
};
use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BPMNMarking {
    pub(crate) element_index_2_sub_markings: Vec<BPMNSubMarking>,
    pub(crate) root_marking: BPMNRootMarking,
}

impl BPMNMarking {
    /// Creates a new marking that is empty.
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

    pub fn to_tokens(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<Vec<Token>> {
        let mut result = vec![];

        //root start
        if self.root_marking.root_initial_choice_token {
            result.push(Token::RootStart);
        }

        //messages
        for (message_flow_index, message_tokens) in
            self.root_marking.message_flow_2_tokens.iter().enumerate()
        {
            for _ in 0..*message_tokens {
                let message_flow = bpmn
                    .message_flows
                    .get(message_flow_index)
                    .and_if_not("message flow not found")?;
                result.push(Token::MessageFlow(message_flow.global_index));
            }
        }

        // sub-markings
        for (element_index, sub_marking) in self.element_index_2_sub_markings.iter().enumerate() {
            let element = bpmn
                .elements
                .get(element_index)
                .and_if_not("Element not found.")?;

            //add initial choice tokens
            if sub_marking.initial_choice_token {
                result.push(Token::SubProcessStart {
                    in_process: element.global_index(),
                });
                return Err(anyhow!("Sub-processes are not supported here for now."));
            }

            //add element tokens
            for (sub_element_index, tokens) in sub_marking.element_index_2_tokens.iter().enumerate()
            {
                for _ in 0..*tokens {
                    let sub_element = element
                        .local_index_2_element(sub_element_index)
                        .ok_or_else(|| anyhow!("message flow not found"))?;
                    result.push(Token::Element(sub_element.global_index()));
                }
            }
        }

        Ok(result)
    }

    /// Updates the marking with the token.
    pub fn add_token(
        &mut self,
        token: &Token,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        match token {
            Token::SequenceFlow(sequence_flow_global_index) => {
                let (sequence_flow, parent) = bpmn
                    .global_index_2_sequence_flow_and_parent(*sequence_flow_global_index)
                    .and_if_not("Sequence flow not found.")?;
                if parent.is_sub_process() {
                    return Err(anyhow!("Sub-processes are not supported for now."));
                }
                self.element_index_2_sub_markings
                    .get_mut(parent.local_index())
                    .and_if_not("Parent not found.")?
                    .sequence_flow_2_tokens[sequence_flow.local_index] += 1;
            }
            Token::MessageFlow(message_flow_global_index) => {
                let message_flow = bpmn
                    .global_index_2_message_flow(*message_flow_global_index)
                    .and_if_not("Message flow not found.")?;
                self.root_marking.message_flow_2_tokens[message_flow.local_index] += 1;
            }
            Token::RootStart => {
                self.root_marking.root_initial_choice_token = true;
            }
            Token::SubProcessStart { .. } => {
                return Err(anyhow!("Sub-processes are not supported for now."));
            }
            Token::Element(global_index) => {
                let element = bpmn
                    .global_index_2_element(*global_index)
                    .and_if_not("Element not found.")?;
                let parent = bpmn
                    .parent_of(element.global_index())
                    .and_if_not("Parent not found.")?;

                if parent.is_sub_process() {
                    return Err(anyhow!("Sub-processes are not supported for now."));
                }

                let sub_marking = self
                    .element_index_2_sub_markings
                    .get_mut(parent.local_index())
                    .and_if_not("Sub-marking not found.")?;

                *sub_marking
                    .element_index_2_tokens
                    .get_mut(parent.local_index())
                    .and_if_not("Element not found.")? += 1;
            }
        }
        Ok(())
    }
}

impl Display for BPMNMarking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        if !self.root_marking.is_empty() {
            write!(f, "root: {}", self.root_marking)?;
        }
        if self
            .element_index_2_sub_markings
            .iter()
            .any(|x| !x.is_empty())
        {
            write!(f, "sub-markings: [")?;
            for (i, x) in self.element_index_2_sub_markings.iter().enumerate() {
                write!(f, "{}: {}", i, x)?;
            }
            write!(f, "]")?;
        }
        write!(f, "}}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BPMNRootMarking {
    pub(crate) root_initial_choice_token: bool,
    pub(crate) message_flow_2_tokens: Vec<u64>,
}

impl BPMNRootMarking {
    pub fn is_empty(&self) -> bool {
        !self.root_initial_choice_token && self.message_flow_2_tokens.iter().all(|x| x == &0)
    }
}

impl Display for BPMNRootMarking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "empty")
        } else {
            if self.root_initial_choice_token {
                write!(f, "initial choice")?;
            }
            if self.message_flow_2_tokens.iter().any(|x| x > &0) {
                write!(f, "messages: [")?;
                for message_flow in &self.message_flow_2_tokens {
                    write!(f, "{}", message_flow)?;
                }
                write!(f, "]")?;
            }
            write!(f, "")
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BPMNSubMarking {
    pub(crate) sequence_flow_2_tokens: Vec<u64>,
    pub(crate) initial_choice_token: bool,
    pub(crate) element_index_2_tokens: Vec<u64>,
    pub(crate) element_index_2_sub_markings: Vec<Vec<BPMNSubMarking>>,
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

    pub fn is_empty(&self) -> bool {
        self.sequence_flow_2_tokens.iter().all(|x| *x == 0)
            && !self.initial_choice_token
            && self.element_index_2_tokens.iter().all(|x| x == &0)
            && self
                .element_index_2_sub_markings
                .iter()
                .all(|s| s.is_empty())
    }
}

impl Display for BPMNSubMarking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = vec![];
        if self.initial_choice_token {
            result.push("initial choice".to_string());
        }

        if self.sequence_flow_2_tokens.iter().any(|x| *x > 0) {
            result.push(format!(
                "sequence flows: [{}]",
                self.sequence_flow_2_tokens
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if x > &1 {
                            Some(format!("{}^{}", i, x))
                        } else if *x == 1 {
                            Some(format!("{}", i))
                        } else {
                            None
                        }
                    })
                    .join(", ")
            ));
        }

        if self.element_index_2_tokens.iter().any(|x| *x > 0) {
            result.push(format!(
                "elements: [{}]",
                self.element_index_2_tokens
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if x > &1 {
                            Some(format!("{}^{}", i, x))
                        } else if *x == 1 {
                            Some(format!("{}", i))
                        } else {
                            None
                        }
                    })
                    .join(", ")
            ));
        }

        if self
            .element_index_2_sub_markings
            .iter()
            .any(|x| x.len() > 0)
        {
            result.push(format!(
                "\nsub-markings [{}]",
                self.element_index_2_sub_markings
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if x.len() > 1 {
                            format!(
                                "{}: [{}]",
                                i,
                                x.iter().map(|submarking| submarking.to_string()).join("\n")
                            )
                        } else {
                            format!(
                                "{}: {}",
                                i,
                                x.iter().map(|submarking| submarking.to_string()).join("")
                            )
                        }
                    })
                    .join(", ")
            ));
        }

        write!(f, "{{{}}}", result.join(", "))
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    /// A token on a sequence flow.
    SequenceFlow(GlobalIndex),

    /// A token on a message flow
    MessageFlow(GlobalIndex),

    /// A virtual token that allows start events at the root to fire.
    RootStart,

    /// A virtual token that allows start events of a sub-process to fire.
    SubProcessStart { in_process: GlobalIndex },

    /// A token in front of an element on a virtual sequence flow; used if there are no start events to start the process with.
    Element(GlobalIndex),
}
