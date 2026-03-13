use crate::parser::parser_state::GlobalIndex;

/// A struct that represents a message flow in a BPMN model.
#[derive(Clone, Debug)]
pub struct BPMNMessageFlow {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) source_pool_index: usize,
    pub(crate) source_global_index: GlobalIndex,
    pub(crate) target_pool_index: usize,
    pub(crate) target_global_index: GlobalIndex,
}

impl BPMNMessageFlow {
    pub fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    pub fn source_global_index(&self) -> GlobalIndex {
        self.source_global_index
    }

    pub fn target_global_index(&self) -> GlobalIndex {
        self.target_global_index
    }
}
