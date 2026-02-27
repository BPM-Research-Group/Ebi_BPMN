use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    enabledness_xor_join_only, number_of_transitions_xor_join_only,
    semantics::{BPMNMarking, TransitionIndex},
    traits::{objectable::BPMNObject, transitionable::Transitionable},
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
        &self.incoming_message_flows
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &self.outgoing_message_flows
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

impl Transitionable for BPMNCollapsedSubProcess {
    fn number_of_transitions(&self) -> usize {
        number_of_transitions_xor_join_only!(self)
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        _parent_index: Option<usize>,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        //a collapsed sub-process behaves according to the sequence flows
        //messages do not influence enablement
        Ok(enabledness_xor_join_only!(self, marking))
    }

    fn transition_activity(&self, _transition_index: TransitionIndex) -> Option<Activity> {
        Some(self.activity)
    }

    fn transition_debug(&self, transition_index: TransitionIndex) -> Option<String> {
        Some(format!(
            "collapsed sub-process `{}`; internal transition {}",
            self.id,
            transition_index
        ))
    }
}
