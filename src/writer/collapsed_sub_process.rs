use crate::{
    BusinessProcessModelAndNotation,
    elements::collapsed_sub_process::BPMNCollapsedSubProcess,
    traits::{processable::Processable, writable::Writable},
    write_external_sequence_flows,
};
use quick_xml::events::BytesText;

impl Writable for BPMNCollapsedSubProcess {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        x.create_element("subProcess")
            .with_attributes([
                ("id", self.id.as_str()),
                ("name", bpmn.activity_key.deprocess_activity(&self.activity)),
            ])
            .write_inner_content(|x| {
                write_external_sequence_flows!(x, self, parent);
                Ok(())
            })?;
        Ok(())
    }
}
