use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    elements::{task::BPMNTask, timer_intermediate_catch_event::BPMNTimerIntermediateCatchEvent},
    enabledness_xor_join_only, number_of_transitions_xor_join_only,
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
use strum_macros::EnumIs;

#[derive(Debug, Clone)]
pub struct BPMNEventBasedGateway {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNEventBasedGateway {
    fn verify_structural_correctness(
        &self,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        //an event-based gateway must have two+ outgoing sequence flows
        if self.outgoing_sequence_flows.len() < 2 {
            return Err(anyhow!(
                "an event-based gateway must have at least two outgoing sequence flows (standard page 296)"
            ));
        }

        #[derive(EnumIs)]
        enum Configuration {
            Undecided,
            Tasks,
            Events,
        }

        //check the configuration
        let mut configuration = Configuration::Undecided;
        for sequence_flow_index in &self.outgoing_sequence_flows {
            if let Some(sequence_flow) = parent
                .sequence_flows_non_recursive()
                .get(*sequence_flow_index)
            {
                let target = &parent.elements_non_recursive()[sequence_flow.target_index];
                //the target must not have any other incominge sequence flows
                if target.incoming_sequence_flows().len() > 1 {
                    return Err(anyhow!(
                        "element `{}` cannot have other incoming sequence flows besides from its preceding event-based gateway",
                        target.id()
                    ));
                }

                match target {
                    BPMNElement::CollapsedPool(_)
                    | BPMNElement::CollapsedSubProcess(_)
                    | BPMNElement::EndEvent(_)
                    | BPMNElement::EventBasedGateway(_)
                    | BPMNElement::ExclusiveGateway(_)
                    | BPMNElement::ExpandedSubProcess(_)
                    | BPMNElement::InclusiveGateway(_)
                    | BPMNElement::IntermediateCatchEvent(_)
                    | BPMNElement::IntermediateThrowEvent(_)
                    | BPMNElement::MessageEndEvent(_)
                    | BPMNElement::MessageIntermediateThrowEvent(_)
                    | BPMNElement::MessageStartEvent(_)
                    | BPMNElement::ParallelGateway(_)
                    | BPMNElement::Process(_)
                    | BPMNElement::StartEvent(_)
                    | BPMNElement::TimerStartEvent(_) => {
                        return Err(anyhow!(
                            "element `{}` not allowed as a target of a sequence flow from an event-based gateway (standard page 297)",
                            target.id()
                        ));
                    }

                    BPMNElement::MessageIntermediateCatchEvent(_) => {
                        if configuration.is_tasks() {
                            return Err(anyhow!(
                                "after event-based gateway `{}`, cannot combine message intermediate events and receive tasks (standard page 297)",
                                self.id()
                            ));
                        }
                        configuration = Configuration::Events;
                    }

                    BPMNElement::TimerIntermediateCatchEvent(BPMNTimerIntermediateCatchEvent {
                        ..
                    }) => {
                        //always allowed
                    }

                    BPMNElement::Task(BPMNTask {
                        incoming_message_flow,
                        ..
                    }) => {
                        //the task must have an incoming message flow
                        if !incoming_message_flow.is_some() {
                            return Err(anyhow!(
                                "a task after an event-based gateway must have an incoming message flow"
                            ));
                        }

                        if configuration.is_events() {
                            return Err(anyhow!(
                                "after event-based gateway `{}`, cannot combine message intermediate events and receive tasks (standard page 297)",
                                self.id()
                            ));
                        }
                        configuration = Configuration::Tasks;
                    }
                }
            } else {
                return Err(anyhow!("non-existing sequence flow"));
            }
        }
        Ok(())
    }

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
}

impl BPMNObject for BPMNEventBasedGateway {
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

impl Transitionable for BPMNEventBasedGateway {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        number_of_transitions_xor_join_only!(self)
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        Ok(enabledness_xor_join_only!(self, marking))
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
            "event-based gateway `{}`; internal transition {}",
            self.id, transition_index
        ))
    }
}
