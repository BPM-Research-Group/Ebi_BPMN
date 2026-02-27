use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    semantics::{BPMNMarking, TransitionIndex},
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        searchable::Searchable,
        startable::Startable,
        transitionable::Transitionable,
    },
    verify_structural_correctness_initiation_mode,
};
use anyhow::{Result, anyhow};
use bitvec::prelude::BitVec;
use ebi_activity_key::Activity;

#[derive(Clone, Debug)]
pub struct BPMNProcess {
    pub index: usize,
    pub id: String,
    pub elements: Vec<BPMNElement>,
}

impl Searchable for BPMNProcess {
    fn index_2_object(&self, index: usize) -> Option<&dyn BPMNObject> {
        if self.index == index {
            return Some(self);
        }
        self.elements.index_2_object(index)
    }

    fn id_2_pool_and_index(&self, id: &str) -> Option<(Option<usize>, usize)> {
        if self.id == id {
            Some((Some(self.index), self.index))
        } else {
            if let Some((_, index)) = self.elements.id_2_pool_and_index(id) {
                Some((Some(self.index), index))
            } else {
                None
            }
        }
    }

    fn all_elements_ref(&self) -> Vec<&BPMNElement> {
        self.elements.all_elements_ref()
    }

    fn index_2_element(&self, index: usize) -> Option<&BPMNElement> {
        self.elements.index_2_element(index)
    }

    fn index_2_element_mut(&mut self, index: usize) -> Option<&mut BPMNElement> {
        self.elements.index_2_element_mut(index)
    }
}

impl BPMNElementTrait for BPMNProcess {
    fn verify_structural_correctness(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<()> {
        //check children individually
        for element in &self.elements {
            element.verify_structural_correctness(bpmn)?
        }

        //verify initiation and termination
        verify_structural_correctness_initiation_mode!(self, bpmn);

        Ok(())
    }

    fn add_incoming_sequence_flow(&mut self, _flow_index: usize) -> anyhow::Result<()> {
        Err(anyhow!("processes cannot have incoming sequence flows"))
    }

    fn add_outgoing_sequence_flow(&mut self, _flow_index: usize) -> anyhow::Result<()> {
        Err(anyhow!("processes cannot have outgoing sequence flows"))
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> anyhow::Result<()> {
        Err(anyhow!("processes cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> anyhow::Result<()> {
        Err(anyhow!("processes cannot have outgoing message flows"))
    }
}

impl BPMNObject for BPMNProcess {
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
        &EMPTY_FLOWS
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn can_start_process_instance(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        Ok(false)
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        false
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        false
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        false
    }
}

impl Transitionable for BPMNProcess {
    fn number_of_transitions(&self) -> usize {
        self.elements.number_of_transitions()
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNMarking,
        parent_index: Option<usize>,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        self.elements
            .enabled_transitions(marking, parent_index, bpmn)
    }

    fn transition_activity(&self, transition_index: TransitionIndex) -> Option<Activity> {
        self.elements.transition_activity(transition_index)
    }

    fn transition_debug(&self, transition_index: TransitionIndex) -> Option<String> {
        self.elements.transition_debug(transition_index)
    }
}

impl Startable for BPMNProcess {
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
