use crate::{
    BusinessProcessModelAndNotation,
    elements::participant::BPMNParticipant,
    traits::{processable::Processable, writable::Writable},
};

impl Writable for BPMNParticipant {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        let mut el = x.create_element("participant").with_attributes([
            ("id", self.id.as_str()),
            ("processRef", self.process_id.as_str()),
        ]);

        if let Some(name) = &self.name {
            el = el.with_attributes([("name", name.as_str())]);
        }

        el.write_empty()?;
        Ok(())
    }
}
