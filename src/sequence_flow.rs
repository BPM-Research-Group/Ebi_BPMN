use crate::parser::parser_state::GlobalIndex;

#[derive(Clone, Debug)]
pub struct BPMNSequenceFlow {
    pub global_index: GlobalIndex,
    pub id: String,
    pub flow_index: usize,
    pub source_global_index: GlobalIndex,
    pub source_local_index: usize,
    pub target_global_index: GlobalIndex,
    pub target_local_index: usize,
}
