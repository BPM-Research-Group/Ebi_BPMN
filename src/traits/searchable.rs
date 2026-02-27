use crate::{
    element::BPMNElement, parser::parser_state::GlobalIndex, traits::objectable::BPMNObject,
};

pub trait Searchable {
    /// find an object with the given index
    fn index_2_object(&self, index: GlobalIndex) -> Option<&dyn BPMNObject>;

    /// find an object with the given id, returns (pool index, element index)
    fn id_2_pool_and_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)>;

    /// return all elements recursively
    fn all_elements_ref(&self) -> Vec<&BPMNElement>;

    /// find an element with the given index
    fn index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement>;

    /// find an element with the given index
    fn index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement>;
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

    fn id_2_pool_and_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)> {
        for element in self {
            let x = element.id_2_pool_and_index(id);
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

    fn index_2_element(&self, index: GlobalIndex) -> Option<&BPMNElement> {
        self.iter()
            .filter_map(|element| element.index_2_element(index))
            .next()
    }

    fn index_2_element_mut(&mut self, index: GlobalIndex) -> Option<&mut BPMNElement> {
        self.iter_mut()
            .filter_map(|element| element.index_2_element_mut(index))
            .next()
    }
}
