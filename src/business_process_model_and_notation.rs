use crate::{
    element::BPMNElement,
    elements::{collapsed_sub_process::BPMNCollapsedSubProcess, task::BPMNTask},
    message_flow::BPMNMessageFlow,
    parser::parser_state::GlobalIndex,
    sequence_flow::BPMNSequenceFlow,
    traits::{
        objectable::BPMNObject, processable::Processable, searchable::Searchable,
    },
};
use anyhow::{Result, anyhow};
#[cfg(any(test, feature = "testactivities"))]
use ebi_activity_key::TestActivityKey;
use ebi_activity_key::{ActivityKey, ActivityKeyTranslator, TranslateActivityKey};
use ebi_derive::ActivityKey;
use std::fmt::{Display, Formatter};

/// A struct with a Business Process Model and Notation (BPMN) model.
///
///Example:
///  ```
///  use std::io::prelude::*;
///  use std::io::BufReader;
///  use std::fs::File;
///  use ebi_bpmn::BusinessProcessModelAndNotation;
///
///  fn main() -> anyhow::Result<()> {
///   let f = File::open("testfiles/model.bpmn")?;
///   let mut reader = BufReader::new(f);
///
///   let bpmn = BusinessProcessModelAndNotation::import_from_reader(&mut reader, true)?;
///
///   let mut marking = bpmn.get_initial_marking()?.unwrap();
///  assert_eq!(bpmn.get_enabled_transitions(&marking)?, vec![0]);
///   bpmn.execute_transition(&mut marking, 0)?;
///
///  Ok(())
///  }
///  ```
///
///  To create a BPMN model programmatically, please consider using a [BPMNCreator].
///
/// [BPMNCreator]: crate::BPMNCreator

#[derive(Clone, ActivityKey, Debug)]
pub struct BusinessProcessModelAndNotation {
    pub(crate) stochastic_namespace: bool,
    pub activity_key: ActivityKey,

    pub collaboration_index: Option<GlobalIndex>,
    pub collaboration_id: Option<String>,
    pub definitions_index: GlobalIndex,
    pub definitions_id: String,

    pub elements: Vec<BPMNElement>,
    pub message_flows: Vec<BPMNMessageFlow>,
}

impl BusinessProcessModelAndNotation {
    /// Returns the number of elements in the BPMN model (recurses).
    pub fn number_of_elements(&self) -> usize {
        self.elements().len()
    }

    /// Returns the number of message flows in the BPMN model (does not recurse).
    pub fn number_of_message_flows(&self) -> usize {
        self.message_flows.len()
    }

    /// Returns all elements in the model (recurses).
    pub fn elements(&self) -> Vec<&BPMNElement> {
        self.elements.all_elements_ref()
    }

    /// Returns the parent of the element or flow with the given index (recursively).
    pub fn parent_of(&self, global_index: GlobalIndex) -> Option<&dyn Processable> {
        for element in &self.elements {
            let x = element.parent_of(global_index);
            if x.1 {
                return x.0;
            }
        }
        None
    }

    /// Returns all sequence flows (recurses).
    pub fn sequence_flows(&self) -> Vec<&BPMNSequenceFlow> {
        self.elements.all_sequence_flows_ref()
    }

    /// Find an element with the given index (recurses).
    pub fn global_index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement> {
        self.elements.global_index_2_element(index)
    }

    /// Find an element with the given index and returns a mutable reference to it (recurses).
    pub fn global_index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement> {
        self.elements.global_index_2_element_mut(index)
    }

    /// Returns the element that is the source of the given message flow.
    pub fn message_flow_index_2_source(&self, message_flow_index: usize) -> Result<&BPMNElement> {
        let message_flow = self
            .message_flows
            .get(message_flow_index)
            .ok_or_else(|| anyhow!("Message flow of index {} not found.", message_flow_index))?;
        self.global_index_2_element(message_flow.source_global_index)
            .ok_or_else(|| {
                anyhow!(
                    "The source of message flow `{}` was not found.",
                    message_flow.id
                )
            })
    }

    /// Returns the element that is the target of the given message flow.
    pub fn message_flow_index_2_target(&self, message_flow_index: usize) -> Result<&BPMNElement> {
        let message_flow = self
            .message_flows
            .get(message_flow_index)
            .ok_or_else(|| anyhow!("Message flow of index {} not found.", message_flow_index))?;
        self.global_index_2_element(message_flow.target_global_index)
            .ok_or_else(|| {
                anyhow!(
                    "The target of message flow `{}` was not found.",
                    message_flow.id
                )
            })
    }

    /// Returns the sequence flow with the given global index.
    pub fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, &dyn Processable)> {
        self.elements
            .global_index_2_sequence_flow_and_parent(sequence_flow_global_index)
    }

    /// Returns the sequence flow with this index, if it exists (recurses).
    pub fn global_index_2_sequence_flow_mut(
        &mut self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<&mut BPMNSequenceFlow> {
        self.elements
            .global_index_2_sequence_flow_mut(sequence_flow_global_index)
    }
}

impl Display for BusinessProcessModelAndNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BPMN model with {} elements", self.number_of_elements())
    }
}

impl TranslateActivityKey for BusinessProcessModelAndNotation {
    fn translate_using_activity_key(&mut self, to_activity_key: &mut ActivityKey) {
        let translator = ActivityKeyTranslator::new(&self.activity_key, to_activity_key);

        //gather indices of elements
        let mut indices = vec![];
        for element in self.elements() {
            if element.is_task() || element.is_collapsed_sub_process() {
                indices.push(element.global_index());
            }
        }

        //adjust activities
        for index in indices {
            match self.elements.global_index_2_element_mut(index) {
                Some(BPMNElement::Task(BPMNTask { activity, .. }))
                | Some(BPMNElement::CollapsedSubProcess(BPMNCollapsedSubProcess {
                    activity,
                    ..
                })) => {
                    *activity = translator.translate_activity(&activity);
                }
                _ => unreachable!(),
            }
        }

        self.activity_key = to_activity_key.clone();
    }
}

#[cfg(any(test, feature = "testactivities"))]
impl TestActivityKey for BusinessProcessModelAndNotation {
    fn test_activity_key(&self) {
        for element in self.elements() {
            if let BPMNElement::Task(BPMNTask { activity, .. }) = element {
                self.activity_key.assert_activity_is_of_key(activity);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BusinessProcessModelAndNotation;
    use ebi_activity_key::TranslateActivityKey;
    use ebi_activity_key::has_activity_key::TestActivityKey;
    use std::fs::{self};

    #[test]
    fn bpmn_pool_translate() {
        let fin = fs::read_to_string("testfiles/model-lanes.bpmn").unwrap();
        let mut bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        bpmn.test_activity_key();

        let fin2 = fs::read_to_string("testfiles/model.bpmn").unwrap();
        let mut bpmn2 = fin2.parse::<BusinessProcessModelAndNotation>().unwrap();

        bpmn.translate_using_activity_key(&mut bpmn2.activity_key);

        bpmn.test_activity_key();
    }
}
