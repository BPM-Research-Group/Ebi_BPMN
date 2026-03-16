use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    elements::{
        event_based_gateway::BPMNEventBasedGateway, exclusive_gateway::BPMNExclusiveGateway,
        inclusive_gateway::BPMNInclusiveGateway,
    },
    stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
    traits::{objectable::BPMNObject, startable::Startable},
};
use anyhow::{Context, Result, anyhow};
use ebi_arithmetic::Signed;

impl BusinessProcessModelAndNotation {
    /// Verify whether the model is structurally correct using several, though not exhaustive, checks.
    /// If the BPMN model is imported by [import_from_reader] or created using a [BPMNCreator], there is no need to call this method.
    /// 
    /// [import_from_reader]: BusinessProcessModelAndNotation::import_from_reader
    /// [BPMNCreator]: crate::BPMNCreator
    pub fn is_structurally_correct(&self) -> Result<()> {
        //check elements
        for element in &self.elements {
            element
                .verify_structural_correctness(self, self)
                .with_context(|| {
                    anyhow!(
                        "Found a structural correctness issue with element `{}`.",
                        element.id()
                    )
                })?;
        }

        //check messages
        for message_flow in &self.message_flows {
            //each message must connect different pools
            if message_flow.source_pool_index == message_flow.target_pool_index {
                return Err(anyhow!(
                    "message flow with id `{}` is intra-pool",
                    message_flow.id
                ));
            }
        }

        Ok(())
    }
}

impl StochasticBusinessProcessModelAndNotation {
    pub fn is_structurally_correct(&self) -> Result<()> {
        //check the bpmn itself
        self.bpmn
            .is_structurally_correct()
            .with_context(|| anyhow!("Checking structural correctness of control flow."))?;

        //we cannot handle models with multiple start events
        {
            //gather the start elements
            let mut start_elements = vec![];
            for element in &self.bpmn.elements {
                if let BPMNElement::Process(process) = element {
                    start_elements
                        .extend(process.unconstrained_start_events_without_recursing(&self.bpmn));
                }
            }
            if start_elements.len() > 1 {
                return Err(anyhow!("An SBPMN model can have at most one start event."));
            }
        }

        //choice-based sequence flows must have weights
        {
            for element in self.bpmn.elements() {
                if element.is_event_based_gateway()
                    || element.is_exclusive_gateway()
                    || element.is_inclusive_gateway()
                {
                    if element.outgoing_sequence_flows().len() > 1 {
                        for sequence_flow_index in element.outgoing_sequence_flows() {
                            let parent = self
                                .bpmn
                                .parent_of(element.global_index())
                                .ok_or_else(|| anyhow!("parent not found"))?;

                            let sequence_flow = parent
                                .sequence_flows_non_recursive()
                                .get(*sequence_flow_index)
                                .ok_or_else(|| anyhow!("sequence flow not found"))?;

                            if sequence_flow.weight.is_none() {
                                return Err(anyhow!(
                                    "Sequence flow `{}` originates from a choice-making gateway and therefore must have a weight.",
                                    sequence_flow.id
                                ));
                            }
                        }
                    }
                }
            }
        }

        //we cannot handle models with steered event-based gateways
        {
            for element in self.bpmn.elements() {
                if element.is_event_based_gateway() {
                    let parent = self
                        .bpmn
                        .parent_of(element.global_index())
                        .ok_or_else(|| anyhow!("parent not found"))?;

                    for sequence_flow_index in element.outgoing_sequence_flows() {
                        let target = parent.sequence_flow_index_2_target(*sequence_flow_index)?;
                        if let Some(message_flow_index) = element.incoming_message_flows().get(0) {
                            let message_source =
                                self.bpmn.message_flow_index_2_source(*message_flow_index)?;
                            if message_source.outgoing_message_flows_always_have_tokens() {
                                //a message from a collapsed pool is always there
                                //no problem
                            } else {
                                //otherwise, the message must be there
                                return Err(anyhow!(
                                    "Event-based gateway `{}` has an outgoing sequence flow to element `{}`, which depends on an uncertain message. This is not supported.",
                                    element.id(),
                                    target.id()
                                ));
                            }
                        } else {
                            //there is no constraining message, so this message start event can start a process instance
                            //no problem
                        }
                    }
                }
            }
        }

        //check that outgoing sequence flows have weights
        for element in self.bpmn.elements() {
            match element {
                BPMNElement::CollapsedPool(_)
                | BPMNElement::CollapsedSubProcess(_)
                | BPMNElement::EndEvent(_)
                | BPMNElement::ExpandedSubProcess(_)
                | BPMNElement::IntermediateCatchEvent(_)
                | BPMNElement::IntermediateThrowEvent(_)
                | BPMNElement::MessageEndEvent(_)
                | BPMNElement::MessageIntermediateCatchEvent(_)
                | BPMNElement::MessageIntermediateThrowEvent(_)
                | BPMNElement::MessageStartEvent(_)
                | BPMNElement::ParallelGateway(_)
                | BPMNElement::Process(_)
                | BPMNElement::ReceiveTask(_)
                | BPMNElement::StartEvent(_)
                | BPMNElement::Task(_)
                | BPMNElement::TimerIntermediateCatchEvent(_)
                | BPMNElement::TimerStartEvent(_) => {}
                BPMNElement::EventBasedGateway(BPMNEventBasedGateway {
                    outgoing_sequence_flows,
                    ..
                })
                | BPMNElement::ExclusiveGateway(BPMNExclusiveGateway {
                    outgoing_sequence_flows,
                    ..
                })
                | BPMNElement::InclusiveGateway(BPMNInclusiveGateway {
                    outgoing_sequence_flows,
                    ..
                }) => {
                    if outgoing_sequence_flows.len() > 1 {
                        //only splits matter
                        let parent = self
                            .bpmn
                            .parent_of(element.global_index())
                            .ok_or_else(|| anyhow!("parent not found"))?;
                        for sequence_flow_local_index in outgoing_sequence_flows {
                            let sequence_flow = parent
                                .sequence_flows_non_recursive()
                                .get(*sequence_flow_local_index)
                                .ok_or_else(|| anyhow!("sequence flow not found"))?;

                            if let Some(weight) = &sequence_flow.weight {
                                if weight.is_negative() {
                                    return Err(anyhow!(
                                        "Sequence flow `{}` has a negative weight.",
                                        sequence_flow.id
                                    ));
                                }
                            } else {
                                return Err(anyhow!(
                                    "Sequence flow `{}` does not have a weight. It should have a weight as it is an outgoing sequence flow of a gateway that makes a choice.",
                                    sequence_flow.id
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

macro_rules! verify_structural_correctness_initiation_mode {
    ($process:ident, $bpmn:ident) => {
        //verify initiation and termination
        if $process
            .initiation_mode($bpmn)?
            .is_choice_between_start_events()
        {
            //there must be end events
            if $process.end_events_without_recursing().is_empty() {
                return Err(anyhow!(
                    "Process `{}` has start events but no end events.",
                    $process.id
                ));
            }

            //all elements must have incoming and outgoing arcs
            for element in &$process.elements {
                if element.can_have_incoming_sequence_flows() {
                    if element.incoming_sequence_flows().is_empty() {
                        return Err(anyhow!(
                            "Given that there are start events in process `{}`, element `{}` should have an incoming sequence flow.",
                            $process.id,
                            element.id()
                        ));
                    }
                }
                if element.can_have_outgoing_sequence_flows() {
                    if element.outgoing_sequence_flows().is_empty() {
                        return Err(anyhow!(
                            "Given that there are start events in process `{}`, element `{}` should have an outgoing sequence flow.",
                            $process.id,
                            element.id()
                        ));
                    }
                }
            }
        }
    };
}
pub(crate) use verify_structural_correctness_initiation_mode;
