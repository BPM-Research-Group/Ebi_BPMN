use crate::{
    BusinessProcessModelAndNotation,
    element::{BPMNElement, BPMNElementTrait},
    elements::{
        end_event::BPMNEndEvent, event_based_gateway::BPMNEventBasedGateway,
        exclusive_gateway::BPMNExclusiveGateway, expanded_sub_process::BPMNExpandedSubProcess,
        inclusive_gateway::BPMNInclusiveGateway,
        intermediate_catch_event::BPMNIntermediateCatchEvent,
        intermediate_throw_event::BPMNIntermediateThrowEvent,
        message_end_event::BPMNMessageEndEvent,
        message_intermediate_catch_event::BPMNMessageIntermediateCatchEvent,
        message_start_event::BPMNMessageStartEvent, parallel_gateway::BPMNParallelGateway,
        process::BPMNProcess, start_event::BPMNStartEvent, task::BPMNTask,
        timer_intermediate_catch_event::BPMNTimerIntermediateCatchEvent,
        timer_start_event::BPMNTimerStartEvent,
    },
    if_not::IfNot,
    parser::parser_state::GlobalIndex,
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, searchable::Searchable},
};
use anyhow::{Result, anyhow};
use ebi_activity_key::{Activity, ActivityKey};

/// A helper struct that assists with creating BPMN models programmatically.
/// The advantage of a [BPMNCreator] over editing a [BusinessProcessModelAndNotation] struct directly is that the methods of a [BPMNCreator] are guaranteed to leave the model in a valid state.
/// Structural correctness is verified on transformation to [BusinessProcessModelAndNotation].
pub struct BPMNCreator {
    bpmn: BusinessProcessModelAndNotation,
    max_id: usize,
}

impl BPMNCreator {
    pub fn new() -> Self {
        let bpmn = BusinessProcessModelAndNotation {
            stochastic_namespace: false,
            activity_key: ActivityKey::new(),
            collaboration_index: None,
            collaboration_id: None,
            definitions_index: (0, ()),
            definitions_id: "definitions".to_string(),
            elements: vec![],
            message_flows: vec![],
        };
        Self { bpmn, max_id: 0 }
    }

    pub fn new_with_activity_key(activity_key: ActivityKey) -> Self {
        let bpmn = BusinessProcessModelAndNotation {
            stochastic_namespace: false,
            activity_key,
            collaboration_index: None,
            collaboration_id: None,
            definitions_index: (0, ()),
            definitions_id: "definitions".to_string(),
            elements: vec![],
            message_flows: vec![],
        };
        Self { bpmn, max_id: 0 }
    }

    /// Checks the model for structural correctness, and returns the model.
    pub fn to_bpmn(self) -> Result<BusinessProcessModelAndNotation> {
        self.bpmn.is_structurally_correct()?;
        Ok(self.bpmn)
    }

    fn new_global_index(&mut self) -> GlobalIndex {
        self.max_id += 1;
        (self.max_id, ())
    }

    pub fn add_process(&mut self, name: Option<String>) -> Container {
        let global_index = self.new_global_index();
        let process = BPMNElement::Process(BPMNProcess {
            global_index,
            id: format!("process_{}", global_index.0),
            local_index: self.bpmn.elements.len(),
            name,
            participant_global_index: None,
            participant_id: None,
            elements: vec![],
            sequence_flows: vec![],
        });
        self.bpmn.elements.push(process);
        Container { global_index }
    }

    pub fn add_start_event(
        &mut self,
        parent: Container,
        start_event_type: StartEventType,
    ) -> Result<GlobalIndex> {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(start_event_type.to_element(global_index, local_index));
                Ok(global_index)
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn add_start_event_unchecked(
        &mut self,
        parent: Container,
        start_event_type: StartEventType,
    ) -> GlobalIndex {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(start_event_type.to_element(global_index, local_index));
                global_index
            }
            _ => panic!("parent not found"),
        }
    }

    pub fn add_intermediate_event(
        &mut self,
        parent: Container,
        intermediate_event_type: IntermediateEventType,
    ) -> Result<GlobalIndex> {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(intermediate_event_type.to_element(global_index, local_index));
                Ok(global_index)
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn add_intermediate_event_unchecked(
        &mut self,
        parent: Container,
        intermediate_event_type: IntermediateEventType,
    ) -> GlobalIndex {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(intermediate_event_type.to_element(global_index, local_index));
                global_index
            }
            _ => panic!("parent not found"),
        }
    }

    pub fn add_end_event(
        &mut self,
        parent: Container,
        end_event_type: EndEventType,
    ) -> Result<GlobalIndex> {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(end_event_type.to_element(global_index, local_index));
                Ok(global_index)
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn add_end_event_unchecked(
        &mut self,
        parent: Container,
        end_event_type: EndEventType,
    ) -> GlobalIndex {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(end_event_type.to_element(global_index, local_index));
                global_index
            }
            _ => panic!("parent not found"),
        }
    }

    pub fn add_task(&mut self, parent: Container, activity: Activity) -> Result<GlobalIndex> {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(BPMNElement::Task(BPMNTask {
                    global_index,
                    id: format!("task_{}", global_index.0),
                    local_index,
                    activity,
                    incoming_sequence_flows: vec![],
                    outgoing_sequence_flows: vec![],
                    incoming_message_flow: None,
                    outgoing_message_flow: None,
                }));
                Ok(global_index)
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn add_task_unchecked(&mut self, parent: Container, activity: Activity) -> GlobalIndex {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(BPMNElement::Task(BPMNTask {
                    global_index,
                    id: format!("task_{}", global_index.0),
                    local_index,
                    activity,
                    incoming_sequence_flows: vec![],
                    outgoing_sequence_flows: vec![],
                    incoming_message_flow: None,
                    outgoing_message_flow: None,
                }));
                global_index
            }
            _ => panic!("parent not found"),
        }
    }

    pub fn add_gateway(
        &mut self,
        parent: Container,
        gateway_type: GatewayType,
    ) -> Result<GlobalIndex> {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(gateway_type.to_element(global_index, local_index));
                Ok(global_index)
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn add_gateway_unchecked(
        &mut self,
        parent: Container,
        gateway_type: GatewayType,
    ) -> GlobalIndex {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess { elements, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess { elements, .. })) => {
                let local_index = elements.len();
                elements.push(gateway_type.to_element(global_index, local_index));
                global_index
            }
            _ => panic!("parent not found"),
        }
    }

    pub fn add_sequence_flow(
        &mut self,
        source: GlobalIndex,
        target: GlobalIndex,
    ) -> Result<GlobalIndex> {
        let global_index = self.new_global_index();
        let parent_a = self
            .bpmn
            .parent_of(source)
            .and_if_not("Parent not found.")?;

        let parent_b = self
            .bpmn
            .parent_of(target)
            .and_if_not("Parent not found.")?;

        if parent_a.global_index() != parent_b.global_index() {
            return Err(anyhow!("Elements have different parents."));
        }

        match self
            .bpmn
            .global_index_2_element_mut(parent_a.global_index())
        {
            Some(BPMNElement::Process(BPMNProcess {
                elements,
                sequence_flows,
                ..
            }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess {
                elements,
                sequence_flows,
                ..
            })) => {
                let local_index = sequence_flows.len();

                //find source
                let source = elements
                    .global_index_2_element_mut(source)
                    .ok_or_else(|| anyhow!("source not found"))?;
                source.add_outgoing_sequence_flow(local_index)?;
                let source_global_index = source.global_index();
                let source_local_index = source.local_index();

                let target = elements
                    .global_index_2_element_mut(target)
                    .ok_or_else(|| anyhow!("target not found"))?;
                target.add_incoming_sequence_flow(local_index)?;
                let target_global_index = target.global_index();
                let target_local_index = target.local_index();

                sequence_flows.push(BPMNSequenceFlow {
                    global_index,
                    id: format!("sequenceflow_{}", global_index.0),
                    local_index,
                    source_global_index,
                    source_local_index,
                    target_global_index,
                    target_local_index,
                    weight: None,
                });
                Ok(global_index)
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn add_sequence_flow_unchecked(
        &mut self,
        parent: Container,
        source: GlobalIndex,
        target: GlobalIndex,
    ) -> GlobalIndex {
        let global_index = self.new_global_index();
        match self.bpmn.global_index_2_element_mut(parent.global_index) {
            Some(BPMNElement::Process(BPMNProcess {
                elements,
                sequence_flows,
                ..
            }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess {
                elements,
                sequence_flows,
                ..
            })) => {
                let local_index = sequence_flows.len();

                //find source
                let source = elements.global_index_2_element_mut(source).unwrap();
                source.add_outgoing_sequence_flow(local_index).unwrap();
                let source_global_index = source.global_index();
                let source_local_index = source.local_index();

                let target = elements.global_index_2_element_mut(target).unwrap();
                target.add_incoming_sequence_flow(local_index).unwrap();
                let target_global_index = target.global_index();
                let target_local_index = target.local_index();

                sequence_flows.push(BPMNSequenceFlow {
                    global_index,
                    id: format!("sequenceflow_{}", global_index.0),
                    local_index,
                    source_global_index,
                    source_local_index,
                    target_global_index,
                    target_local_index,
                    weight: None,
                });
                global_index
            }
            _ => panic!("parent not found"),
        }
    }

    /// Removes a sequence flow with the given index `sequence_flow` from the `parent`.
    /// Returns an error if the parent cannot be found.
    pub fn remove_sequence_flow(
        &mut self,
        sequence_flow: GlobalIndex,
    ) -> Result<()> {
        let parent = self
            .bpmn
            .parent_of(sequence_flow)
            .and_if_not("Parent not found.")?;

        match self.bpmn.global_index_2_element_mut(parent.global_index()) {
            Some(BPMNElement::Process(BPMNProcess {
                elements,
                sequence_flows,
                ..
            }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess {
                elements,
                sequence_flows,
                ..
            })) => {
                if let Some(local_index) = sequence_flows
                    .iter()
                    .position(|f| f.global_index == sequence_flow)
                {
                    //we need to update every local index of every sequence flow that is >= local_index
                    for element in elements.iter_mut() {
                        let new_incoming_sequence_flows = element
                            .incoming_sequence_flows()
                            .iter()
                            .filter_map(|x| {
                                if *x < local_index {
                                    Some(*x)
                                } else if *x == local_index {
                                    None
                                } else {
                                    Some(x - 1)
                                }
                            })
                            .collect::<Vec<_>>();
                        element.clear_incoming_sequence_flows();
                        for nosf in new_incoming_sequence_flows {
                            element.add_incoming_sequence_flow(nosf)?;
                        }

                        let new_outgoing_sequence_flows = element
                            .outgoing_sequence_flows()
                            .iter()
                            .filter_map(|x| {
                                if *x < local_index {
                                    Some(*x)
                                } else if *x == local_index {
                                    None
                                } else {
                                    Some(x - 1)
                                }
                            })
                            .collect::<Vec<_>>();
                        element.clear_outgoing_sequence_flows();
                        for nosf in new_outgoing_sequence_flows {
                            element.add_outgoing_sequence_flow(nosf)?;
                        }
                    }

                    sequence_flows.retain(|f| f.global_index != sequence_flow);
                    Ok(())
                } else {
                    Err(anyhow!("Sequence flow not found."))
                }
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn incoming_sequence_flows_of_element(
        &self,
        element: GlobalIndex,
    ) -> Result<impl Iterator<Item = GlobalIndex>> {
        let parent = self
            .bpmn
            .parent_of(element)
            .and_if_not("Parent not found.")?;
        match self.bpmn.global_index_2_element(parent.global_index()) {
            Some(BPMNElement::Process(BPMNProcess { sequence_flows, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess {
                sequence_flows,
                ..
            })) => {
                let element = self
                    .bpmn
                    .global_index_2_element(element)
                    .ok_or_else(|| anyhow!("Element not found."))?;

                Ok(element
                    .incoming_sequence_flows()
                    .iter()
                    .map(|local_id| sequence_flows[*local_id].global_index))
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn outgoing_sequence_flows_of_element(
        &self,
        element: GlobalIndex,
    ) -> Result<impl Iterator<Item = GlobalIndex>> {
        let parent = self
            .bpmn
            .parent_of(element)
            .and_if_not("Parent not found.")?;

        match self.bpmn.global_index_2_element(parent.global_index()) {
            Some(BPMNElement::Process(BPMNProcess { sequence_flows, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess {
                sequence_flows,
                ..
            })) => {
                let element = self
                    .bpmn
                    .global_index_2_element(element)
                    .ok_or_else(|| anyhow!("Element not found."))?;

                Ok(element
                    .outgoing_sequence_flows()
                    .iter()
                    .map(|local_id| sequence_flows[*local_id].global_index))
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn source_of_sequence_flow(&self, sequence_flow: GlobalIndex) -> Option<GlobalIndex> {
        let (sequence_flow, _) = self
            .bpmn
            .global_index_2_sequence_flow_and_parent(sequence_flow)?;
        Some(sequence_flow.source_global_index())
    }

    pub fn target_of_sequence_flow(&self, sequence_flow: GlobalIndex) -> Option<GlobalIndex> {
        let (sequence_flow, _) = self
            .bpmn
            .global_index_2_sequence_flow_and_parent(sequence_flow)?;
        Some(sequence_flow.target_global_index())
    }

    /// Returns a list of all elements; an element is anything that is not a flow.
    pub fn elements(&self) -> Vec<GlobalIndex> {
        self.bpmn
            .elements()
            .iter()
            .map(|element| element.global_index())
            .collect()
    }

    /// Swaps the incoming sequence flows of two elements.
    /// Returns an error if one of the elements cannot have incoming sequence flows.
    pub fn swap_incoming_sequence_flows(
        &mut self,
        element_a: GlobalIndex,
        element_b: GlobalIndex,
    ) -> Result<()> {
        let parent_a = Container {
            global_index: self
                .bpmn
                .parent_of(element_a)
                .and_if_not("Parent not found.")?
                .global_index(),
        };

        let parent_b = Container {
            global_index: self
                .bpmn
                .parent_of(element_b)
                .and_if_not("Parent not found.")?
                .global_index(),
        };

        if parent_a != parent_b {
            return Err(anyhow!("Elements have different parents."));
        }

        let element_a_global_index = element_a;
        let element_b_global_index = element_b;

        //first, swap the pointers to the flows within the elements
        let element_a = self
            .bpmn
            .global_index_2_element_mut(element_a_global_index)
            .and_if_not("Element not found.")?;
        let element_a_local_index = element_a.local_index();
        let incoming_a = std::mem::take(element_a.incoming_sequence_flows_mut()?);

        let element_b = self
            .bpmn
            .global_index_2_element_mut(element_b_global_index)
            .and_if_not("Element not found.")?;
        let element_b_local_index = element_b.local_index();
        let incoming_b = std::mem::take(element_b.incoming_sequence_flows_mut()?);

        let element_a = self
            .bpmn
            .global_index_2_element_mut(element_a_global_index)
            .and_if_not("Element not found.")?;
        _ = std::mem::replace(element_a.incoming_sequence_flows_mut()?, incoming_b.clone());

        let element_b = self
            .bpmn
            .global_index_2_element_mut(element_b_global_index)
            .and_if_not("Element not found.")?;
        _ = std::mem::replace(element_b.incoming_sequence_flows_mut()?, incoming_a.clone());

        //second, update the pointers in the sequence flows of the parent
        match self.bpmn.global_index_2_element_mut(parent_a.global_index) {
            Some(BPMNElement::Process(BPMNProcess { sequence_flows, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess {
                sequence_flows,
                ..
            })) => {
                for sequence_flow_local_index in incoming_a {
                    let sequence_flow = &mut sequence_flows[sequence_flow_local_index];
                    sequence_flow.target_global_index = element_b_global_index;
                    sequence_flow.target_local_index = element_b_local_index;
                }
                for sequence_flow_local_index in incoming_b {
                    let sequence_flow = &mut sequence_flows[sequence_flow_local_index];
                    sequence_flow.target_global_index = element_a_global_index;
                    sequence_flow.target_local_index = element_a_local_index;
                }
            }
            _ => return Err(anyhow!("Parent not found.")),
        }
        Ok(())
    }

    /// Swaps the outgoing sequence flows of two elements.
    /// Returns an error if one of the elements cannot have outgoing sequence flows.
    pub fn swap_outgoing_sequence_flows(
        &mut self,
        element_a: GlobalIndex,
        element_b: GlobalIndex,
    ) -> Result<()> {
        let parent_a = Container {
            global_index: self
                .bpmn
                .parent_of(element_a)
                .and_if_not("Parent not found.")?
                .global_index(),
        };

        let parent_b = Container {
            global_index: self
                .bpmn
                .parent_of(element_b)
                .and_if_not("Parent not found.")?
                .global_index(),
        };

        if parent_a != parent_b {
            return Err(anyhow!("Elements have different parents."));
        }

        let element_a_global_index = element_a;
        let element_b_global_index = element_b;

        //first, swap the pointers to the flows within the elements
        let element_a = self
            .bpmn
            .global_index_2_element_mut(element_a_global_index)
            .and_if_not("Element not found.")?;
        let element_a_local_index = element_a.local_index();
        let outgoing_a = std::mem::take(element_a.outgoing_sequence_flows_mut()?);

        let element_b = self
            .bpmn
            .global_index_2_element_mut(element_b_global_index)
            .and_if_not("Element not found.")?;
        let element_b_local_index = element_b.local_index();
        let outgoing_b = std::mem::take(element_b.outgoing_sequence_flows_mut()?);

        let element_a = self
            .bpmn
            .global_index_2_element_mut(element_a_global_index)
            .and_if_not("Element not found.")?;
        _ = std::mem::replace(element_a.outgoing_sequence_flows_mut()?, outgoing_b.clone());

        let element_b = self
            .bpmn
            .global_index_2_element_mut(element_b_global_index)
            .and_if_not("Element not found.")?;
        _ = std::mem::replace(element_b.outgoing_sequence_flows_mut()?, outgoing_a.clone());

        //second, update the pointers in the sequence flows of the parent
        match self.bpmn.global_index_2_element_mut(parent_a.global_index) {
            Some(BPMNElement::Process(BPMNProcess { sequence_flows, .. }))
            | Some(BPMNElement::ExpandedSubProcess(BPMNExpandedSubProcess {
                sequence_flows,
                ..
            })) => {
                for sequence_flow_local_index in outgoing_a {
                    let sequence_flow = &mut sequence_flows[sequence_flow_local_index];
                    sequence_flow.source_global_index = element_b_global_index;
                    sequence_flow.source_local_index = element_b_local_index;
                }
                for sequence_flow_local_index in outgoing_b {
                    let sequence_flow = &mut sequence_flows[sequence_flow_local_index];
                    sequence_flow.source_global_index = element_a_global_index;
                    sequence_flow.source_local_index = element_a_local_index;
                }
            }
            _ => return Err(anyhow!("Parent not found.")),
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Container {
    global_index: GlobalIndex,
}

pub enum GatewayType {
    EventBased,
    Exclusive,
    Inclusive,
    Parallel,
}

impl GatewayType {
    fn to_element(self, global_index: GlobalIndex, local_index: usize) -> BPMNElement {
        match self {
            GatewayType::EventBased => BPMNElement::EventBasedGateway(BPMNEventBasedGateway {
                global_index,
                id: format!("gateway_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
                outgoing_sequence_flows: vec![],
            }),
            GatewayType::Exclusive => BPMNElement::ExclusiveGateway(BPMNExclusiveGateway {
                global_index,
                id: format!("gateway_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
                outgoing_sequence_flows: vec![],
            }),
            GatewayType::Inclusive => BPMNElement::InclusiveGateway(BPMNInclusiveGateway {
                global_index,
                id: format!("gateway_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
                outgoing_sequence_flows: vec![],
            }),
            GatewayType::Parallel => BPMNElement::ParallelGateway(BPMNParallelGateway {
                global_index,
                id: format!("gateway_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
                outgoing_sequence_flows: vec![],
            }),
        }
    }
}

pub enum StartEventType {
    None,
    Message,
    Timer,
}

impl StartEventType {
    fn to_element(self, global_index: GlobalIndex, local_index: usize) -> BPMNElement {
        match self {
            StartEventType::None => BPMNElement::StartEvent(BPMNStartEvent {
                global_index,
                id: format!("startevent_{}", global_index.0),
                local_index,
                outgoing_sequence_flows: vec![],
            }),
            StartEventType::Message => BPMNElement::MessageStartEvent(BPMNMessageStartEvent {
                global_index,
                id: format!("messagestartevent_{}", global_index.0),
                local_index,
                incoming_message_flow: None,
                message_marker_id: Some(format!("messagemarker_{}", global_index.0)),
                outgoing_sequence_flows: vec![],
            }),
            StartEventType::Timer => BPMNElement::TimerStartEvent(BPMNTimerStartEvent {
                global_index,
                id: format!("timerstartevent_{}", global_index.0),
                local_index,
                timer_marker_id: Some(format!("timermarker_{}", global_index.0)),
                outgoing_sequence_flows: vec![],
            }),
        }
    }
}

pub enum EndEventType {
    None,
    Message,
}

impl EndEventType {
    fn to_element(self, global_index: GlobalIndex, local_index: usize) -> BPMNElement {
        match self {
            EndEventType::None => BPMNElement::EndEvent(BPMNEndEvent {
                global_index,
                id: format!("endevent_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
            }),
            EndEventType::Message => BPMNElement::MessageEndEvent(BPMNMessageEndEvent {
                global_index,
                id: format!("endevent_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
                message_marker_id: Some(format!("messagemarker_{}", global_index.0)),
                outgoing_message_flow: None,
            }),
        }
    }
}

pub enum IntermediateEventType {
    NoneCatch,
    NoneThrow,
    MessageCatch,
    MessageThrow,
    Timer,
}

impl IntermediateEventType {
    fn to_element(self, global_index: GlobalIndex, local_index: usize) -> BPMNElement {
        match self {
            IntermediateEventType::NoneCatch => {
                BPMNElement::IntermediateCatchEvent(BPMNIntermediateCatchEvent {
                    global_index,
                    id: format!("intermediatecatchevent_{}", global_index.0),
                    local_index,
                    incoming_sequence_flows: vec![],
                    outgoing_sequence_flows: vec![],
                })
            }
            IntermediateEventType::NoneThrow => {
                BPMNElement::IntermediateThrowEvent(BPMNIntermediateThrowEvent {
                    global_index,
                    id: format!("intermediatethrowevent_{}", global_index.0),
                    local_index,
                    incoming_sequence_flows: vec![],
                    outgoing_sequence_flows: vec![],
                })
            }
            IntermediateEventType::MessageCatch => {
                BPMNElement::MessageIntermediateCatchEvent(BPMNMessageIntermediateCatchEvent {
                    global_index,
                    id: format!("messageintermediatecatchevent_{}", global_index.0),
                    local_index,
                    incoming_sequence_flows: vec![],
                    outgoing_sequence_flows: vec![],
                    message_marker_id: Some(format!("messagemarker_{}", global_index.0)),
                    incoming_message_flow: None,
                })
            }
            IntermediateEventType::MessageThrow => {
                BPMNElement::MessageIntermediateCatchEvent(BPMNMessageIntermediateCatchEvent {
                    global_index,
                    id: format!("messageintermediatethrowevent_{}", global_index.0),
                    local_index,
                    incoming_sequence_flows: vec![],
                    outgoing_sequence_flows: vec![],
                    message_marker_id: Some(format!("messagemarker_{}", global_index.0)),
                    incoming_message_flow: None,
                })
            }
            IntermediateEventType::Timer => {
                BPMNElement::TimerIntermediateCatchEvent(BPMNTimerIntermediateCatchEvent {
                    global_index,
                    id: format!("timerintermediatethrowevent_{}", global_index.0),
                    local_index,
                    timer_marker_id: Some(format!("timermarker_{}", global_index.0)),
                    incoming_sequence_flows: vec![],
                    outgoing_sequence_flows: vec![],
                })
            }
        }
    }
}
