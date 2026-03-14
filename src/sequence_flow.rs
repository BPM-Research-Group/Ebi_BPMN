use ebi_arithmetic::{Fraction, Signed};

use crate::parser::parser_state::GlobalIndex;

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

    pub fn has_fireable_weight(&self) -> bool {
        if let Some(weight) = &self.weight {
            weight.is_positive()
        } else {
            true
        }
    }
}
