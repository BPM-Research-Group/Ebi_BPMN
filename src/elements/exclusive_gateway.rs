use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    parser::parser_state::GlobalIndex,
    semantics::{BPMNSubMarking, TransitionIndex},
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

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
        marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        let mut result = bitvec![0;self.number_of_transitions(marking)];

        let outgoing = self.outgoing_sequence_flows.len().max(1);

        match (
            self.incoming_sequence_flows.len() > 0,
            self.outgoing_sequence_flows.len() > 0,
        ) {
            (true, true) => {
                //join & split
                for (incoming_index, incoming_sequence_flow) in
                    self.incoming_sequence_flows.iter().enumerate()
                {
                    if marking.sequence_flow_2_tokens[*incoming_sequence_flow] >= 1 {
                        for i in incoming_index * outgoing..(1 + incoming_index) * outgoing {
                            result.set(i, true);
                        }
                    }
                }
            }
            (true, false) => {
                //join only
                for (incoming_index, incoming_sequence_flow) in
                    self.incoming_sequence_flows.iter().enumerate()
                {
                    if marking.sequence_flow_2_tokens[*incoming_sequence_flow] >= 1 {
                        result.set(incoming_index, true);
                    }
                }
            }
            (false, true) => {
                //split only; we are in initiation mode 2.
                if marking.element_index_2_tokens[self.local_index] >= 1 {
                    result.fill(true);
                }
            }
            (false, false) => {
                //no flows at all; we are in initiation mode 2.
                if marking.element_index_2_tokens[self.local_index] >= 1 {
                    result.set(0, true);
                }
            }
        };

        Ok(result)
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
    ) -> Option<String> {
        Some(format!(
            "exclusive gateway `{}`; internal transition {}",
            self.id, transition_index
        ))
    }
}
