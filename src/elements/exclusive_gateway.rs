use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    if_not::{IfNot, IfNotDefault},
    marking::{BPMNRootMarking, BPMNSubMarking, Token},
    parser::parser_state::GlobalIndex,
    semantics::TransitionIndex,
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;
use ebi_arithmetic::{Fraction, One};

#[derive(Debug, Clone)]
pub struct BPMNExclusiveGateway {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNExclusiveGateway {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("gateways cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("gateways cannot have outgoing message flows"))
    }

    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNExclusiveGateway {
    fn local_index(&self) -> usize {
        self.local_index
    }

    fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn activity(&self) -> Option<Activity> {
        None
    }

    fn is_unconstrained_start_event(
        &self,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<bool> {
        Ok(false)
    }

    fn is_end_event(&self) -> bool {
        false
    }

    fn incoming_sequence_flows(&self) -> &[usize] {
        &self.incoming_sequence_flows
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &self.outgoing_sequence_flows
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn can_start_process_instance(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        Ok(self.incoming_sequence_flows().len() == 0)
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        false
    }

    fn outgoing_messages_cannot_be_removed(&self) -> bool {
        false
    }

    fn incoming_messages_are_ignored(&self) -> bool {
        false
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        true
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        true
    }
}

impl Transitionable for BPMNExclusiveGateway {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        self.incoming_sequence_flows.len().max(1) * self.outgoing_sequence_flows.len().max(1)
    }

    fn enabled_transitions(
        &self,
        _root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        let mut result = bitvec![0;self.number_of_transitions(sub_marking)];

        let outgoing = self.outgoing_sequence_flows.len().max(1);

        match (
            self.incoming_sequence_flows.len() > 0,
            self.outgoing_sequence_flows.len() > 0,
        ) {
            (true, true) => {
                //join & split
                for (incoming_index, incoming_sequence_flow_index) in
                    self.incoming_sequence_flows.iter().enumerate()
                {
                    if sub_marking.sequence_flow_2_tokens[*incoming_sequence_flow_index] >= 1 {
                        for (outgoing_index, outgoing_sequence_flow_local_index) in
                            self.outgoing_sequence_flows.iter().enumerate()
                        {
                            let outgoing_sequence_flow = parent
                                .sequence_flows_non_recursive()
                                .get(*outgoing_sequence_flow_local_index)
                                .ok_or_else(|| anyhow!("sequence flow not found"))?;
                            if outgoing_sequence_flow.has_fireable_weight() {
                                let transition = incoming_index * outgoing + outgoing_index;
                                result.set(transition, true);
                            }
                        }
                    }
                }
            }
            (true, false) => {
                //join only
                for (incoming_index, incoming_sequence_flow_index) in
                    self.incoming_sequence_flows.iter().enumerate()
                {
                    if sub_marking.sequence_flow_2_tokens[*incoming_sequence_flow_index] >= 1 {
                        result.set(incoming_index, true);
                    }
                }
            }
            (false, true) => {
                //split only; we are in initiation mode 2.
                if sub_marking.element_index_2_tokens[self.local_index] >= 1 {
                    for (outgoing_index, outgoing_sequence_flow_local_index) in
                        self.outgoing_sequence_flows.iter().enumerate()
                    {
                        let outgoing_sequence_flow = parent
                            .sequence_flows_non_recursive()
                            .get(*outgoing_sequence_flow_local_index)
                            .ok_or_else(|| anyhow!("sequence flow not found"))?;
                        if outgoing_sequence_flow.has_fireable_weight() {
                            result.set(outgoing_index, true);
                        }
                    }
                }
            }
            (false, false) => {
                //no flows at all; we are in initiation mode 2.
                if sub_marking.element_index_2_tokens[self.local_index] >= 1 {
                    result.set(0, true);
                }
            }
        };

        Ok(result)
    }

    fn execute_transition(
        &self,
        transition_index: TransitionIndex,
        _root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        let outgoing = self.outgoing_sequence_flows.len().max(1);
        match (
            self.incoming_sequence_flows.len() > 0,
            self.outgoing_sequence_flows.len() > 0,
        ) {
            (true, true) => {
                //join & split

                //consume
                sub_marking.sequence_flow_2_tokens
                    [self.incoming_sequence_flows[transition_index / outgoing]] -= 1;

                //produce
                sub_marking.sequence_flow_2_tokens
                    [self.outgoing_sequence_flows[transition_index % outgoing]] += 1;
            }
            (true, false) => {
                //join only

                //consume
                sub_marking.sequence_flow_2_tokens
                    [self.incoming_sequence_flows[transition_index]] -= 1;
            }
            (false, true) => {
                //split only; we are in initiation mode 2.

                //consume
                sub_marking.element_index_2_tokens[self.local_index] -= 1;

                //produce
                sub_marking.sequence_flow_2_tokens
                    [self.outgoing_sequence_flows[transition_index]] += 1;
            }
            (false, false) => {
                //no flows at all; we are in initiation mode 2.

                //consume
                sub_marking.element_index_2_tokens[self.local_index] -= 1;
            }
        }
        Ok(())
    }

    fn transition_activity(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        None
    }

    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        Some(format!(
            "exclusive gateway `{}`; internal transition {}",
            self.id, transition_index
        ))
    }

    fn transition_probabilistic_penalty(
        &self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        parent: &dyn Processable,
    ) -> Option<Fraction> {
        if self.outgoing_sequence_flows.len() == 0 {
            Some(Fraction::one())
        } else {
            let sum_weight = self
                .outgoing_sequence_flows
                .iter()
                .filter_map(|sequence_flow_index| {
                    parent
                        .sequence_flows_non_recursive()
                        .get(*sequence_flow_index)?
                        .weight
                        .as_ref()
                })
                .sum();
            let outgoing = self.outgoing_sequence_flows.len().max(1);
            match (
                self.incoming_sequence_flows.len() > 0,
                self.outgoing_sequence_flows.len() > 0,
            ) {
                (true, true) => {
                    //join & split
                    let sequence_flow_index =
                        self.outgoing_sequence_flows[transition_index % outgoing];
                    Some(
                        parent
                            .sequence_flows_non_recursive()
                            .get(sequence_flow_index)?
                            .weight
                            .as_ref()?
                            / &sum_weight,
                    )
                }
                (true, false) => {
                    //join only
                    Some(Fraction::one())
                }
                (false, true) => {
                    let sequence_flow_index = self.outgoing_sequence_flows[transition_index];
                    Some(
                        parent
                            .sequence_flows_non_recursive()
                            .get(sequence_flow_index)?
                            .weight
                            .as_ref()?
                            / &sum_weight,
                    )
                }
                (false, false) => {
                    //no flows at all; we are in initiation mode 2.
                    Some(Fraction::one())
                }
            }
        }
    }

    fn transition_2_consumed_tokens(
        &self,
        transition_index: TransitionIndex,
        _root_marking: &BPMNRootMarking,
        _sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<Token>> {
        let outgoing = self.outgoing_sequence_flows.len().max(1);
        match (
            self.incoming_sequence_flows.len() > 0,
            self.outgoing_sequence_flows.len() > 0,
        ) {
            (true, true) => {
                //join & split
                let sequence_flow_local_index = self
                    .incoming_sequence_flows
                    .get(transition_index / outgoing)
                    .and_if_not("Sequence flow not found.")?;
                let sequence_flow = parent
                    .sequence_flows_non_recursive()
                    .get(*sequence_flow_local_index)
                    .and_if_not_error_default()?;
                Ok(vec![Token::SequenceFlow(sequence_flow.global_index)])
            }
            (true, false) => {
                //join only

                //consume
                let sequence_flow_local_index = self
                    .incoming_sequence_flows
                    .get(transition_index)
                    .and_if_not("Sequence flow not found.")?;
                let sequence_flow = parent
                    .sequence_flows_non_recursive()
                    .get(*sequence_flow_local_index)
                    .and_if_not_error_default()?;
                Ok(vec![Token::SequenceFlow(sequence_flow.global_index)])
            }
            (false, true) => {
                //split only; we are in initiation mode 2.
                Ok(vec![Token::Element(self.global_index)])
            }
            (false, false) => {
                //no flows at all; we are in initiation mode 2.
                Ok(vec![Token::Element(self.global_index)])
            }
        }
    }

    fn transition_2_produced_tokens(
        &self,
        transition_index: TransitionIndex,
        _root_marking: &BPMNRootMarking,
        _sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<Token>> {
        if self.outgoing_sequence_flows.len() == 0 {
            Ok(vec![])
        } else {
            let outgoing = self.outgoing_sequence_flows.len().max(1);
            match (
                self.incoming_sequence_flows.len() > 0,
                self.outgoing_sequence_flows.len() > 0,
            ) {
                (true, true) => {
                    //join & split
                    let sequence_flow_index =
                        self.outgoing_sequence_flows[transition_index % outgoing];
                    Ok(vec![Token::SequenceFlow(
                        parent
                            .sequence_flows_non_recursive()
                            .get(sequence_flow_index)
                            .and_if_not_error_default()?
                            .global_index,
                    )])
                }
                (true, false) => {
                    //join only
                    Ok(vec![])
                }
                (false, true) => {
                    let sequence_flow_index = self.outgoing_sequence_flows[transition_index];
                    Ok(vec![Token::SequenceFlow(
                        parent
                            .sequence_flows_non_recursive()
                            .get(sequence_flow_index)
                            .and_if_not_error_default()?
                            .global_index,
                    )])
                }
                (false, false) => {
                    //no flows at all; we are in initiation mode 2.
                    Ok(vec![])
                }
            }
        }
    }
}
