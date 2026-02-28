use crate::{
    BusinessProcessModelAndNotation, element::BPMNElement,
    elements::collapsed_pool::BPMNCollapsedPool, message_flow::BPMNMessageFlow,
    sequence_flow::BPMNSequenceFlow, traits::processable::Processable,
};
use anyhow::Result;
use quick_xml::Writer;
use std::io::Write;

/// Methods to write to XML
pub(crate) trait Writable {
    fn write<W: Write>(
        &self,
        x: &mut Writer<W>,
        parent: &dyn Processable,
        bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()>;
}

macro_rules! vec_writable {
    ($t:ty) => {
        impl Writable for $t {
            fn write<W: Write>(
                &self,
                x: &mut Writer<W>,
                parent: &dyn Processable,
                bpmn: &BusinessProcessModelAndNotation,
            ) -> Result<()> {
                for element in self {
                    element.write(x, parent, bpmn)?;
                }
                Ok(())
            }
        }
    };
}

vec_writable!(Vec<BPMNElement>);
vec_writable!(Vec<BPMNMessageFlow>);
vec_writable!(Vec<&BPMNCollapsedPool>);
vec_writable!(Vec<BPMNSequenceFlow>);

#[macro_export]
macro_rules! write_external_incoming {
    ($x: ident, $self:ident, $parent:ident) => {
        for incoming_sequence_flow in &$self.incoming_sequence_flows {
            $x.create_element("incoming")
                .write_text_content(BytesText::new(
                    &$parent.sequence_flows_non_recursive()[*incoming_sequence_flow].id,
                ))?;
        }
    };
}

#[macro_export]
macro_rules! write_external_outgoing {
    ($x: ident, $self:ident, $parent:ident) => {
        for outgoing_sequence_flow in &$self.outgoing_sequence_flows {
            $x.create_element("outgoing")
                .write_text_content(BytesText::new(
                    &$parent.sequence_flows_non_recursive()[*outgoing_sequence_flow].id,
                ))?;
        }
    };
}

#[macro_export]
macro_rules! write_external_sequence_flows {
    ($x: ident, $self:ident, $parent:ident) => {
        for incoming_sequence_flow in &$self.incoming_sequence_flows {
            $x.create_element("incoming")
                .write_text_content(BytesText::new(
                    &$parent.sequence_flows_non_recursive()[*incoming_sequence_flow].id,
                ))?;
        }
        for outgoing_sequence_flow in &$self.outgoing_sequence_flows {
            $x.create_element("outgoing")
                .write_text_content(BytesText::new(
                    &$parent.sequence_flows_non_recursive()[*outgoing_sequence_flow].id,
                ))?;
        }
    };
}
