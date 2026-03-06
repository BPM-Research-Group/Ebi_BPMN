use crate::{
    BusinessProcessModelAndNotation,
    elements::expanded_sub_process::BPMNExpandedSubProcess,
    traits::{processable::Processable, writable::Writable},
    write_external_sequence_flows,
};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

impl Writable for BPMNExpandedSubProcess {
    fn write<W: std::io::Write>(
        &self,
        x: &mut quick_xml::Writer<W>,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        let mut attributes = vec![("id", self.id.as_str())];
        if let Some(name) = &self.name {
            attributes.push(("name", name.as_str()));
        }

        x.write_event(Event::Start(
            BytesStart::new("subProcess").with_attributes(attributes),
        ))?;

        //external sequence flows
        write_external_sequence_flows!(x, self, parent);

        //internal sequence flows
        self.sequence_flows.write(x, self, bpmn)?;

        //recursive elements
        self.elements.write(x, self, bpmn)?;

        x.write_event(Event::End(BytesEnd::new("subProcess")))?;
        Ok(())
    }
}
