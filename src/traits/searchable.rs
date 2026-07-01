use crate::{
    element::BPMNElement,
    parser::parser_state::GlobalIndex,
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, processable::Processable},
};

pub(crate) trait Searchable {
    /// find an object with the given id, returns (pool index, element index)
    fn id_2_pool_and_global_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)>;

    /// return the local index of the id, if it is exists (does not recurse)
    fn id_2_local_index(&self, id: &str) -> Option<usize>;

    /// return the sequence flow with this index, if it exists (recurses)
    fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, Option<&dyn Processable>)>;

    /// return all elements recursively
    fn all_elements_ref(&self) -> Vec<&BPMNElement>;

    /// return the direct parent that contains the global index
    fn parent_of(&self, global_index: GlobalIndex) -> (Option<&dyn Processable>, bool);

    /// return all sequence flows recursively
    fn all_sequence_flows_ref(&self) -> Vec<&BPMNSequenceFlow>;

    /// return the sequence flow with this index, if it exists (recurses)
    fn global_index_2_sequence_flow_mut(
        &mut self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<&mut BPMNSequenceFlow>;

    /// find an element with the given index
    fn global_index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement>;

    /// find an element with the given index
    fn global_index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement>;

    /// find a local element with the given index
    fn local_index_2_element(&self, index: usize) -> Option<&BPMNElement>;

    /// find a local element with the given index
    fn local_index_2_element_mut(&mut self, index: usize) -> Option<&mut BPMNElement>;
}

impl Searchable for Vec<BPMNElement> {
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
    ) -> Option<(&BPMNSequenceFlow, Option<&dyn Processable>)> {
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

    fn parent_of(&self, global_index: GlobalIndex) -> (Option<&dyn Processable>, bool) {
        for element in self {
            let x = element.parent_of(global_index);
            if x.1 {
                return x;
            }
        }
        (None, false)
    }

    fn all_sequence_flows_ref(&self) -> Vec<&BPMNSequenceFlow> {
        self.iter()
            .map(|element| element.all_sequence_flows_ref())
            .flatten()
            .collect()
    }

    fn global_index_2_sequence_flow_mut(
        &mut self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<&mut BPMNSequenceFlow> {
        self.iter_mut()
            .filter_map(|element| {
                element.global_index_2_sequence_flow_mut(sequence_flow_global_index)
            })
            .next()
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

    fn local_index_2_element(&self, index: usize) -> Option<&BPMNElement> {
        self.get(index)
    }

    fn local_index_2_element_mut(&mut self, index: usize) -> Option<&mut BPMNElement> {
        self.get_mut(index)
    }
}

impl Searchable for Vec<BPMNSequenceFlow> {
    fn id_2_pool_and_global_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)> {
        for flow in self {
            let x = flow.id_2_pool_and_global_index(id);
            if x.is_some() {
                return x;
            }
        }
        None
    }

    fn id_2_local_index(&self, id: &str) -> Option<usize> {
        for flow in self {
            if flow.id == id {
                return Some(flow.local_index);
            }
        }
        None
    }

    fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, Option<&dyn Processable>)> {
        for sequence_flow in self {
            if sequence_flow.global_index == sequence_flow_global_index {
                return Some((sequence_flow, None));
            }
        }
        None
    }

    fn all_elements_ref(&self) -> Vec<&BPMNElement> {
        vec![]
    }

    fn parent_of(&self, global_index: GlobalIndex) -> (Option<&dyn Processable>, bool) {
        for flow in self {
            let x = flow.parent_of(global_index);
            if x.1 {
                return x;
            }
        }
        (None, false)
    }

    fn all_sequence_flows_ref(&self) -> Vec<&BPMNSequenceFlow> {
        self.iter().collect()
    }

    fn global_index_2_sequence_flow_mut(
        &mut self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<&mut BPMNSequenceFlow> {
        self.iter_mut()
            .filter_map(|element| {
                element.global_index_2_sequence_flow_mut(sequence_flow_global_index)
            })
            .next()
    }

    fn global_index_2_element(&self, _index: GlobalIndex) -> Option<&BPMNElement> {
        None
    }

    fn global_index_2_element_mut(&mut self, _index: GlobalIndex) -> Option<&mut BPMNElement> {
        None
    }

    fn local_index_2_element(&self, _index: usize) -> Option<&BPMNElement> {
        None
    }

    fn local_index_2_element_mut(&mut self, _index: usize) -> Option<&mut BPMNElement> {
        None
    }
}
