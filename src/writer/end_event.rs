use crate::{
    BusinessProcessModelAndNotation,
    elements::end_event::BPMNEndEvent,
    traits::{processable::Processable, writable::Writable},
    write_external_incoming,
};
use quick_xml::events::BytesText;

impl Writable for BPMNEndEvent {
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
                Ok(())
            })?;
        Ok(())
    }
}
