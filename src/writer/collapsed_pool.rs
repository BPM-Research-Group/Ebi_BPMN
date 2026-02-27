use crate::{elements::collapsed_pool::BPMNCollapsedPool, traits::writable::Writable};

impl Writable for BPMNCollapsedPool {
    fn write<W: std::io::Write>(
        &self,
        _x: &mut quick_xml::Writer<W>,
        _bpmn: &crate::BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
