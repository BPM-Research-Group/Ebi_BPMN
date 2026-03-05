use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    elements::{
        event_based_gateway::BPMNEventBasedGateway, exclusive_gateway::BPMNExclusiveGateway,
        inclusive_gateway::BPMNInclusiveGateway,
    },
    stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
    traits::objectable::BPMNObject,
};
use anyhow::{Context, Result, anyhow};
use ebi_arithmetic::Signed;

impl BusinessProcessModelAndNotation {
    pub fn is_structurally_correct(&self) -> Result<()> {
        //check elements
        for element in self.elements() {
            element
                .verify_structural_correctness(self, self)
                .with_context(|| anyhow!("element `{}`", element.id()))?;
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
        self.bpmn
            .is_structurally_correct()
            .with_context(|| anyhow!("Checking structural correctness of control flow."))?;

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
                            if !weight.is_positive() {
                                return Err(anyhow!(
                                    "Sequence flow `{}` has a non-positive weight.",
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

        Ok(())
    }
}

#[macro_export]
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
                    "process `{}` has start events but no end events",
                    $process.id
                ));
            }

            //all elements must have incoming and outgoing arcs
            for element in &$process.elements {
                if element.can_have_incoming_sequence_flows() {
                    if element.incoming_sequence_flows().is_empty() {
                        return Err(anyhow!(
                            "given that there are start events in process `{}`, element `{}` should have an incoming sequence flow",
                            $process.id,
                            element.id()
                        ));
                    }
                }
                if element.can_have_outgoing_sequence_flows() {
                    if element.outgoing_sequence_flows().is_empty() {
                        return Err(anyhow!(
                            "given that there are start events in process `{}`, element `{}` should have an outgoing sequence flow",
                            $process.id,
                            element.id()
                        ));
                    }
                }
            }
        }
    };
}
