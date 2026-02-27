use crate::{
    element::BPMNElement,
    elements::{
        message_start_event::BPMNMessageStartEvent, start_event::BPMNStartEvent,
        timer_start_event::BPMNTimerStartEvent,
    },
    parser::{
        parser_state::ParserState,
        parser_traits::{Closeable, Openable, Recognisable},
        tags::{OpenedTag, Tag},
    },
};
use anyhow::{Result, anyhow};
use quick_xml::events::{BytesEnd, BytesStart};

pub struct TagStartEvent {}

impl Recognisable for TagStartEvent {
    fn recognise_tag(e: &BytesStart, state: &ParserState) -> Option<Tag>
    where
        Self: Sized,
    {
        match state.open_tags.iter().last() {
            Some(OpenedTag::Process { .. }) | Some(OpenedTag::SubProcess { .. }) => {
                if e.local_name().as_ref() == b"startEvent" {
                    return Some(Tag::StartEvent);
                }
            }
            _ => {}
        }
        None
    }
}

impl Openable for TagStartEvent {
    fn open_tag(_tag: Tag, e: &BytesStart, state: &mut ParserState) -> Result<OpenedTag>
    where
        Self: Sized,
    {
        let (index, id) = state.read_and_add_id(e)?;

        Ok(OpenedTag::StartEvent {
            global_index: index,
            id,
            message_marker_id: None,
            timer_marker_id: None,
        })
    }
}

impl Closeable for TagStartEvent {
    fn close_tag(opened_tag: OpenedTag, _e: &BytesEnd, state: &mut ParserState) -> Result<()> {
        match state.open_tags.iter_mut().last() {
            Some(OpenedTag::Process { elements, .. })
            | Some(OpenedTag::SubProcess { elements, .. }) => {
                if let OpenedTag::StartEvent {
                    global_index,
                    id,
                    message_marker_id,
                    timer_marker_id,
                } = opened_tag
                {
                    let local_index = elements.len();
                    match (message_marker_id, timer_marker_id) {
                        (None, None) => {
                            //no marker
                            elements.push(BPMNElement::StartEvent(BPMNStartEvent {
                                global_index,
                                id,
                                local_index,
                                outgoing_sequence_flows: vec![],
                            }));
                        }
                        (None, Some(timer_marker_id)) => {
                            //timer marker
                            elements.push(BPMNElement::TimerStartEvent(BPMNTimerStartEvent {
                                global_index,
                                id,
                                local_index,
                                timer_marker_id,
                                outgoing_sequence_flows: vec![],
                            }));
                        }
                        (Some(message_marker_id), None) => {
                            //message marker
                            elements.push(BPMNElement::MessageStartEvent(BPMNMessageStartEvent {
                                global_index,
                                id,
                                local_index,
                                message_marker_id,
                                outgoing_sequence_flows: vec![],
                                incoming_message_flow: None,
                            }));
                        }
                        (Some(_), Some(_)) => {
                            return Err(anyhow!(
                                "a start event cannot be both a timer and a message event"
                            ));
                        }
                    }
                    Ok(())
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}
