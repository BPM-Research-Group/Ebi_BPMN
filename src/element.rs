use crate::{
    BusinessProcessModelAndNotation,
    elements::{
        collapsed_pool::BPMNCollapsedPool, collapsed_sub_process::BPMNCollapsedSubProcess,
        end_event::BPMNEndEvent, event_based_gateway::BPMNEventBasedGateway,
        exclusive_gateway::BPMNExclusiveGateway, expanded_sub_process::BPMNExpandedSubProcess,
        inclusive_gateway::BPMNInclusiveGateway,
        intermediate_catch_event::BPMNIntermediateCatchEvent,
        intermediate_throw_event::BPMNIntermediateThrowEvent, manual_task::BPMNManualTask,
        message_end_event::BPMNMessageEndEvent,
        message_intermediate_catch_event::BPMNMessageIntermediateCatchEvent,
        message_intermediate_throw_event::BPMNMessageIntermediateThrowEvent,
        message_start_event::BPMNMessageStartEvent, parallel_gateway::BPMNParallelGateway,
        process::BPMNProcess, receive_task::BPMNReceiveTask, start_event::BPMNStartEvent,
        task::BPMNTask, timer_intermediate_catch_event::BPMNTimerIntermediateCatchEvent,
        timer_start_event::BPMNTimerStartEvent, user_task::BPMNUserTask,
    },
    parser::parser_state::GlobalIndex,
    semantics::{BPMNRootMarking, BPMNSubMarking, TransitionIndex},
    sequence_flow::BPMNSequenceFlow,
    traits::{
        objectable::BPMNObject, processable::Processable, searchable::Searchable,
        transitionable::Transitionable, writable::Writable,
    },
};
use anyhow::{Ok, Result};
use bitvec::vec::BitVec;
use ebi_activity_key::Activity;
use quick_xml::Writer;
use strum_macros::EnumIs;

#[derive(Clone, Debug, EnumIs)]
pub enum BPMNElement {
    CollapsedPool(BPMNCollapsedPool),
    CollapsedSubProcess(BPMNCollapsedSubProcess),
    EndEvent(BPMNEndEvent),
    EventBasedGateway(BPMNEventBasedGateway),
    ExclusiveGateway(BPMNExclusiveGateway),
    ExpandedSubProcess(BPMNExpandedSubProcess),
    InclusiveGateway(BPMNInclusiveGateway),
    IntermediateCatchEvent(BPMNIntermediateCatchEvent),
    IntermediateThrowEvent(BPMNIntermediateThrowEvent),
    ManualTask(BPMNManualTask),
    MessageEndEvent(BPMNMessageEndEvent),
    MessageIntermediateCatchEvent(BPMNMessageIntermediateCatchEvent),
    MessageIntermediateThrowEvent(BPMNMessageIntermediateThrowEvent),
    MessageStartEvent(BPMNMessageStartEvent),
    ParallelGateway(BPMNParallelGateway),
    Process(BPMNProcess),
    ReceiveTask(BPMNReceiveTask),
    StartEvent(BPMNStartEvent),
    Task(BPMNTask),
    TimerIntermediateCatchEvent(BPMNTimerIntermediateCatchEvent),
    TimerStartEvent(BPMNTimerStartEvent),
    UserTask(BPMNUserTask),
}

pub trait BPMNElementTrait {
    ///verify that structural requirements specific to this element are fulfilled
    fn verify_structural_correctness(
        &self,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()>;

    ///Add an incoming sequence flow to the element. Returns whether successful.
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()>;

    ///Add an outgoing sequence flow to the element. Returns whether successful.
    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()>;

    ///Add an incoming message flow to the element. Returns whether successful.
    fn add_incoming_message_flow(&mut self, flow_index: usize) -> Result<()>;

    ///Add an outgoing message flow to the element. Returns whether successful.
    fn add_outgoing_message_flow(&mut self, flow_index: usize) -> Result<()>;
}

macro_rules! enums {
    ($self:ident, $fn:ident, $($v:ident),*) => {
        match $self {
            BPMNElement::CollapsedPool(x) => BPMNCollapsedPool::$fn(x, $($v),*),
            BPMNElement::CollapsedSubProcess(x) => BPMNCollapsedSubProcess::$fn(x, $($v),*),
            BPMNElement::EndEvent(x) => BPMNEndEvent::$fn(x, $($v),*),
            BPMNElement::EventBasedGateway(x) => BPMNEventBasedGateway::$fn(x, $($v),*),
            BPMNElement::ExclusiveGateway(x) => BPMNExclusiveGateway::$fn(x, $($v),*),
            BPMNElement::ExpandedSubProcess(x) => BPMNExpandedSubProcess::$fn(x, $($v),*),
            BPMNElement::InclusiveGateway(x) => BPMNInclusiveGateway::$fn(x, $($v),*),
            BPMNElement::IntermediateCatchEvent(x) => BPMNIntermediateCatchEvent::$fn(x, $($v),*),
            BPMNElement::IntermediateThrowEvent(x) => BPMNIntermediateThrowEvent::$fn(x, $($v),*),
            BPMNElement::ManualTask(x) => BPMNManualTask::$fn(x, $($v),*),
            BPMNElement::MessageEndEvent(x) => BPMNMessageEndEvent::$fn(x, $($v),*),
            BPMNElement::MessageIntermediateCatchEvent(x) => {
                BPMNMessageIntermediateCatchEvent::$fn(x, $($v),*)
            }
            BPMNElement::MessageIntermediateThrowEvent(x) => {
                BPMNMessageIntermediateThrowEvent::$fn(x, $($v),*)
            }
            BPMNElement::MessageStartEvent(x) => BPMNMessageStartEvent::$fn(x, $($v),*),
            BPMNElement::ParallelGateway(x) => BPMNParallelGateway::$fn(x, $($v),*),
            BPMNElement::Process(x) => BPMNProcess::$fn(x, $($v),*),
            BPMNElement::ReceiveTask(x) => BPMNReceiveTask::$fn(x, $($v),*),
            BPMNElement::StartEvent(x) => BPMNStartEvent::$fn(x, $($v),*),
            BPMNElement::Task(x) => BPMNTask::$fn(x, $($v),*),
            BPMNElement::TimerIntermediateCatchEvent(x) => BPMNTimerIntermediateCatchEvent::$fn(x, $($v),*),
            BPMNElement::TimerStartEvent(x) => BPMNTimerStartEvent::$fn(x, $($v),*),
            BPMNElement::UserTask(x) => BPMNUserTask::$fn(x, $($v),*)
        }
    };
}

impl BPMNElementTrait for BPMNElement {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        enums!(self, add_incoming_sequence_flow, flow_index)
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        enums!(self, add_outgoing_sequence_flow, flow_index)
    }

    fn add_incoming_message_flow(&mut self, flow_index: usize) -> Result<()> {
        enums!(self, add_incoming_message_flow, flow_index)
    }

    fn add_outgoing_message_flow(&mut self, flow_index: usize) -> Result<()> {
        enums!(self, add_outgoing_message_flow, flow_index)
    }

    fn verify_structural_correctness(
        &self,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        enums!(self, verify_structural_correctness, parent, bpmn)
    }
}

impl Searchable for BPMNElement {
    fn id_2_pool_and_global_index(&self, search_id: &str) -> Option<(Option<usize>, GlobalIndex)> {
        if self.id() == search_id && self.is_collapsed_pool() {
            Some((Some(self.local_index()), self.global_index()))
        } else if self.id() == search_id {
            Some((None, self.global_index()))
        } else if let BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })
        | BPMNElement::Process(BPMNProcess { elements, .. }) = self
        {
            if let Some((_, index)) = elements.id_2_pool_and_global_index(search_id) {
                Some((Some(self.local_index()), index))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn id_2_local_index(&self, id: &str) -> Option<usize> {
        if let BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })
        | BPMNElement::Process(BPMNProcess { elements, .. }) = self
        {
            elements.id_2_local_index(id)
        } else {
            None
        }
    }

    fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, &dyn Processable)> {
        if let BPMNElement::Process(process) = self {
            process.global_index_2_sequence_flow_and_parent(sequence_flow_global_index)
        } else if let BPMNElement::ExpandedSubProcess(process) = self {
            process.global_index_2_sequence_flow_and_parent(sequence_flow_global_index)
        } else {
            None
        }
    }

    fn all_elements_ref(&self) -> Vec<&BPMNElement> {
        if let BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })
        | BPMNElement::Process(BPMNProcess { elements, .. }) = self
        {
            let mut result = elements
                .iter()
                .map(|element| element.all_elements_ref())
                .flatten()
                .collect::<Vec<_>>();
            result.push(self);
            result
        } else {
            vec![self]
        }
    }

    fn parent_of(&self, global_index: GlobalIndex) -> (Option<&dyn Processable>, bool) {
        if let BPMNElement::Process(process) = self {
            process.parent_of(global_index)
        } else if let BPMNElement::ExpandedSubProcess(process) = self {
            process.parent_of(global_index)
        } else if self.global_index() == global_index {
            (None, true)
        } else {
            (None, false)
        }
    }

    fn all_sequence_flows_ref(&self) -> Vec<&BPMNSequenceFlow> {
        match self {
            BPMNElement::ExpandedSubProcess(p) => p.all_sequence_flows_ref(),
            BPMNElement::Process(p) => p.all_sequence_flows_ref(),
            _ => vec![],
        }
    }

    fn global_index_2_sequence_flow_mut(
        &mut self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<&mut BPMNSequenceFlow> {
        match self {
            BPMNElement::ExpandedSubProcess(p) => {
                p.global_index_2_sequence_flow_mut(sequence_flow_global_index)
            }
            BPMNElement::Process(p) => {
                p.global_index_2_sequence_flow_mut(sequence_flow_global_index)
            }
            _ => None,
        }
    }

    fn global_index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement> {
        if self.global_index() == index {
            Some(self)
        } else if let BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })
        | BPMNElement::Process(BPMNProcess { elements, .. }) = self
        {
            elements.global_index_2_element(index)
        } else {
            None
        }
    }

    fn global_index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement> {
        if self.global_index() == index {
            Some(self)
        } else if let BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })
        | BPMNElement::Process(BPMNProcess { elements, .. }) = self
        {
            elements.global_index_2_element_mut(index)
        } else {
            None
        }
    }

    fn local_index_2_element_mut(&mut self, index: usize) -> Option<&mut BPMNElement> {
        if let BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })
        | BPMNElement::Process(BPMNProcess { elements, .. }) = self
        {
            elements.local_index_2_element_mut(index)
        } else {
            None
        }
    }
}

impl Transitionable for BPMNElement {
    fn number_of_transitions(&self, marking: &BPMNSubMarking) -> usize {
        enums!(self, number_of_transitions, marking)
    }

    fn enabled_transitions(
        &self,
        root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        enums!(
            self,
            enabled_transitions,
            root_marking,
            sub_marking,
            parent,
            bpmn
        )
    }

    fn execute_transition(
        &self,
        transition_index: TransitionIndex,
        root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        enums!(
            self,
            execute_transition,
            transition_index,
            root_marking,
            sub_marking,
            parent,
            bpmn
        )
    }

    fn transition_activity(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        enums!(self, transition_activity, transition_index, marking)
    }

    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        enums!(self, transition_debug, transition_index, marking, bpmn)
    }

    fn transition_weight(
        &self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        parent: &dyn Processable,
    ) -> Option<ebi_arithmetic::Fraction> {
        enums!(self, transition_weight, transition_index, marking, parent)
    }

    fn transition_2_marked_sequence_flows<'a>(
        &'a self,
        transition_index: TransitionIndex,
        marking: &BPMNSubMarking,
        parent: &'a dyn Processable,
    ) -> Option<Vec<GlobalIndex>> {
        enums!(
            self,
            transition_2_marked_sequence_flows,
            transition_index,
            marking,
            parent
        )
    }
}

impl Writable for BPMNElement {
    fn write<W: std::io::Write>(
        &self,
        x: &mut Writer<W>,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        enums!(self, write, x, parent, bpmn)?;

        Ok(())
    }
}

impl BPMNObject for BPMNElement {
    fn global_index(&self) -> GlobalIndex {
        enums!(self, global_index,)
    }

    fn id(&self) -> &str {
        enums!(self, id,)
    }

    fn activity(&self) -> Option<Activity> {
        enums!(self, activity,)
    }

    fn local_index(&self) -> usize {
        enums!(self, local_index,)
    }

    fn is_unconstrained_start_event(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        enums!(self, is_unconstrained_start_event, bpmn)
    }

    fn is_end_event(&self) -> bool {
        enums!(self, is_end_event,)
    }

    fn incoming_sequence_flows(&self) -> &[usize] {
        enums!(self, incoming_sequence_flows,)
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        enums!(self, outgoing_sequence_flows,)
    }

    fn incoming_message_flows(&self) -> &[usize] {
        enums!(self, incoming_message_flows,)
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        enums!(self, outgoing_message_flows,)
    }

    fn can_start_process_instance(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        enums!(self, can_start_process_instance, bpmn)
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        enums!(self, outgoing_message_flows_always_have_tokens,)
    }

    fn outgoing_messages_cannot_be_removed(&self) -> bool {
        enums!(self, outgoing_messages_cannot_be_removed,)
    }

    fn incoming_messages_are_ignored(&self) -> bool {
        enums!(self, incoming_messages_are_ignored,)
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        enums!(self, can_have_incoming_sequence_flows,)
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        enums!(self, can_have_outgoing_sequence_flows,)
    }
}
