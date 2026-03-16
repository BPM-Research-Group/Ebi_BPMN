use crate::{
    GlobalIndex,
    elements::collapsed_pool::BPMNCollapsedPool,
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

pub(crate) struct TagParticipant {}

impl Recognisable for TagParticipant {
    fn recognise_tag(e: &BytesStart, state: &ParserState, n: NameSpace) -> Option<Tag>
    where
        Self: Sized,
    {
        if n.is_bpmn() {
            match state.open_tags.iter().last() {
                Some(OpenedTag::Collaboration { .. }) => {
                    if e.local_name().as_ref() == b"participant" {
                        return Some(Tag::Participant);
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

impl Openable for TagParticipant {
    fn open_tag(_tag: Tag, e: &BytesStart, state: &mut ParserState) -> Result<OpenedTag>
    where
        Self: Sized,
    {
        let (index, id) = state.read_and_add_id(e)?;

        let name = parse_attribute(e, "name");

        let process_id = parse_attribute(e, "processRef");

        Ok(OpenedTag::Participant {
            global_index: index,
            id,
            name,
            process_id,
        })
    }
}

#[derive(Debug)]
pub(crate) struct DraftTagParticipant {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) process_id: String,
}

impl Closeable for TagParticipant {
    fn close_tag(opened_tag: OpenedTag, _e: &BytesEnd, state: &mut ParserState) -> Result<()> {
        match state.open_tags.iter_mut().last() {
            Some(OpenedTag::Collaboration {
                collapsed_pools,
                draft_participants,
                ..
            }) => {
                if let OpenedTag::Participant {
                    global_index,
                    id,
                    name,
                    process_id,
                } = opened_tag
                {
                    if let Some(process_id) = process_id {
                        //this is not a BPMN element (that's the process)
                        draft_participants.push(DraftTagParticipant {
                            global_index,
                            id,
                            name,
                            process_id,
                        });
                    } else {
                        //this is a collapsed pool
                        let local_index = collapsed_pools.len();
                        collapsed_pools.push(BPMNCollapsedPool {
                            global_index,
                            id,
                            local_index,
                            name,
                            incoming_message_flows: vec![],
                            outgoing_message_flows: vec![],
                        });
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}
