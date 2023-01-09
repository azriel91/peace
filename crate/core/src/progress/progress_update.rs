use serde::{Deserialize, Serialize};

use crate::progress::{ProgressDelta, ProgressLimit};

use super::ProgressComplete;

/// Message to update the `OutputWrite`.
///
/// # Potential Future Variants
///
/// * `Interrupt`
/// * `PendingInput`
/// * `Stall`
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgressUpdate {
    /// Progress limit has been discovered.
    Limit(ProgressLimit),
    /// Progress units have changed.
    Delta(ProgressDelta),
    /// Execution has completed.
    Complete(ProgressComplete),
}
