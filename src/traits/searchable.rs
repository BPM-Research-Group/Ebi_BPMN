use crate::{
    element::BPMNElement,
    parser::parser_state::GlobalIndex,
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, processable::Processable},
};

pub(crate) trait Searchable {
    /// find an object with the given index
    fn index_2_object(&self, index: GlobalIndex) -> Option<&dyn BPMNObject>;

    /// find an object with the given id, returns (pool index, element index)
    fn id_2_pool_and_global_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)>;

    /// return the local index of the id, if it is exists (does not recurse)
    fn id_2_local_index(&self, id: &str) -> Option<usize>;

    /// return the sequence flow with this index, if it exists (recurses)
    fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, &dyn Processable)>;

    /// return all elements recursively
    fn all_elements_ref(&self) -> Vec<&BPMNElement>;

    /// find an element with the given index
    fn global_index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement>;

    /// find an element with the given index
    fn global_index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement>;

    /// find a local element with the given index
    fn local_index_2_element_mut(&mut self, index: usize) -> Option<&mut BPMNElement>;
}

impl Searchable for Vec<BPMNElement> {
    fn index_2_object(&self, index: GlobalIndex) -> Option<&dyn BPMNObject> {
        for process in self {
            let x = process.index_2_object(index);
            if x.is_some() {
                return x;
            }
        }
        None
    }

    fn id_2_pool_and_global_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)> {
        for element in self {
            let x = element.id_2_pool_and_global_index(id);
            if x.is_some() {
                return x;
            }
        }
        None
    }

    fn id_2_local_index(&self, id: &str) -> Option<usize> {
        for element in self {
            if element.id() == id {
                return Some(element.local_index());
            }
        }
        None
    }

    fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, &dyn Processable)> {
        for element in self {
            let x = element.global_index_2_sequence_flow_and_parent(sequence_flow_global_index);
            if x.is_some() {
                return x;
            }
        }
        None
    }

    fn all_elements_ref(&self) -> Vec<&BPMNElement> {
        self.iter()
            .map(|element| element.all_elements_ref())
            .flatten()
            .collect()
    }

    fn global_index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement> {
        self.iter()
            .filter_map(|element| element.global_index_2_element(index))
            .next()
    }

    fn global_index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement> {
        self.iter_mut()
            .filter_map(|element| element.global_index_2_element_mut(index))
            .next()
    }

    fn local_index_2_element_mut(&mut self, index: usize) -> Option<&mut BPMNElement> {
        self.get_mut(index)
    }
}
