use anyhow::{Result, anyhow};

use crate::{BPMNMessageFlow, BPMNSequenceFlow};

pub trait IfNot<T> {
    fn and_if_not(self, error_message: &'static str) -> Result<T>;
}

impl<T> IfNot<T> for Option<T> {
    fn and_if_not(self, error: &'static str) -> Result<T> {
        self.ok_or_else(|| anyhow!(error))
    }
}

pub trait IfNotDefault<T> {
    /// Transforms to a Result, and gives a defaulte message if the object is not present.
    fn and_if_not_error_default(self) -> Result<T>;
}

impl<'a> IfNotDefault<&'a BPMNMessageFlow> for Option<&'a BPMNMessageFlow> {
    fn and_if_not_error_default(self) -> Result<&'a BPMNMessageFlow> {
        self.ok_or_else(|| anyhow!("Message flow not found."))
    }
}

impl<'a> IfNotDefault<&'a BPMNSequenceFlow> for Option<&'a BPMNSequenceFlow> {
    fn and_if_not_error_default(self) -> Result<&'a BPMNSequenceFlow> {
        self.ok_or_else(|| anyhow!("Sequence flow not found."))
    }
}
