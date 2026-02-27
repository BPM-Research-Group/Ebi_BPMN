use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    parser::parser_state::GlobalIndex,
    semantics::{BPMNMarking, BPMNSubMarking, TransitionIndex},
    sequence_flow::BPMNSequenceFlow,
    to_sub_marking,
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        searchable::Searchable,
        startable::{InitiationMode, Startable},
        transitionable::Transitionable,
    },
    verify_structural_correctness_initiation_mode,
};
use anyhow::{Result, anyhow};
use bitvec::prelude::BitVec;
use ebi_activity_key::Activity;

/// more common name: pool
#[derive(Clone, Debug)]
pub struct BPMNProcess {
    pub global_index: GlobalIndex,
    pub id: String,
    pub local_index: usize,
    pub elements: Vec<BPMNElement>,
    pub sequence_flows: Vec<BPMNSequenceFlow>,
}

impl Searchable for BPMNProcess {
    fn index_2_object(&self, index: GlobalIndex) -> Option<&dyn BPMNObject> {
        if self.global_index == index {
            return Some(self);
        }
        self.elements.index_2_object(index)
    }

    fn id_2_pool_and_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)> {
        if self.id == id {
            Some((Some(self.local_index), self.global_index))
        } else {
            if let Some((_, index)) = self.elements.id_2_pool_and_index(id) {
                Some((Some(self.local_index), index))
            } else {
                None
            }
        }
    }

    fn all_elements_ref(&self) -> Vec<&BPMNElement> {
        self.elements.all_elements_ref()
    }

    fn index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement> {
        self.elements.index_2_element(index)
    }

    fn index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement> {
        self.elements.index_2_element_mut(index)
    }
}

impl BPMNElementTrait for BPMNProcess {
    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        //check children individually
        for element in &self.elements {
            element.verify_structural_correctness(self, bpmn)?
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
    fn number_of_transitions(&self, marking: &BPMNSubMarking) -> usize {
        self.elements.number_of_transitions(marking)
    }

    fn enabled_transitions(
        &self,
        marking: &BPMNSubMarking,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        self.elements.enabled_transitions(marking, parent, bpmn)
    }

    fn transition_activity(
        &self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        self.elements.transition_activity(transition_index, marking)
    }

    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
    ) -> Option<String> {
        self.elements.transition_debug(transition_index, marking)
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

impl Processable for BPMNProcess {
    fn elements_non_recursive(&self) -> &Vec<BPMNElement> {
        &self.elements
    }

    fn sequence_flows_non_recursive(&self) -> &Vec<BPMNSequenceFlow> {
        &self.sequence_flows
    }

    fn to_sub_marking(
        &self,
        initiation_mode: InitiationMode,
        root_marking: Rc<BPMNMarking>,
    ) -> Result<BPMNSubMarking> {
        to_sub_marking!(self, initiation_mode, root_marking)
    }
}
