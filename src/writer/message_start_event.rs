use crate::{
    BusinessProcessModelAndNotation,
    elements::message_start_event::BPMNMessageStartEvent,
    traits::{processable::Processable, writable::Writable}, write_external_outgoing,
};
use quick_xml::events::{BytesStart, BytesText, Event};

impl Writable for BPMNMessageStartEvent {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        x.create_element("startEvent")
            .with_attributes([("id", self.id.as_str())])
            .write_inner_content(|x| {
                write_external_outgoing!(x, self, parent);
                x.write_event(Event::Empty(
                    BytesStart::new("messageEventDefinition")
                        .with_attributes([("id", self.message_marker_id.as_str())]),
                ))?;
                Ok(())
            })?;
        Ok(())
    }
}
