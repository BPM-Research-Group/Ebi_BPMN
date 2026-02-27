use crate::{
    element::{BPMNElement, BPMNElementTrait},
    elements::{
        collapsed_sub_process::BPMNCollapsedSubProcess,
        expanded_sub_process::BPMNExpandedSubProcess,
    },
    importer::parse_attribute,
    parser::{
        parser_state::ParserState,
        parser_traits::{Closeable, Openable, Recognisable},
        tag_sequence_flow::DraftSequenceFlow,
        tags::{OpenedTag, Tag},
    },
    process_internal_sequence_flows,
    sequence_flow::BPMNSequenceFlow,
    traits::searchable::Searchable,
};
use anyhow::{Context, Result, anyhow};
use quick_xml::events::{BytesEnd, BytesStart};

pub struct TagSubProcess {}

impl Recognisable for TagSubProcess {
    fn recognise_tag(e: &BytesStart, state: &ParserState) -> Option<Tag>
    where
        Self: Sized,
    {
        match state.open_tags.iter().last() {
            Some(OpenedTag::Process { .. }) | Some(OpenedTag::SubProcess { .. }) => {
                if e.local_name().as_ref() == b"subProcess" {
                    return Some(Tag::SubProcess);
                }
            }
            _ => {}
        }
        None
    }
}

impl Openable for TagSubProcess {
    fn open_tag(_tag: Tag, e: &BytesStart, state: &mut ParserState) -> Result<OpenedTag>
    where
        Self: Sized,
    {
        let (index, id) = state.read_and_add_id(e)?;

        let name = parse_attribute(e, "name");
        Ok(OpenedTag::SubProcess {
            global_index: index,
            id,
            name,
            elements: vec![],
            draft_sequence_flows: vec![],
        })
    }
}

impl Closeable for TagSubProcess {
    fn close_tag(opened_tag: OpenedTag, _e: &BytesEnd, state: &mut ParserState) -> Result<()> {
        if let OpenedTag::SubProcess {
            global_index,
            id,
            name,
            mut elements,
            draft_sequence_flows,
        } = opened_tag
        {
            //process the internal sequence flows
            let sequence_flows =
                process_internal_sequence_flows!(draft_sequence_flows, elements, state);

            match state.open_tags.iter_mut().last() {
                Some(OpenedTag::Process {
                    elements: super_elements,
                    ..
                })
                | Some(OpenedTag::SubProcess {
                    elements: super_elements,
                    ..
                }) => {
                    if elements.is_empty() {
                        //create a collapsed sub-process
                        let local_index = super_elements.len();
                        super_elements.push(BPMNElement::CollapsedSubProcess(
                            BPMNCollapsedSubProcess {
                                global_index,
                                id,
                                local_index,
                                activity: state
                                    .activity_key
                                    .process_activity(&name.unwrap_or("".to_string())),
                                incoming_sequence_flows: vec![],
                                outgoing_sequence_flows: vec![],
                                incoming_message_flows: vec![],
                                outgoing_message_flows: vec![],
                            },
                        ));
                    } else {
                        //create an expanded sub-process
                        let local_index = super_elements.len();
                        super_elements.push(BPMNElement::ExpandedSubProcess(
                            BPMNExpandedSubProcess {
                                global_index,
                                id,
                                local_index,
                                name,
                                elements,
                                sequence_flows,
                                incoming_sequence_flows: vec![],
                                outgoing_sequence_flows: vec![],
                            },
                        ));
                    }
                    Ok(())
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }
}
