use std::rc::Rc;

use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    semantics::{BPMNMarking, BPMNSubMarking},
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, startable::InitiationMode},
};
use anyhow::{Result, anyhow};

/// a trait with methods for container elements
pub trait Processable: BPMNObject {
    /// return the elements of this process, but do not recurse
    fn elements_non_recursive(&self) -> &Vec<BPMNElement>;

    /// return the sequence flows of this process, but do not recurse
    fn sequence_flows_non_recursive(&self) -> &Vec<BPMNSequenceFlow>;

    /// return a initial marking
    fn to_sub_marking(
        &self,
        initiation_mode: InitiationMode,
        root_marking: Rc<BPMNMarking>,
    ) -> Result<BPMNSubMarking>;

    /// return whether this is a sub-process, i.e. not a pool or a root of the model
    fn is_sub_process(&self) -> bool;
}

static EMPTY_FLOWS: Vec<BPMNSequenceFlow> = vec![];

impl Processable for BusinessProcessModelAndNotation {
    fn elements_non_recursive(&self) -> &Vec<BPMNElement> {
        &self.elements
    }

    fn sequence_flows_non_recursive(&self) -> &Vec<BPMNSequenceFlow> {
        &EMPTY_FLOWS
    }

    fn to_sub_marking(
        &self,
        _initiation_mode: InitiationMode,
        _root_marking: Rc<BPMNMarking>,
    ) -> Result<BPMNSubMarking> {
        Err(anyhow!("call the dedicated function"))
    }

    fn is_sub_process(&self) -> bool {
        false
    }
}
