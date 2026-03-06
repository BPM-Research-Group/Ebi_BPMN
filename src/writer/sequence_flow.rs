use crate::{
    BusinessProcessModelAndNotation,
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, processable::Processable, writable::Writable},
};
use quick_xml::events::{BytesEnd, BytesStart, Event};

impl Writable for BPMNSequenceFlow {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        let source_id = parent.elements_non_recursive()[self.source_local_index].id();
        let target_id = parent.elements_non_recursive()[self.target_local_index].id();

        let bytesstart = BytesStart::new("sequenceFlow").with_attributes([
            ("id", self.id.as_str()),
            ("sourceRef", source_id),
            ("targetRef", target_id),
        ]);

        if let Some(weight) = &self.weight {
            //with weight
            x.write_event(Event::Start(bytesstart))?;

            x.write_event(Event::Empty(
                BytesStart::new("sbpmn:weight")
                    .with_attributes([("constant", weight.to_string().as_str())]),
            ))?;

            x.write_event(Event::End(BytesEnd::new("sequenceFlow")))?;
        } else {
            //without weight
            x.write_event(Event::Empty(bytesstart))?;
        }

        Ok(())
    }
}
