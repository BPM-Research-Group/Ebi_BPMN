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
    /// Attempts to import a BPMN model. If `disallow_sequence_flow_weights` is set to true, parsing will fail if any sequence flow has a weight.
    /// Will only succeed if the model is structurally correct.
    pub fn import_from_reader(
        reader: &mut dyn BufRead,
        disallow_sequence_flow_weights: bool,
    ) -> Result<Self>
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
                .with_context(|| "Cannot read XML event.")?;
            let in_namespace = is_in_namespace(namespace);
            match (in_namespace, xml_event) {
                //start tag
                (Some(n), Event::Start(e)) => {
                    open_tag(&mut state, &e, n).with_context(|| {
                        format!(
                            "Tag `{}` at position {}.",
                            String::from_utf8_lossy(e.local_name().as_ref()),
                            xml_reader.buffer_position()
                        )
                    })?;
                }

                //end of tag
                (Some(n), Event::End(e)) => close_tag(&mut state, &e, n).with_context(|| {
                    format!(
                        "Tag `{}` at position {}.",
                        String::from_utf8_lossy(e.local_name().as_ref()),
                        xml_reader.buffer_position()
                    )
                })?,

                //empty tag
                (Some(n), Event::Empty(e)) => empty_tag(&mut state, &e, n).with_context(|| {
                    format!(
                        "Tag `{}` at position {}.",
                        String::from_utf8_lossy(e.local_name().as_ref()),
                        xml_reader.buffer_position()
                    )
                })?,

                //end of file: check whether we can finish
                (_, Event::Eof) => {
                    can_eof(&state).with_context(|| "Unexpected end of file.")?;
                    return Ok(state.to_model(disallow_sequence_flow_weights)?);
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
        Self::import_from_reader(&mut reader, false)
    }
}

pub(crate) fn parse_attribute(e: &BytesStart, attribute_name: &str) -> Option<String> {
    if let Ok(Some(attribute)) = e.try_get_attribute(attribute_name) {
        Some(
            attribute
                .decode_and_unescape_value(e.decoder())
                .ok()?
                .as_ref()
                .to_owned(),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BusinessProcessModelAndNotation, semantics::tests::debug_transitions,
        stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
        traits::processable::Processable,
    };
    use std::fs::{self};

    #[test]
    fn bpmn_import() {
        let fin = fs::read_to_string("testfiles/model.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        assert_eq!(bpmn.sequence_flows_non_recursive().len(), 0);
        assert_eq!(bpmn.elements().len(), 10);
    }

    #[test]
    fn sbpmn_import() {
        let fin = fs::read_to_string("testfiles/model.sbpmn").unwrap();
        let _bpmn = fin
            .parse::<StochasticBusinessProcessModelAndNotation>()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn sbpmn_import_fail() {
        let fin = fs::read_to_string("testfiles/model-lanes.bpmn").unwrap();
        let bpmn = fin
            .parse::<StochasticBusinessProcessModelAndNotation>()
            .unwrap();

        dbg!(bpmn);
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

    #[test]
    fn sbpmn_import_zero_weight() {
        let fin = fs::read_to_string("testfiles/model-zeroweight.sbpmn").unwrap();
        let sbpmn = fin
            .parse::<StochasticBusinessProcessModelAndNotation>()
            .unwrap();

        dbg!(&sbpmn);

        let mut marking = sbpmn.get_initial_marking().unwrap().unwrap();
        assert_eq!(sbpmn.number_of_transitions(&marking), 13);
        debug_transitions(&sbpmn.bpmn, &marking);

        let enabled = sbpmn.get_enabled_transitions(&marking).unwrap();
        assert_eq!(enabled, [0]);

        //execute start event
        sbpmn.execute_transition(&mut marking, 0).unwrap();
        assert_eq!(sbpmn.get_enabled_transitions(&marking).unwrap(), [1]);

        //execute task
        assert_eq!(
            sbpmn
                .bpmn
                .activity_key
                .deprocess_activity(&sbpmn.get_transition_activity(1, &marking).unwrap()),
            "Register claim\n(2min)"
        );
        sbpmn.execute_transition(&mut marking, 1).unwrap();
        assert_eq!(sbpmn.get_enabled_transitions(&marking).unwrap(), [3]);

        //execute XOR split
        sbpmn.is_transition_silent(3, &marking);
        sbpmn.execute_transition(&mut marking, 3).unwrap();
        assert_eq!(sbpmn.get_enabled_transitions(&marking).unwrap(), [5]);
    }

    #[test]
    fn recourse() {
        // Test case kindly provided by Camunda at https://github.com/camunda/bpmn-for-research
        let fin = fs::read_to_string("testfiles/recourse.bpmn").unwrap();
        let _bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    }

    #[test]
    #[should_panic]
    fn credit_scoring_sync() {
        // Test case kindly provided by Camunda at https://github.com/camunda/bpmn-for-research
        let fin = fs::read_to_string("testfiles/credit-scoring-synchronous.bpmn").unwrap();
        let _bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    }

    #[test]
    #[should_panic]
    fn credit_scoring_async() {
        // Test case kindly provided by Camunda at https://github.com/camunda/bpmn-for-research
        let fin = fs::read_to_string("testfiles/credit-scoring-asynchronous.bpmn").unwrap();
        let _bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    }

    #[test]
    #[should_panic]
    fn camunda_dispatch_of_goods() {
        // Test case kindly provided by Camunda at https://github.com/camunda/bpmn-for-research
        let fin = fs::read_to_string("testfiles/Exercise1_18550dfe434d4b1ab796685f63d461f5.bpmn")
            .unwrap();
        let _bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    }

    #[test]
    fn camunda_recourse() {
        // Test case kindly provided by Camunda at https://github.com/camunda/bpmn-for-research
        let fin =
            fs::read_to_string("testfiles/Ex_3_906bd7c47cfb4257ac43aa297c2da4cf.bpmn").unwrap();
        let _bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    }

    #[test]
    fn camunda_recourse_2() {
        // Test case kindly provided by Camunda at https://github.com/camunda/bpmn-for-research
        let fin =
            fs::read_to_string("testfiles/ex_5_c70fdbb34707491aa86dfa0e8cf7ab74.bpmn").unwrap();
        let _bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();
    }
}
