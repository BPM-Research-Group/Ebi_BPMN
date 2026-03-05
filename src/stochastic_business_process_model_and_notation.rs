use anyhow::{Error, Result};
use std::{io::BufRead, str::FromStr};

use crate::BusinessProcessModelAndNotation;

pub struct StochasticBusinessProcessModelAndNotation {
    pub bpmn: BusinessProcessModelAndNotation,
}

impl StochasticBusinessProcessModelAndNotation {
    pub fn import_from_reader(reader: &mut dyn BufRead) -> Result<Self>
    where
        Self: Sized,
    {
        let bpmn = BusinessProcessModelAndNotation::import_from_reader(reader, false)?;
        let sbpmn = Self { bpmn };
        sbpmn.is_structurally_correct()?;
        Ok(sbpmn)
    }
}

impl FromStr for StochasticBusinessProcessModelAndNotation {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut reader = std::io::Cursor::new(s);
        Self::import_from_reader(&mut reader)
    }
}
