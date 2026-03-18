use crate::{
    BPMNMarking, StochasticBusinessProcessModelAndNotation, marking::Token,
    semantics::TransitionIndex,
};
use anyhow::{Result, anyhow};
use ebi_activity_key::Activity;
use ebi_arithmetic::{ChooseRandomly, Fraction, One};

/// A hypergraph representing a partially ordered run of an SBPMN model
pub struct PartiallyOrderedRun {
    pub state_2_token: Vec<Token>,
    pub state_2_output_edge: Vec<Option<usize>>,
    pub state_2_input_edge: Vec<Option<usize>>,
    pub edge_2_inputs: Vec<Vec<usize>>,
    pub edge_2_outputs: Vec<Vec<usize>>,
    pub edge_2_activity: Vec<Option<Activity>>,
    terminated: bool,
}

impl PartiallyOrderedRun {
    pub fn new_random(sbpmn: &StochasticBusinessProcessModelAndNotation) -> Result<Self> {
        let mut run = Self::from_initial_marking(sbpmn)?;
        run.execute_free_transitions_exhaustively(sbpmn)?;
        while !run.terminated {
            run.execute_random_transition(sbpmn)?;
            run.execute_free_transitions_exhaustively(sbpmn)?;
        }

        Ok(run)
    }

    pub fn from_initial_marking(sbpmn: &StochasticBusinessProcessModelAndNotation) -> Result<Self> {
        let mut result = Self {
            state_2_token: vec![],
            state_2_input_edge: vec![],
            state_2_output_edge: vec![],
            edge_2_activity: vec![],
            edge_2_inputs: vec![],
            edge_2_outputs: vec![],
            terminated: false,
        };
        if let Some(initial_marking) = sbpmn.get_initial_marking()? {
            for token in initial_marking.to_tokens(&sbpmn.bpmn)? {
                result.state_2_token.push(token);
                result.state_2_input_edge.push(None);
                result.state_2_output_edge.push(None);
            }
            Ok(result)
        } else {
            Err(anyhow!("SBPMN does not have partially ordered runs."))
        }
    }

    /// Front states are states that have no outgoing edge. That is, the token is still there.
    fn front_states(&self) -> Vec<usize> {
        (0..self.number_of_states())
            .filter(|state| self.state_2_output_edge[*state].is_none())
            .collect()
    }

    fn get_marking(
        &self,
        front_states: &Vec<usize>,
        sbpmn: &StochasticBusinessProcessModelAndNotation,
    ) -> Result<BPMNMarking> {
        //create empty marking
        let mut marking = BPMNMarking::new_empty(&sbpmn.bpmn);

        //fill the marking
        for token in front_states.iter().map(|state| &self.state_2_token[*state]) {
            marking.add_token(token, &sbpmn.bpmn);
        }

        Ok(marking)
    }

    fn tokens_to_states(
        &self,
        tokens: Vec<Token>,
        front_states: &Vec<usize>,
    ) -> Result<Vec<usize>> {
        let mut result = Vec::with_capacity(tokens.len());
        for token in &tokens {
            for state in front_states {
                if &self.state_2_token[*state] == token {
                    result.push(*state);
                }
            }
        }
        if result.len() == tokens.len() {
            Ok(result)
        } else {
            Err(anyhow!("not all tokens found"))
        }
    }

    fn execute_random_transition(
        &mut self,
        sbpmn: &StochasticBusinessProcessModelAndNotation,
    ) -> Result<()> {
        //create a marking
        let front_states = self.front_states();
        let marking = self.get_marking(&front_states, sbpmn)?;
        let enabled_transitions = sbpmn.get_enabled_transitions(&marking)?;
        if enabled_transitions.is_empty() {
            self.terminated = true;
            return Ok(());
        }

        //gather probabilities
        let mut outgoing_probabilities = vec![];
        for transition in &enabled_transitions {
            outgoing_probabilities.push(
                sbpmn
                    .get_transition_probabilistic_penalty(*transition, &marking)
                    .ok_or_else(|| anyhow!("transition not found"))?,
            );
        }

        // choose a transition
        let i = Fraction::choose_randomly(&outgoing_probabilities)?;
        let chosen_transition = enabled_transitions[i];

        // execute transition
        self.execute_transition(chosen_transition, &marking, &front_states, sbpmn)?;
        Ok(())
    }

    /// Execute all transitions that have no probabilistic cost attached.
    pub fn execute_free_transitions_exhaustively(
        &mut self,
        sbpmn: &StochasticBusinessProcessModelAndNotation,
    ) -> Result<()> {
        while self.execute_a_free_transition(sbpmn)? {}

        Ok(())
    }

    /// Find an arbitrary transition without weight cost and execute it.
    /// Returns whether a transition was executed.
    fn execute_a_free_transition(
        &mut self,
        sbpmn: &StochasticBusinessProcessModelAndNotation,
    ) -> Result<bool> {
        //create a marking
        let front_states = self.front_states();
        let marking = self.get_marking(&front_states, sbpmn)?;

        for transition_index in sbpmn.get_enabled_transitions(&marking)? {
            if let Some(weight) =
                sbpmn.get_transition_probabilistic_penalty(transition_index, &marking)
                && weight.is_one()
            {
                self.execute_transition(transition_index, &marking, &front_states, sbpmn)?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Execute a transition
    pub fn execute_transition(
        &mut self,
        transition_index: TransitionIndex,
        marking: &BPMNMarking,
        front_states: &Vec<usize>,
        sbpmn: &StochasticBusinessProcessModelAndNotation,
    ) -> Result<()> {
        let new_edge = self.number_of_edges();

        //get the activity
        let activity = sbpmn
            .bpmn
            .get_transition_activity(transition_index, &marking);
        self.edge_2_activity.push(activity);

        //consume tokens
        {
            let consumed_tokens = sbpmn
                .bpmn
                .transition_2_consumed_tokens(transition_index, &marking)
                .ok_or_else(|| anyhow!("Could not obtain consumed tokens."))?;
            let consume_states = self.tokens_to_states(consumed_tokens, &front_states)?;

            //add to states
            for state in &consume_states {
                self.state_2_output_edge[*state] = Some(new_edge);
            }

            //add to edge
            self.edge_2_inputs.push(consume_states);
        }

        //produce tokens
        {
            let produced_tokens = sbpmn
                .bpmn
                .transition_2_produced_tokens(transition_index, &marking)
                .ok_or_else(|| anyhow!("Could not obtain produced tokens."))?;

            // add states
            let mut new_states = vec![];
            for token in produced_tokens {
                let new_state = self.number_of_states();
                self.state_2_token.push(token);
                self.state_2_input_edge.push(Some(new_edge));
                self.state_2_output_edge.push(None);
                new_states.push(new_state);
            }

            // add to edge
            self.edge_2_outputs.push(new_states);
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
