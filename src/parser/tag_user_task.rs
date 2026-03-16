use crate::{
    element::BPMNElement,
    elements::user_task::BPMNUserTask,
    importer::parse_attribute,
    parser::{
        parser::NameSpace,
        parser_state::ParserState,
        parser_traits::{Closeable, Openable, Recognisable},
        tags::{OpenedTag, Tag},
    },
};
use anyhow::Result;
use quick_xml::events::{BytesEnd, BytesStart};

pub(crate) struct TagUserTask {}

impl Recognisable for TagUserTask {
    fn recognise_tag(e: &BytesStart, state: &ParserState, n: NameSpace) -> Option<Tag>
    where
        Self: Sized,
    {
        if n.is_bpmn() {
            match state.open_tags.iter().last() {
                Some(OpenedTag::Process { .. }) | Some(OpenedTag::SubProcess { .. }) => {
                    if e.local_name().as_ref() == b"userTask" {
                        return Some(Tag::UserTask);
                    }
                }
                _ => {}
            }
            None
        } else {
            None
        }
    }
}

impl Openable for TagUserTask {
    fn open_tag(_tag: Tag, e: &BytesStart, state: &mut ParserState) -> Result<OpenedTag>
    where
        Self: Sized,
    {
        let (index, id) = state.read_and_add_id(e)?;

        let label = parse_attribute(e, "name").unwrap_or_else(|| String::new());
        let activity = state.activity_key.process_activity(&label);
        Ok(OpenedTag::UserTask {
            global_index: index,
            id,
            activity,
        })
    }
}

impl Closeable for TagUserTask {
    fn close_tag(opened_tag: OpenedTag, _e: &BytesEnd, state: &mut ParserState) -> Result<()> {
        match state.open_tags.iter_mut().last() {
            Some(OpenedTag::Process { elements, .. })
            | Some(OpenedTag::SubProcess { elements, .. }) => {
                if let OpenedTag::UserTask {
                    global_index,
                    id,
                    activity,
                } = opened_tag
                {
                    let local_index = elements.len();
                    elements.push(BPMNElement::UserTask(BPMNUserTask {
                        global_index,
                        id,
                        local_index,
                        activity,
                        incoming_sequence_flows: vec![],
                        outgoing_sequence_flows: vec![],
                        incoming_message_flow: None,
                        outgoing_message_flow: None,
                    }));
                    Ok(())
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}
