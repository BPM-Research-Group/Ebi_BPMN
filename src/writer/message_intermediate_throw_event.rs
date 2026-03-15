use crate::{
    BusinessProcessModelAndNotation,
    elements::message_intermediate_throw_event::BPMNMessageIntermediateThrowEvent,
    traits::{processable::Processable, writable::{Writable, write_external_sequence_flows}},
};
use quick_xml::events::{BytesStart, BytesText, Event};

impl Writable for BPMNMessageIntermediateThrowEvent {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        x.create_element("intermediateThrowEvent")
            .with_attribute(("id", self.id.as_str()))
            .write_inner_content(|x| {
                write_external_sequence_flows!(x, self, parent);
                let mut b = BytesStart::new("messageEventDefinition");
                if let Some(id) = &self.message_marker_id {
                    b = b.with_attributes([("id", id.as_str())]);
                }
                x.write_event(Event::Empty(b))?;
                Ok(())
            })?;
        Ok(())
    }
}
