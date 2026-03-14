use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElement,
    elements::collapsed_pool::BPMNCollapsedPool,
    stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
    traits::{objectable::BPMNObject, writable::Writable},
};
use anyhow::Result;
use quick_xml::{
    Writer,
    events::{BytesDecl, BytesEnd, BytesStart, Event},
};
use std::io::Write;

impl BusinessProcessModelAndNotation {
    /// Exports the model to a writer.
    pub fn export_to_writer(&self, f: &mut dyn Write) -> Result<()> {
        let mut x = Writer::new_with_indent(f, b'\t', 1);

        //XML declaration
        x.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

        //definitions
        let mut bytes_start = BytesStart::new("definitions").with_attributes([
            ("id", self.definitions_id.as_str()),
            ("xmlns", "http://www.omg.org/spec/BPMN/20100524/MODEL"),
            ("exporter", "Ebi-bpmn"),
        ]);
        if self.stochastic_namespace {
            bytes_start = bytes_start
                .with_attributes([(("xmlns:sbpmn", "https://www.ebitools.org/sbpmn/20260305"))]);
        }
        x.write_event(Event::Start(bytes_start))?;

        //collaboration
        if let Some(collaboration_id) = &self.collaboration_id {
            x.write_event(Event::Start(
                BytesStart::new("collaboration")
                    .with_attributes([("id", collaboration_id.as_str())]),
            ))?;

            //expanded pools
            self.elements
                .iter()
                .filter_map(|element| {
                    if let BPMNElement::CollapsedPool(p) = element {
                        Some(p)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .write(&mut x, self, self)?;

            //collapsed pools
            for element in &self.elements {
                if element.is_collapsed_pool() {
                    let mut el = x
                        .create_element("participant")
                        .with_attributes([("id", element.id())]);
                    if let BPMNElement::CollapsedPool(BPMNCollapsedPool {
                        name: Some(name), ..
                    }) = element
                    {
                        el = el.with_attribute(("name", name.as_str()));
                    }
                    el.write_empty()?;
                }
            }

            //messages
            self.message_flows.write(&mut x, self, self)?;

            x.write_event(Event::End(BytesEnd::new("collaboration")))?;
        }

        self.elements.write(&mut x, self, self)?;

        x.write_event(Event::End(BytesEnd::new("definitions")))?;

        Ok(())
    }
}

impl StochasticBusinessProcessModelAndNotation {
    pub fn export_to_writer(&self, f: &mut dyn Write) -> Result<()> {
        self.bpmn.export_to_writer(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BusinessProcessModelAndNotation,
        stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
    };
    use std::fs::{self};

    #[test]
    fn bpmn_export_import() {
        let fin = fs::read_to_string("testfiles/model-lanes.bpmn").unwrap();
        let bpmn = fin.parse::<BusinessProcessModelAndNotation>().unwrap();

        let mut f = vec![];
        bpmn.export_to_writer(&mut f).unwrap();

        let fout = String::from_utf8_lossy(&f);
        let _bpmn2 = fout.parse::<BusinessProcessModelAndNotation>();

        println!("{}", String::from_utf8_lossy(&f));
    }

    #[test]
    fn sbpmn_export_import() {
        let fin = fs::read_to_string("testfiles/model.sbpmn").unwrap();
        let sbpmn = fin
            .parse::<StochasticBusinessProcessModelAndNotation>()
            .unwrap();

        let mut f = vec![];
        sbpmn.export_to_writer(&mut f).unwrap();
        let fout = String::from_utf8_lossy(&f);
        let _sbpmn2 = fout.parse::<StochasticBusinessProcessModelAndNotation>().unwrap();

        println!("{}", String::from_utf8_lossy(&f));
    }

    #[test]
    fn sbpmn_export_import_or() {
        let fin = fs::read_to_string("testfiles/and-a-b-xor-c-or.sbpmn").unwrap();
        let sbpmn = fin
            .parse::<StochasticBusinessProcessModelAndNotation>()
            .unwrap();

        let mut f = vec![];
        sbpmn.export_to_writer(&mut f).unwrap();
        let fout = String::from_utf8_lossy(&f);
        let _sbpmn2 = fout.parse::<StochasticBusinessProcessModelAndNotation>().unwrap();

        println!("{}", String::from_utf8_lossy(&f));
    }
}
