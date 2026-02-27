use std::rc::Rc;

use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    semantics::{BPMNMarking, BPMNSubMarking},
    sequence_flow::BPMNSequenceFlow,
    traits::startable::InitiationMode,
};
use anyhow::{Result, anyhow};

/// a trait with methods for container elements
pub trait Processable {
    fn elements_non_recursive(&self) -> &Vec<BPMNElement>;

    fn sequence_flows_non_recursive(&self) -> &Vec<BPMNSequenceFlow>;

    fn to_sub_marking(
        &self,
        initiation_mode: InitiationMode,
        root_marking: Rc<BPMNMarking>,
    ) -> Result<BPMNSubMarking>;
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
}
