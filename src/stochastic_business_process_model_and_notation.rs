use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    parser::{parser::NAMESPACE_SBPMN, parser_state::GlobalIndex},
    semantics::BPMNMarking,
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, processable::Processable},
};
use anyhow::{Error, Result, anyhow};
#[cfg(any(test, feature = "testactivities"))]
use ebi_activity_key::TestActivityKey;
use ebi_activity_key::{ActivityKey, HasActivityKey, TranslateActivityKey};
use std::{
    fmt::{Display, Formatter},
    io::BufRead,
    str::FromStr,
};

#[derive(Clone, Debug)]
pub struct StochasticBusinessProcessModelAndNotation {
    pub bpmn: BusinessProcessModelAndNotation,
}

impl StochasticBusinessProcessModelAndNotation {
    pub fn import_from_reader(reader: &mut dyn BufRead) -> Result<Self>
    where
        Self: Sized,
    {
        let bpmn = BusinessProcessModelAndNotation::import_from_reader(reader, false)?;
        if !bpmn.stochastic_namespace {
            return Err(anyhow!(
                "The SBPMN namespace of `{}` must be declared on the definitions tag.",
                String::from_utf8_lossy(NAMESPACE_SBPMN)
            ));
        }
        let sbpmn = Self { bpmn };
        sbpmn.is_structurally_correct()?;
        Ok(sbpmn)
    }

    pub fn number_of_elements(&self) -> usize {
        self.bpmn.number_of_elements()
    }

    pub fn number_of_message_flows(&self) -> usize {
        self.bpmn.number_of_message_flows()
    }

    /// returns all elements in the model (recursively)
    pub fn elements(&self) -> Vec<&BPMNElement> {
        self.bpmn.elements()
    }

    pub fn parent_of(&self, global_index: GlobalIndex) -> Option<&dyn Processable> {
        self.bpmn.parent_of(global_index)
    }

    /// Returns all sequence flows (recursive)
    pub fn sequence_flows(&self) -> Vec<&BPMNSequenceFlow> {
        self.bpmn.sequence_flows()
    }

    /// find an element with the given index
    pub fn global_index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement> {
        self.bpmn.global_index_2_element(index)
    }

    /// find an element with the given index
    pub fn global_index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement> {
        self.bpmn.global_index_2_element_mut(index)
    }

    /// find the object of the given index
    pub fn index_2_object(&self, index: GlobalIndex) -> Option<&dyn BPMNObject> {
        self.bpmn.index_2_object(index)
    }

    /// return the element that is the source of the given message flow
    pub fn message_flow_index_2_source(&self, message_flow_index: usize) -> Result<&BPMNElement> {
        self.bpmn.message_flow_index_2_source(message_flow_index)
    }

    /// return the element that is the target of the given message flow
    pub fn message_flow_index_2_target(&self, message_flow_index: usize) -> Result<&BPMNElement> {
        self.bpmn.message_flow_index_2_target(message_flow_index)
    }

    /// return the sequence flow with the given global index
    pub fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, &dyn Processable)> {
        self.bpmn
            .global_index_2_sequence_flow_and_parent(sequence_flow_global_index)
    }

    pub fn transition_debug(
        &self,
        transition_index: usize,
        marking: &BPMNMarking,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        self.bpmn.transition_debug(transition_index, marking, bpmn)
    }
}

impl HasActivityKey for StochasticBusinessProcessModelAndNotation {
    fn activity_key(&self) -> &ActivityKey {
        &self.bpmn.activity_key
    }

    fn activity_key_mut(&mut self) -> &mut ActivityKey {
        &mut self.bpmn.activity_key
    }
}

impl FromStr for StochasticBusinessProcessModelAndNotation {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut reader = std::io::Cursor::new(s);
        Self::import_from_reader(&mut reader)
    }
}

impl Display for StochasticBusinessProcessModelAndNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SBPMN model with {} elements", self.number_of_elements())
    }
}

impl TranslateActivityKey for StochasticBusinessProcessModelAndNotation {
    fn translate_using_activity_key(&mut self, to_activity_key: &mut ActivityKey) {
        self.bpmn.translate_using_activity_key(to_activity_key);
    }
}

#[cfg(any(test, feature = "testactivities"))]
impl TestActivityKey for StochasticBusinessProcessModelAndNotation {
    fn test_activity_key(&self) {
        self.bpmn.test_activity_key()
    }
}
