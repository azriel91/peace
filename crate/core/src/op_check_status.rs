use serde::{Deserialize, Serialize};

#[cfg(feature = "output_progress")]
use crate::progress::ProgressLimit;

/// Whether an operation needs to be executed.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum OpCheckStatus {
    /// Operation is not in desired state.
    #[cfg(not(feature = "output_progress"))]
    ExecRequired,
    /// Operation is not in desired state.
    #[cfg(feature = "output_progress")]
    ExecRequired {
        /// Unit of measurement and limit to indicate progress.
        progress_limit: ProgressLimit,
    },
    /// Operation is already in desired state.
    ExecNotRequired,
}
