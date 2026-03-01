use crate::{
    BusinessProcessModelAndNotation,
    parser::{
        parser::{can_eof, close_tag, empty_tag, is_in_namespace, open_tag},
        parser_state::ParserState,
    },
};
use anyhow::{Context, Error, Result};
use quick_xml::{
    NsReader,
    events::{BytesStart, Event},
};
use std::{io::BufRead, str::FromStr};

impl BusinessProcessModelAndNotation {
    pub fn import_from_reader(reader: &mut dyn BufRead) -> Result<Self>
    where
        Self: Sized,
    {
        let mut xml_reader = NsReader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        let mut buf = vec![];
        let mut state = ParserState::new();
        loop {
            buf.clear();
            let (namespace, xml_event) = xml_reader
                .read_resolved_event_into(&mut buf)
                .with_context(|| "cannot read XML event")?;
            let in_namespace = is_in_namespace(namespace);
            match (in_namespace, xml_event) {
                //start tag
                (true, Event::Start(e)) => {
                    open_tag(&mut state, &e).with_context(|| {
                        format!(
                            "start tag `{}` at position {}",
                            String::from_utf8_lossy(e.local_name().as_ref()),
                            xml_reader.buffer_position()
                        )
                    })?;
                }

                //end of tag
                (true, Event::End(e)) => close_tag(&mut state, &e).with_context(|| {
                    format!(
                        "close tag `{}` at position {}",
                        String::from_utf8_lossy(e.local_name().as_ref()),
                        xml_reader.buffer_position()
                    )
                })?,

                //empty tag
                (true, Event::Empty(e)) => empty_tag(&mut state, &e).with_context(|| {
                    format!(
                        "empty tag `{}` at position {}",
                        String::from_utf8_lossy(e.local_name().as_ref()),
                        xml_reader.buffer_position()
                    )
                })?,

                //end of file: check whether we can finish
                (_, Event::Eof) => {
                    can_eof(&state).with_context(|| "unexpected end of file")?;
                    return Ok(state.to_model()?);
                }

                _ => (),
            }
        }
    }
}

impl FromStr for BusinessProcessModelAndNotation {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut reader = std::io::Cursor::new(s);
        Self::import_from_reader(&mut reader)
    }
}

pub(crate) fn parse_attribute(e: &BytesStart, attribute_name: &str) -> Option<String> {
    if let Ok(Some(attribute)) = e.try_get_attribute(attribute_name) {
        Some(
            String::from_utf8_lossy(&attribute.value)
                .as_ref()
                .to_owned(),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{BusinessProcessModelAndNotation, traits::processable::Processable};
    use std::fs::{self};

    #[test]
    fn bpmn_import() {
        let fin = fs::read_to_string("testfiles/model.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        assert_eq!(bpmn.sequence_flows_non_recursive().len(), 0);
        assert_eq!(bpmn.all_elements_ref().len(), 10);
    }

    #[test]
    #[should_panic]
    fn bpmn_pool_invalid() {
        let fin = fs::read_to_string("testfiles/invalid-pool.bpmn").unwrap();
        let _bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    }

    #[test]
    #[should_panic]
    fn bpmn_message_invalid() {
        let fin = fs::read_to_string("testfiles/invalid-message.bpmn").unwrap();
        fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    }

    #[test]
    fn bpmn_lanes_import() {
        let fin = fs::read_to_string("testfiles/model-lanes.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        assert_eq!(bpmn.elements.len(), 2);
        assert_eq!(bpmn.message_flows.len(), 1);
    }
}
