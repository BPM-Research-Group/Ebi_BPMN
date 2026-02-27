use crate::{
    BusinessProcessModelAndNotation,
    elements::timer_intermediate_catch_event::BPMNTimerIntermediateCatchEvent,
    traits::{processable::Processable, writable::Writable},
    write_external_sequence_flows,
};
use quick_xml::events::{BytesStart, BytesText, Event};

impl Writable for BPMNTimerIntermediateCatchEvent {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        x.create_element("intermediateCatchEvent")
            .with_attribute(("id", self.id.as_str()))
            .write_inner_content(|x| {
                write_external_sequence_flows!(x, self, parent);
                x.write_event(Event::Empty(
                    BytesStart::new("timerEventDefinition")
                        .with_attributes([("id", self.timer_marker_id.as_str())]),
                ))?;
                Ok(())
            })?;
        Ok(())
    }
}
