use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    objects_objectable::{BPMNObject, EMPTY_FLOWS},
    objects_transitionable::Transitionable,
    semantics::BPMNMarking,
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};

#[derive(Debug, Clone)]
pub struct BPMNMessageStartEvent {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) message_marker_id: String,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
    pub(crate) incoming_message_flow: Option<usize>,
}

impl BPMNElementTrait for BPMNMessageStartEvent {
    fn add_incoming_sequence_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "message start events cannot have incoming sequence flows"
        ))
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, flow_index: usize) -> Result<()> {
        if self.incoming_message_flow.is_some() {
            return Err(anyhow!("cannot add a second incoming message flow"));
        }
        self.incoming_message_flow = Some(flow_index);
        Ok(())
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "message start events cannot have outgoing message flows"
        ))
    }

    fn verify_structural_correctness(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        if self.incoming_message_flow.is_none() {
            return Err(anyhow!(
                "a message start event must have an incoming message flow"
            ));
        }
        Ok(())
    }
}

impl BPMNObject for BPMNMessageStartEvent {
    fn index(&self) -> usize {
        self.index
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn incoming_sequence_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &self.outgoing_sequence_flows
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &self.incoming_message_flow.as_slice()
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        false
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        false
    }
}

impl Transitionable for BPMNMessageStartEvent {
    fn number_of_transitions(&self) -> usize {
        1
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> BitVec {
        if let Some(incoming_message_flow) = self.incoming_message_flow {
            //Two cases apply:
            //1) the source of the message always has tokens -> leave enablement to the environment (will put a token on the incoming message flow)
            //2) the source of the message is normal -> normal enablement
            // in both cases, we are enabled if there is a message on the incoming message flow
            if marking.message_flow_2_tokens[incoming_message_flow] == 0 {
                //enabled
                return bitvec![0;1];
            } else {
                //not enabled
                return bitvec![1;1];
            }
        } else {
            //model is not structurally correct
            unreachable!()
        }
    }
}
