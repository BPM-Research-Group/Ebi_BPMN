use crate::{
    element::BPMNElement,
    elements::task::BPMNTask,
    message_flow::BPMNMessageFlow,
    parser::parser_state::GlobalIndex,
    semantics::BPMNMarking,
    sequence_flow::BPMNSequenceFlow,
    traits::{
        objectable::BPMNObject, processable::Processable, searchable::Searchable,
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
#[cfg(any(test, feature = "testactivities"))]
use ebi_activity_key::TestActivityKey;
use ebi_activity_key::{ActivityKey, ActivityKeyTranslator, TranslateActivityKey};
use ebi_derive::ActivityKey;
use std::fmt::{Display, Formatter};

#[derive(Clone, ActivityKey, Debug)]
pub struct BusinessProcessModelAndNotation {
    pub(crate) activity_key: ActivityKey,

    pub collaboration_index: Option<GlobalIndex>,
    pub collaboration_id: Option<String>,
    pub definitions_index: GlobalIndex,
    pub definitions_id: String,

    pub elements: Vec<BPMNElement>,
    pub message_flows: Vec<BPMNMessageFlow>,
}

impl BusinessProcessModelAndNotation {
    pub fn number_of_elements(&self) -> usize {
        self.all_elements_ref().len()
    }

    pub fn number_of_message_flows(&self) -> usize {
        self.message_flows.len()
    }

    pub fn all_elements_ref(&self) -> Vec<&BPMNElement> {
        self.elements.all_elements_ref()
    }

    /// find an element with the given index
    pub fn global_index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement> {
        self.elements.global_index_2_element(index)
    }

    /// find the object of the given index
    pub fn index_2_object(&self, index: GlobalIndex) -> Option<&dyn BPMNObject> {
        self.elements.index_2_object(index)
    }

    /// return the element that is the source of the given message flow
    pub fn message_flow_index_2_source(&self, message_flow_index: usize) -> Result<&BPMNElement> {
        let message_flow = self
            .message_flows
            .get(message_flow_index)
            .ok_or_else(|| anyhow!("message flow of index {} not found", message_flow_index))?;
        self.global_index_2_element(message_flow.source_element_index)
            .ok_or_else(|| {
                anyhow!(
                    "the source of message flow `{}` was not found",
                    message_flow.id
                )
            })
    }

    /// return the sequence flow with the given global index
    pub fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, &dyn Processable)> {
        self.elements
            .global_index_2_sequence_flow_and_parent(sequence_flow_global_index)
    }

    pub fn transition_debug(
        &self,
        mut transition_index: usize,
        marking: &BPMNMarking,
    ) -> Option<String> {
        for (element, sub_marking) in self
            .elements
            .iter()
            .zip(marking.element_index_2_sub_markings.iter())
        {
            let number_of_transitions = element.number_of_transitions(sub_marking);
            if transition_index < number_of_transitions {
                return element.transition_debug(transition_index, sub_marking);
            }
            transition_index -= number_of_transitions;
        }
        None
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
        for element in self.all_elements_ref() {
            if element.is_task() {
                indices.push(element.global_index());
            }
        }

        //adjust activities
        for index in indices {
            if let Some(BPMNElement::Task(BPMNTask { activity, .. })) =
                self.elements.global_index_2_element_mut(index)
            {
                *activity = translator.translate_activity(&activity);
            } else {
                unreachable!()
            }
        }

        self.activity_key = to_activity_key.clone();
    }
}

#[cfg(any(test, feature = "testactivities"))]
impl TestActivityKey for BusinessProcessModelAndNotation {
    fn test_activity_key(&self) {
        for element in self.all_elements_ref() {
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
