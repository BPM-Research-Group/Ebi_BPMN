use anyhow::{Result, anyhow};

pub trait IfNot<T> {
    fn if_not(self, error_message: &'static str) -> Result<T>;
}

impl<T> IfNot<T> for Option<T> {
    fn if_not(self, error: &'static str) -> Result<T> {
        self.ok_or_else(|| anyhow!(error))
    }
}
