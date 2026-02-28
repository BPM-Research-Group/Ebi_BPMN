use crate::{
    element::{BPMNElement, BPMNElementTrait},
    elements::process::BPMNProcess,
    parser::{
        parser_state::ParserState,
        parser_traits::{Closeable, Openable, Recognisable},
        tag_sequence_flow::DraftSequenceFlow,
        tags::{OpenedTag, Tag},
    },
    sequence_flow::BPMNSequenceFlow,
    traits::searchable::Searchable,
};
use anyhow::{Context, Result, anyhow};
use quick_xml::events::{BytesEnd, BytesStart};

pub(crate) struct TagProcess {}

impl Recognisable for TagProcess {
    fn recognise_tag(e: &BytesStart, state: &ParserState) -> Option<Tag>
    where
        Self: Sized,
    {
        match state.open_tags.iter().last() {
            Some(OpenedTag::Definitions { .. }) => {
                if e.local_name().as_ref() == b"process" {
                    return Some(Tag::Process);
                }
            }
            _ => {}
        }
        None
    }
}

impl Openable for TagProcess {
    fn open_tag(_tag: Tag, e: &BytesStart, state: &mut ParserState) -> Result<OpenedTag>
    where
        Self: Sized,
    {
        let (index, id) = state.read_and_add_id(e)?;

        Ok(OpenedTag::Process {
            global_index: index,
            id,
            elements: vec![],
            draft_sequence_flows: vec![],
        })
    }
}

#[macro_export]
macro_rules! process_internal_sequence_flows {
    ($draft_sequence_flows:ident, $sub_elements:ident, $state:ident) => {
        {
            let mut sequence_flows = Vec::with_capacity($draft_sequence_flows.len());
            for draft_sequence_flow in $draft_sequence_flows {
                let DraftSequenceFlow {
                    global_index,
                    id,
                    source_id,
                    target_id,
                } = draft_sequence_flow;
                let new_flow_index = sequence_flows.len();
                let source_index = $sub_elements
                    .id_2_local_index(&source_id)
                    .ok_or_else(|| {
                        //attempt to give a more helpful error with other found tags
                        if let Some(tag) = $state.not_recognised_id_2_tag.get(&source_id) {
                            anyhow!(
                                "Could not find source `{}` of sequence flow `{}`.\nHowever, a tag with name `{}` was found with this id. That tag is perhaps not supported or is not in an expected location.",
                                source_id,
                                id,
                                tag
                            )
                        } else {
                            anyhow!(
                                "Could not find source `{}` of sequence flow `{}`.",
                                source_id,
                                id
                            )
                        }
                    })?;
                //register the sequence flow in the source element
                let source =
                    $sub_elements
                        .local_index_2_element_mut(source_index)
                        .ok_or_else(|| {
                            anyhow!(
                                "Could not find source with id `{}` of sequence flow `{}`.",
                                source_id,
                                id
                            )
                        })?;
                source
                    .add_outgoing_sequence_flow(new_flow_index)
                    .with_context(|| {
                        anyhow!(
                            "Could not add sequence flow `{}` to its source element `{}`.",
                            id,
                            source_id,
                        )
                    })?;

                let target_index = $sub_elements
                    .id_2_local_index(&target_id)
                    .ok_or_else(|| {
                        //attempt to give a more helpful error message
                        if let Some(tag) = $state.not_recognised_id_2_tag.get(&target_id) {
                            anyhow!(
                                "Could not find target `{}` of sequence flow `{}`.\nHowever, a tag with name `{}` was found with this id. That tag is perhaps not supported or is not in an expected location.",
                                target_id,
                                id,
                                tag
                            )
                        } else {
                        anyhow!(
                            "Could not find target `{}` of sequence flow `{}`.",
                            target_id,
                            id
                        )
                    }
                    })?;
                //register the sequence flow in the target element
                let target = $sub_elements.local_index_2_element_mut(target_index).ok_or_else(
                    || {
                        anyhow!(
                            "Could not target `{}` of sequence flow `{}`.",
                            source_id,
                            id
                        )
                    },
                )?;
                target
                    .add_incoming_sequence_flow(new_flow_index)
                    .with_context(|| {
                        anyhow!(
                            "Could not add sequence flow `{}` to its target element `{}`",
                            id,
                            target_id,
                        )
                    })?;

                sequence_flows.push(BPMNSequenceFlow {
                    global_index,
                    id,
                    flow_index: new_flow_index,
                    source_index,
                    target_index,
                });
            }
            sequence_flows
        }
    };
}

impl Closeable for TagProcess {
    fn close_tag(opened_tag: OpenedTag, _e: &BytesEnd, state: &mut ParserState) -> Result<()> {
        if let OpenedTag::Process {
            global_index,
            id,
            elements: mut sub_elements,
            draft_sequence_flows,
        } = opened_tag
        {
            if let Some(OpenedTag::Definitions {
                elements: super_elements,
                ..
            }) = state.open_tags.iter_mut().last()
            {
                //process internal sequence flows
                let sequence_flows =
                    process_internal_sequence_flows!(draft_sequence_flows, sub_elements, state);

                //create a process
                let local_index = super_elements.len();
                super_elements.push(BPMNElement::Process(BPMNProcess {
                    global_index,
                    id,
                    local_index,
                    elements: sub_elements,
                    sequence_flows,
                }));
                Ok(())
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}
