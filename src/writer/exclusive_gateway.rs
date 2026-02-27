use crate::{
    BusinessProcessModelAndNotation,
    elements::exclusive_gateway::BPMNExclusiveGateway,
    traits::{processable::Processable, writable::Writable}, write_external_sequence_flows,
};
use quick_xml::events::BytesText;

impl Writable for BPMNExclusiveGateway {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        x.create_element("exclusiveGateway")
            .with_attribute(("id", self.id.as_str()))
            .write_inner_content(|x| {
                write_external_sequence_flows!(x, self, parent);
                Ok(())
            })?;
        Ok(())
    }
}
