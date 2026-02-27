use crate::{
    BusinessProcessModelAndNotation,
    elements::collapsed_pool::BPMNCollapsedPool,
    traits::{processable::Processable, writable::Writable},
};

impl Writable for BPMNCollapsedPool {
    fn write<W: std::io::Write>(
        &self,
        _x: &mut quick_xml::Writer<W>,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
