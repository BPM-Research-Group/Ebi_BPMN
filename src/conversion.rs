use crate::{
    BusinessProcessModelAndNotation,
    stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation,
};
use anyhow::{Context, Error, anyhow};

impl From<StochasticBusinessProcessModelAndNotation> for BusinessProcessModelAndNotation {
    fn from(value: StochasticBusinessProcessModelAndNotation) -> Self {
        value.bpmn
    }
}

impl TryFrom<BusinessProcessModelAndNotation> for StochasticBusinessProcessModelAndNotation {
    type Error = Error;

    /// Attempt to transform a BPMN model into an SBPMN model.
    /// This is possible if the appropriate sequence flows are annotated with weights.
    fn try_from(mut value: BusinessProcessModelAndNotation) -> Result<Self, Self::Error> {
        value.stochastic_namespace = true;
        let result = Self { bpmn: value };
        result
            .is_structurally_correct()
            .with_context(|| anyhow!("Verifying structural correctness."))?;
        Ok(result)
    }
}
