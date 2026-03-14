use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    enabledness_xor_join_only, execute_transition_parallel_split,
    execute_transition_xor_join_consume, number_of_transitions_xor_join_only,
    parser::parser_state::GlobalIndex,
    semantics::{BPMNRootMarking, BPMNSubMarking, TransitionIndex},
    traits::{objectable::BPMNObject, processable::Processable, transitionable::Transitionable},
    transition_2_marked_sequence_flows_concurrent_split,
};
use anyhow::Result;
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;
use ebi_arithmetic::{Fraction, One};

#[derive(Debug, Clone)]
pub struct BPMNCollapsedSubProcess {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
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

    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNCollapsedSubProcess {
    fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn activity(&self) -> Option<Activity> {
        Some(self.activity)
    }

    fn local_index(&self) -> usize {
        self.local_index
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

    fn outgoing_messages_cannot_be_removed(&self) -> bool {
        true
    }

    fn incoming_messages_are_ignored(&self) -> bool {
        true
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        true
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        true
    }
}

impl Transitionable for BPMNCollapsedSubProcess {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        number_of_transitions_xor_join_only!(self)
    }

    fn enabled_transitions(
        &self,
        _root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        //a collapsed sub-process behaves according to the sequence flows
        //messages do not influence enablement
        Ok(enabledness_xor_join_only!(self, sub_marking))
    }

    fn execute_transition(
        &self,
        transition_index: TransitionIndex,
        root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        //consume tokens
        execute_transition_xor_join_consume!(self, sub_marking, transition_index);

        //do not consume messages

        //produce tokens
        execute_transition_parallel_split!(self, sub_marking);

        //produce messages
        for outgoing_message_flow in &self.outgoing_message_flows {
            // A collapsed sub-process can produce arbitrarily many messages; we set one that should never be removed.
            root_marking.message_flow_2_tokens[*outgoing_message_flow] = 1;
        }

        Ok(())
    }

    fn transition_activity(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        Some(self.activity)
    }

    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        Some(format!(
            "collapsed sub-process `{}`; internal transition {}; label `{}`",
            self.id,
            transition_index,
            bpmn.activity_key.deprocess_activity(&self.activity)
        ))
    }

    fn transition_weight(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        _parent: &dyn Processable,
    ) -> Option<Fraction> {
        Some(Fraction::one())
    }

    fn transition_2_marked_sequence_flows<'a>(
        &'a self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        parent: &'a dyn Processable,
    ) -> Option<Vec<GlobalIndex>> {
        transition_2_marked_sequence_flows_concurrent_split!(self, parent)
    }
}
