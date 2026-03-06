use anyhow::Error;

use crate::{
    BusinessProcessModelAndNotation,
    stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
};

impl From<StochasticBusinessProcessModelAndNotation> for BusinessProcessModelAndNotation {
    fn from(value: StochasticBusinessProcessModelAndNotation) -> Self {
        value.bpmn
    }
}

impl TryFrom<BusinessProcessModelAndNotation> for StochasticBusinessProcessModelAndNotation {
    type Error = Error;

    /// Attempt to transform a BPMN model into an SBPMN model.
    /// This is possible if the appropriate sequence flows are annotated with weights.
    fn try_from(value: BusinessProcessModelAndNotation) -> Result<Self, Self::Error> {
        let result = Self { bpmn: value };
        result.is_structurally_correct()?;
        Ok(result)
    }
}
