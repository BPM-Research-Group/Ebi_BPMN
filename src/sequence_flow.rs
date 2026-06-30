use ebi_arithmetic::{Fraction, Signed};

use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    parser::parser_state::GlobalIndex,
    traits::{processable::Processable, searchable::Searchable},
};

/// A struct that represents a sequence flow in a BPMN model.
#[derive(Clone, Debug)]
pub struct BPMNSequenceFlow {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) source_global_index: GlobalIndex,
    pub(crate) source_local_index: usize,
    pub(crate) target_global_index: GlobalIndex,
    pub(crate) target_local_index: usize,

    /// A non-negative weight attached to the sequence flow.
    /// Meaningless in a [BusinessProcessModelAndNotation] model, but provides semantics in an [StochasticBusinessProcessModelAndNotation] model.
    ///
    /// [BusinessProcessModelAndNotation]: crate::BusinessProcessModelAndNotation
    /// [StochasticBusinessProcessModelAndNotation]: crate::StochasticBusinessProcessModelAndNotation
    pub weight: Option<Fraction>,
}

impl BPMNSequenceFlow {
    pub fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    pub fn source_global_index(&self) -> GlobalIndex {
        self.source_global_index
    }

    pub fn target_global_index(&self) -> GlobalIndex {
        self.target_global_index
    }

    pub fn has_fireable_weight(&self, bpmn: &BusinessProcessModelAndNotation) -> bool {
        if !bpmn.stochastic_namespace {
            true
        } else if let Some(weight) = &self.weight {
            weight.is_positive()
        } else {
            true
        }
    }
}

impl Searchable for BPMNSequenceFlow {
    fn id_2_pool_and_global_index(&self, id: &str) -> Option<(Option<usize>, GlobalIndex)> {
        if self.id == id {
            Some((None, self.global_index()))
        } else {
            None
        }
    }

    fn id_2_local_index(&self, id: &str) -> Option<usize> {
        if self.id == id {
            Some(self.local_index)
        } else {
            None
        }
    }

    fn global_index_2_sequence_flow_and_parent(
        &self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<(&BPMNSequenceFlow, Option<&dyn Processable>)> {
        if self.global_index == sequence_flow_global_index {
            Some((self, None))
        } else {
            None
        }
    }

    fn all_elements_ref(&self) -> Vec<&BPMNElement> {
        vec![]
    }

    fn parent_of(&self, global_index: GlobalIndex) -> (Option<&dyn Processable>, bool) {
        if self.global_index() == global_index {
            (None, true)
        } else {
            (None, false)
        }
    }

    fn all_sequence_flows_ref(&self) -> Vec<&BPMNSequenceFlow> {
        vec![self]
    }

    fn global_index_2_sequence_flow_mut(
        &mut self,
        sequence_flow_global_index: GlobalIndex,
    ) -> Option<&mut BPMNSequenceFlow> {
        if self.global_index == sequence_flow_global_index {
            Some(self)
        } else {
            None
        }
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
