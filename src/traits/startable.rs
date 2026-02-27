use crate::{
    BusinessProcessModelAndNotation, element::BPMNElement, traits::objectable::BPMNObject,
};
use anyhow::Result;
use std::ops::Add;
use strum_macros::EnumIs;

/// A trait with methods to decide on the initiation mode of process instances.
pub trait Startable {
    /// find the start elements without recursing on sub-processes
    fn unconstrained_start_events_without_recursing(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<&BPMNElement>>;

    fn start_elements_without_recursing(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<&BPMNElement>>;

    /// find the end elements without recursing on sub-processes
    fn end_events_without_recursing(&self) -> Vec<&BPMNElement>;

    /// return the initiation mode
    fn initiation_mode<'a>(
        &'a self,
        bpmn: &'a BusinessProcessModelAndNotation,
    ) -> Result<InitiationMode<'a>> {
        if self
            .unconstrained_start_events_without_recursing(bpmn)?
            .is_empty()
        {
            let parallel_elements = self.start_elements_without_recursing(bpmn)?;
            return Ok(InitiationMode::ParallelElements(parallel_elements));
        } else {
            return Ok(InitiationMode::ChoiceBetweenStartEvents());
        }
    }
}

impl Startable for Vec<BPMNElement> {
    fn unconstrained_start_events_without_recursing(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<&BPMNElement>> {
        let mut result = vec![];
        for element in self {
            if element.is_unconstrained_start_event(bpmn)? {
                result.push(element);
            }
        }
        Ok(result)
    }

    fn start_elements_without_recursing(
        &self,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<Vec<&BPMNElement>> {
        let mut result = vec![];
        for element in self {
            if element.can_start_process_instance(bpmn)? {
                result.push(element);
            }
        }
        Ok(result)
    }

    fn end_events_without_recursing(&self) -> Vec<&BPMNElement> {
        self.iter()
            .filter(|element| element.is_end_event())
            .collect()
    }
}

#[derive(EnumIs)]
pub enum InitiationMode<'a> {
    ChoiceBetweenStartEvents(),
    ParallelElements(Vec<&'a BPMNElement>),
}

impl<'a> Add for InitiationMode<'a> {
    type Output = InitiationMode<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                InitiationMode::ChoiceBetweenStartEvents(),
                InitiationMode::ChoiceBetweenStartEvents(),
            ) => InitiationMode::ChoiceBetweenStartEvents(),
            (InitiationMode::ChoiceBetweenStartEvents(), InitiationMode::ParallelElements(_)) => {
                InitiationMode::ChoiceBetweenStartEvents()
            }
            (InitiationMode::ParallelElements(_), InitiationMode::ChoiceBetweenStartEvents()) => {
                InitiationMode::ChoiceBetweenStartEvents()
            }
            (
                InitiationMode::ParallelElements(mut bpmnelements1),
                InitiationMode::ParallelElements(bpmnelements2),
            ) => {
                bpmnelements1.extend(bpmnelements2);
                InitiationMode::ParallelElements(bpmnelements1)
            }
        }
    }
}
