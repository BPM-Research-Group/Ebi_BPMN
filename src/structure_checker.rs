use crate::{
    BusinessProcessModelAndNotation, element::BPMNElementTrait, objects_objectable::BPMNObject,
};
use anyhow::{Context, Result, anyhow};

impl BusinessProcessModelAndNotation {
    pub fn is_structurally_correct(&self) -> Result<()> {
        //check elements
        for element in self.all_elements_ref() {
            element
                .verify_structural_correctness(self)
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
