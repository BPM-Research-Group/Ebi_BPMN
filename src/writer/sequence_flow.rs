use crate::{
    BusinessProcessModelAndNotation,
    sequence_flow::BPMNSequenceFlow,
    traits::{objectable::BPMNObject, processable::Processable, writable::Writable},
};
use quick_xml::events::{BytesStart, Event};

impl Writable for BPMNSequenceFlow {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        let source_id = parent.elements_non_recursive()[self.source_index].id();
        let target_id = parent.elements_non_recursive()[self.target_index].id();

        x.write_event(Event::Empty(
            BytesStart::new("sequenceFlow").with_attributes([
                ("id", self.id.as_str()),
                ("sourceRef", source_id),
                ("targetRef", target_id),
            ]),
        ))?;
        Ok(())
    }
}
