use crate::{
    importer::parse_attribute,
    parser::{
        parser::NameSpace,
        parser_state::ParserState,
        parser_traits::{Closeable, Openable, Recognisable},
        tags::{OpenedTag, Tag},
    },
};
use anyhow::{Context, Result, anyhow};
use quick_xml::events::{BytesEnd, BytesStart};

pub(crate) struct TagWeight {}

impl Recognisable for TagWeight {
    fn recognise_tag(e: &BytesStart, state: &ParserState, n: NameSpace) -> Option<Tag>
    where
        Self: Sized,
    {
        if n.is_sbpmn() {
            match state.open_tags.iter().last() {
                Some(OpenedTag::SequenceFlow { .. }) => {
                    if e.local_name().as_ref() == b"weight" {
                        return Some(Tag::Weight);
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

impl Openable for TagWeight {
    fn open_tag(_tag: Tag, e: &BytesStart, _state: &mut ParserState) -> Result<OpenedTag>
    where
        Self: Sized,
    {
        let weight_string = parse_attribute(e, "constant")
            .ok_or_else(|| anyhow!("A `constant` attribute is mandatory on a weight tag."))?;
        let weight = weight_string
            .parse()
            .with_context(|| anyhow!("Parsing weight as fraction."))?;
        Ok(OpenedTag::Weight { weight })
    }
}

impl Closeable for TagWeight {
    fn close_tag(opened_tag: OpenedTag, _e: &BytesEnd, state: &mut ParserState) -> Result<()> {
        match state.open_tags.iter_mut().last() {
            Some(OpenedTag::SequenceFlow {
                weight: seq_weight, ..
            }) => {
                if let OpenedTag::Weight { weight } = opened_tag {
                    if seq_weight.is_some() {
                        return Err(anyhow!("Cannot assign two weights to a sequence flow."));
                    }
                    *seq_weight = Some(weight);
                    Ok(())
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}
