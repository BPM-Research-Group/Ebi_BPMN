use crate::parser::parser_state::GlobalIndex;

#[derive(Clone, Debug)]
pub struct BPMNMessageFlow {
    pub global_index: GlobalIndex,
    pub id: String,
    pub source_pool_index: usize,
    pub source_element_index: GlobalIndex,
    pub target_pool_index: usize,
    pub target_element_index: GlobalIndex,
}
