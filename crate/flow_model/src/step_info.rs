use peace_core::StepId;
use serde::{Deserialize, Serialize};

/// Serializable representation of values used for / produced by a [`Step`].
///
/// [`Step`]: https://docs.rs/peace_cfg/latest/peace_cfg/trait.Step.html
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StepInfo {
    /// ID of the `Step`.
    pub step_id: StepId,
}

impl StepInfo {
    /// Returns a new `StepInfo`.
    pub fn new(step_id: StepId) -> Self {
        Self { step_id }
    }
}
