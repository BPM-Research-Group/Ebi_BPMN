use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    marking::BPMNSubMarking,
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, startable::InitiationMode},
};
use anyhow::{Result, anyhow};
use std::fmt::Debug;

/// a trait with methods for container elements
pub trait Processable: BPMNObject + Debug {
    /// Returns the elements of this process, but does not recurse.
    fn elements_non_recursive(&self) -> &Vec<BPMNElement>;

    /// return the sequence flows of this process, but do not recurse.
    fn sequence_flows_non_recursive(&self) -> &Vec<BPMNSequenceFlow>;

    /// return a initial marking
    fn to_sub_marking(&self, initiation_mode: &InitiationMode) -> Result<BPMNSubMarking>;

    /// return whether this is a sub-process, i.e. not a pool or a root of the model.
    fn is_sub_process(&self) -> bool;

    /// return the element that is the target of the given sequence flow
    fn sequence_flow_index_2_source(&self, sequence_flow_index: usize) -> Result<&BPMNElement> {
        let sequence_flow = self
            .sequence_flows_non_recursive()
            .get(sequence_flow_index)
            .ok_or_else(|| anyhow!("sequence flow of index {} not found", sequence_flow_index))?;
        self.elements_non_recursive()
            .get(sequence_flow.source_local_index)
            .ok_or_else(|| {
                anyhow!(
                    "the target of sequence flow `{}` was not found",
                    sequence_flow.id
                )
            })
    }

    /// return the element that is the target of the given sequence flow
    fn sequence_flow_index_2_target(&self, sequence_flow_index: usize) -> Result<&BPMNElement> {
        let sequence_flow = self
            .sequence_flows_non_recursive()
            .get(sequence_flow_index)
            .ok_or_else(|| anyhow!("sequence flow of index {} not found", sequence_flow_index))?;
        self.elements_non_recursive()
            .get(sequence_flow.target_local_index)
            .ok_or_else(|| {
                anyhow!(
                    "the target of sequence flow `{}` was not found",
                    sequence_flow.id
                )
            })
    }
}

static EMPTY_FLOWS: Vec<BPMNSequenceFlow> = vec![];

impl Processable for BusinessProcessModelAndNotation {
    fn elements_non_recursive(&self) -> &Vec<BPMNElement> {
        &self.elements
    }

    fn sequence_flows_non_recursive(&self) -> &Vec<BPMNSequenceFlow> {
        &EMPTY_FLOWS
    }

    fn to_sub_marking(&self, _initiation_mode: &InitiationMode) -> Result<BPMNSubMarking> {
        Err(anyhow!("call the dedicated function"))
    }

    fn is_sub_process(&self) -> bool {
        false
    }
}
