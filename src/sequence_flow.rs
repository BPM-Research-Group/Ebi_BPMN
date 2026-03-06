use ebi_arithmetic::Fraction;

use crate::parser::parser_state::GlobalIndex;

#[derive(Clone, Debug)]
pub struct BPMNSequenceFlow {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) source_global_index: GlobalIndex,
    pub(crate) source_local_index: usize,
    pub(crate) target_global_index: GlobalIndex,
    pub(crate) target_local_index: usize,
    pub weight: Option<Fraction>,
}
