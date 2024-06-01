use peace_core::StepId;
use serde::{Deserialize, Serialize};

/// Serializable representation of how a [`Step`] is configured.
///
/// [`Step`]: https://docs.rs/peace_cfg/latest/peace_cfg/trait.Step.html
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StepSpecInfo {
    /// ID of the `Step`.
    pub step_id: StepId,
}

impl StepSpecInfo {
    /// Returns a new `StepSpecInfo`.
    pub fn new(step_id: StepId) -> Self {
        Self { step_id }
    }
}
