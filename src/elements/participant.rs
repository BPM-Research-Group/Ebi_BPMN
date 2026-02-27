use crate::parser::parser_state::GlobalIndex;

#[derive(Debug, Clone)]
///A white-box participant
pub struct BPMNParticipant {
    pub global_index: GlobalIndex,
    pub id: String,
    pub name: Option<String>,
    pub process_id: String,
}
