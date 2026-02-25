use crate::parser::{
    parser_state::ParserState,
    parser_traits::{Closeable, Openable, Recognisable},
    tags::{OpenedTag, Tag},
};
use anyhow::Result;
use quick_xml::events::{BytesEnd, BytesStart};

pub(crate) struct TagTimerEventDefinition {}

impl Recognisable for TagTimerEventDefinition {
    fn recognise_tag(
        e: &quick_xml::events::BytesStart,
        state: &super::parser_state::ParserState,
    ) -> Option<super::tags::Tag>
    where
        Self: Sized,
    {
        match state.open_tags.iter().last() {
            Some(OpenedTag::StartEvent { .. }) | Some(OpenedTag::IntermediateCatchEvent { .. }) => {
                if e.local_name().as_ref() == b"timerEventDefinition" {
                    return Some(Tag::TimerEventDefinition);
                }
            }
            _ => {}
        }
        None
    }
}

impl Openable for TagTimerEventDefinition {
    fn open_tag(_tag: Tag, e: &BytesStart, state: &mut ParserState) -> Result<OpenedTag>
    where
        Self: Sized,
    {
        let (_, id) = state.read_and_add_id(e)?;

        Ok(OpenedTag::TimerEventDefinition { id })
    }
}

impl Closeable for TagTimerEventDefinition {
    fn close_tag(opened_tag: OpenedTag, _e: &BytesEnd, state: &mut ParserState) -> Result<()> {
        let index = state.open_tags.len() - 1;
        match state.open_tags.get_mut(index) {
            Some(OpenedTag::StartEvent {
                timer_marker_id: timer_id,
                ..
            })
            | Some(OpenedTag::IntermediateCatchEvent {
                timer_marker_id: timer_id,
                ..
            }) => {
                if let OpenedTag::TimerEventDefinition { id } = opened_tag {
                    *timer_id = Some(id);
                    Ok(())
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}
