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
    parser::parser_state::GlobalIndex,
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, searchable::Searchable},
};
use anyhow::{Result, anyhow};
use ebi_activity_key::{Activity, ActivityKey};

/**
 * A struct that assists with creating BPMN models programmatically.
 */
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
        parent: Container,
        source: GlobalIndex,
        target: GlobalIndex,
    ) -> Result<()> {
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
                Ok(())
            }
            _ => Err(anyhow!("parent not found")),
        }
    }

    pub fn add_sequence_flow_unchecked(
        &mut self,
        parent: Container,
        source: GlobalIndex,
        target: GlobalIndex,
    ) {
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
            }
            _ => panic!("parent not found"),
        }
    }
}

#[derive(Copy, Clone)]
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
                id: format!("eventbasedgateway_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
                outgoing_sequence_flows: vec![],
            }),
            GatewayType::Exclusive => BPMNElement::ExclusiveGateway(BPMNExclusiveGateway {
                global_index,
                id: format!("exclusivegateway_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
                outgoing_sequence_flows: vec![],
            }),
            GatewayType::Inclusive => BPMNElement::InclusiveGateway(BPMNInclusiveGateway {
                global_index,
                id: format!("inclusivegateway_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
                outgoing_sequence_flows: vec![],
            }),
            GatewayType::Parallel => BPMNElement::ParallelGateway(BPMNParallelGateway {
                global_index,
                id: format!("parallelgateway_{}", global_index.0),
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
                id: format!("startevent_{}", global_index.0),
                local_index,
                incoming_sequence_flows: vec![],
            }),
            EndEventType::Message => BPMNElement::MessageEndEvent(BPMNMessageEndEvent {
                global_index,
                id: format!("startevent_{}", global_index.0),
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
