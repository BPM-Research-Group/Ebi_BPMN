use crate::{
    BusinessProcessModelAndNotation, element::BPMNElementTrait, objects_objectable::BPMNObject,
    objects_transitionable::Transitionable, semantics::BPMNMarking,
};
use anyhow::Result;
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;

#[derive(Debug, Clone)]
pub struct BPMNCollapsedSubProcess {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) activity: Activity,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
    pub(crate) incoming_message_flows: Vec<usize>,
    pub(crate) outgoing_message_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNCollapsedSubProcess {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_message_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_message_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_message_flows.push(flow_index);
        Ok(())
    }

    fn verify_structural_correctness(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNCollapsedSubProcess {
    fn index(&self) -> usize {
        self.index
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn incoming_sequence_flows(&self) -> &[usize] {
        &self.incoming_sequence_flows
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &self.outgoing_sequence_flows
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &self.incoming_message_flows
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &self.outgoing_message_flows
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        true
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        false
    }
}

impl Transitionable for BPMNCollapsedSubProcess {
    fn number_of_transitions(&self) -> usize {
        self.incoming_sequence_flows.len()
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> BitVec {
        //a collapsed sub-process behaves according to the sequence flows
        //messages do not influence enablement
        let mut result = bitvec![0;self.incoming_sequence_flows.len()];

        for (transition_index, incoming_sequence_flow) in
            self.incoming_sequence_flows.iter().enumerate()
        {
            if marking.sequence_flow_2_tokens[*incoming_sequence_flow] >= 1 {
                result.set(transition_index, true);
            }
        }

        result
    }
}
