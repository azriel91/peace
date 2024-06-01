use serde::{Deserialize, Serialize};

#[cfg(feature = "output_progress")]
use crate::progress::ProgressLimit;

/// Whether the `apply` function needs to be executed.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ApplyCheck {
    /// Step is not in goal state.
    #[cfg(not(feature = "output_progress"))]
    ExecRequired,
    /// Step is not in goal state.
    #[cfg(feature = "output_progress")]
    ExecRequired {
        /// Unit of measurement and limit to indicate progress.
        progress_limit: ProgressLimit,
    },
    /// Step is already in goal state.
    ExecNotRequired,
}
