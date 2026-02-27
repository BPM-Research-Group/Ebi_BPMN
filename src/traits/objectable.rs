use crate::{BusinessProcessModelAndNotation, parser::parser_state::GlobalIndex};
use anyhow::Result;

pub(crate) static EMPTY_FLOWS: Vec<usize> = vec![];

/// provides methods to interact with BPMN objects. Cannot be implemented on Vec<...>.
pub trait BPMNObject {
    /// return the global index
    fn global_index(&self) -> GlobalIndex;

    /// return the index within its parent
    fn local_index(&self) -> usize;

    /// return the id
    fn id(&self) -> &str;

    /// return whether this is a start event that could initiate a process instance
    fn is_unconstrained_start_event(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<bool>;

    /// return whether this is an end event
    fn is_end_event(&self) -> bool;

    /// the flow indices of the incoming sequence flows of this object
    fn incoming_sequence_flows(&self) -> &[usize];

    /// the flow indices of the outgoing sequence flows of this object
    fn outgoing_sequence_flows(&self) -> &[usize];

    /// the flow indices of the incoming message flows of this object
    fn incoming_message_flows(&self) -> &[usize];

    /// the flow indices of the outgoing message flows of this object
    fn outgoing_message_flows(&self) -> &[usize];

    /// return whether this object could start a process instance, regardless of marking
    fn can_start_process_instance(&self, bpmn: &BusinessProcessModelAndNotation) -> Result<bool>;

    /// return whether the outgoing message flows of this object are always enabled
    fn outgoing_message_flows_always_have_tokens(&self) -> bool;

    /// return whether this object can have incoming sequence flows
    fn can_have_incoming_sequence_flows(&self) -> bool;

    /// return whether this object can have incoming sequence flows
    fn can_have_outgoing_sequence_flows(&self) -> bool;
}
