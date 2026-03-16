use std::collections::VecDeque;

use crate::{
    BPMNMarking, BusinessProcessModelAndNotation, GlobalIndex,
    StochasticBusinessProcessModelAndNotation,
    traits::{objectable::BPMNObject, searchable::Searchable},
};
use anyhow::{Result, anyhow};
use ebi_activity_key::Activity;

/// A hypergraph representing a
pub struct PartiallyOrderedRun {
    state_2_token: Vec<Token>,
    edge_2_inputs: Vec<Vec<usize>>,
    edge_2_outputs: Vec<Vec<usize>>,

    edge_2_activity: Vec<Option<Activity>>,
}

impl PartiallyOrderedRun {
    pub fn new() -> Self {
        Self {
            state_2_token: vec![],
            edge_2_inputs: vec![],
            edge_2_outputs: vec![],
            edge_2_activity: vec![],
        }
    }

    pub fn add_initial_marking(
        &mut self,
        marking: &BPMNMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        //add messages
        for (message_flow_index, message_tokens) in marking
            .root_marking
            .message_flow_2_tokens
            .iter()
            .enumerate()
        {
            for _ in 0..*message_tokens {
                let message_flow = bpmn
                    .message_flows
                    .get(message_flow_index)
                    .ok_or_else(|| anyhow!("message flow not found"))?;
                self.state_2_token
                    .push(Token::MessageFlow(message_flow.global_index));
            }
        }

        for (element_index, sub_marking) in marking.element_index_2_sub_markings.iter().enumerate()
        {
            let element = bpmn
                .elements
                .get(element_index)
                .ok_or_else(|| anyhow!("element not found"))?;

            //add initial choice tokens
            if sub_marking.initial_choice_token {
                self.state_2_token.push(Token::Start {
                    in_process: element.global_index(),
                });
            }

            //add element tokens
            for (sub_element_index, tokens) in sub_marking.element_index_2_tokens.iter().enumerate()
            {
                for _ in 0..*tokens {
                    let sub_element = element
                        .local_index_2_element(sub_element_index)
                        .ok_or_else(|| anyhow!("message flow not found"))?;
                    self.state_2_token
                        .push(Token::ParallelElement(sub_element.global_index()));
                }
            }
        }

        Ok(())
    }

    pub fn number_of_states(&self) -> usize {
        self.state_2_token.len()
    }

    pub fn number_of_edges(&self) -> usize {
        self.edge_2_activity.len()
    }
}

pub enum Token {
    /// A token on a sequence flow.
    SequenceFlow(GlobalIndex),

    /// A token on a message flow
    MessageFlow(GlobalIndex),

    /// A virtual token in front of every start event; used to get the process started.
    Start { in_process: GlobalIndex },

    /// A token in front of an element on a virtual sequence flow; used if there are no start events to start the process with.
    ParallelElement(GlobalIndex),
}

pub fn random_partially_ordered_run(
    sbpmn: &StochasticBusinessProcessModelAndNotation,
) -> Result<PartiallyOrderedRun> {
    if let Some(initial_marking) = sbpmn.get_initial_marking()? {
        let mut run = PartiallyOrderedRun::new();
        run.add_initial_marking(&initial_marking, &sbpmn.bpmn)?;

        let mut queue = VecDeque::new();
        queue.extend(0..run.number_of_states());

        while let Some(state_index) = queue.pop_front() {

        }

        Ok(run)
    } else {
        Err(anyhow!("SBPMN does not have partially ordered runs."))
    }
}
