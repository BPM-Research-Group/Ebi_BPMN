use crate::parser::parser_state::GlobalIndex;

#[derive(Clone, Debug)]
pub struct BPMNSequenceFlow {
    pub global_index: GlobalIndex,
    pub id: String,
    pub flow_index: usize,
    pub source_index: usize,
    pub target_index: usize,
}
