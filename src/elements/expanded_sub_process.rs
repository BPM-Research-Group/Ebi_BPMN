use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    semantics::BPMNMarking,
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        startable::Startable,
        transitionable::Transitionable,
    },
    verify_structural_correctness_initiation_mode,
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};

#[derive(Debug, Clone)]
pub struct BPMNExpandedSubProcess {
    pub(crate) index: usize,
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) elements: Vec<BPMNElement>,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}

impl BPMNElementTrait for BPMNExpandedSubProcess {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> anyhow::Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> anyhow::Result<()> {
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "expanded sub-processes cannot have incoming message flows"
        ))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!(
            "expanded sub-processes cannot have outgoing message flows"
        ))
    }

    fn verify_structural_correctness(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        //recurse on elements
        for element in &self.elements {
            element.verify_structural_correctness(bpmn)?
        }

        //verify initiation and termination
        verify_structural_correctness_initiation_mode!(self, bpmn);

        Ok(())
    }
}

impl BPMNObject for BPMNExpandedSubProcess {
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

impl Transitionable for BPMNExpandedSubProcess {
    fn number_of_transitions(&self) -> usize {
        //behaves like an XOR-join to start
        self.incoming_sequence_flows.len().max(1)
        //one transition to end
        + 1
        //and its inner transitions
        + self.elements.number_of_transitions()
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        //start transitions
        let mut result = bitvec![0;self.number_of_transitions()];

        for (transition_index, incoming_sequence_flow) in
            self.incoming_sequence_flows.iter().enumerate()
        {
            if marking.sequence_flow_2_tokens[*incoming_sequence_flow] >= 1 {
                result.set(transition_index, true);
            }
        }

        let children_enabled_transitions = self.elements.enabled_transitions(marking, bpmn)?;

        //end transition
        if children_enabled_transitions.not_any() {
            //child has no enabled transitions -> terminated
            result.set(self.incoming_sequence_flows.len(), true);
        }

        //recurse
        result.extend(children_enabled_transitions);

        Ok(result)
    }
}

impl Startable for BPMNExpandedSubProcess {
    fn unconstrained_start_events_without_recursing(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<&BPMNElement>> {
        self.elements
            .unconstrained_start_events_without_recursing(bpmn)
    }

    fn end_events_without_recursing(&self) -> Vec<&BPMNElement> {
        self.elements.end_events_without_recursing()
    }

    fn start_elements_without_recursing(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<&BPMNElement>> {
        self.elements.start_elements_without_recursing(bpmn)
    }
}
