use crate::{
    BusinessProcessModelAndNotation,
    elements::message_end_event::BPMNMessageEndEvent,
    traits::{processable::Processable, writable::Writable},
    write_external_incoming,
};
use quick_xml::events::{BytesStart, BytesText, Event};

impl Writable for BPMNMessageEndEvent {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        x.create_element("endEvent")
            .with_attributes([("id", self.id.as_str())])
            .write_inner_content(|x| {
                write_external_incoming!(x, self, parent);
                x.write_event(Event::Empty(
                    BytesStart::new("messageEventDefinition")
                        .with_attributes([("id", self.message_marker_id.as_str())]),
                ))?;
                Ok(())
            })?;
        Ok(())
    }
}
