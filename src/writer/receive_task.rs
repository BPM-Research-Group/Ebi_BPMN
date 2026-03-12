use crate::{
    BusinessProcessModelAndNotation, elements::receive_task::BPMNReceiveTask, traits::{processable::Processable, writable::Writable}, write_external_sequence_flows
};
use quick_xml::events::BytesText;

impl Writable for BPMNReceiveTask {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        x.create_element("receiveTask")
            .with_attribute(("id", self.id.as_str()))
            .with_attribute(("name", bpmn.activity_key.deprocess_activity(&self.activity)))
            .write_inner_content(|x| {
                write_external_sequence_flows!(x, self, parent);
                Ok(())
            })?;
        Ok(())
    }
}
