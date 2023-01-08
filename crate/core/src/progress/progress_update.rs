use serde::{Deserialize, Serialize};

use crate::progress::{ProgressDelta, ProgressLimit};

/// Message to update the `OutputWrite`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgressUpdate {
    /// Progress limit has been discovered.
    Limit {
        /// The amount that the item spec progressed by.
        limit: ProgressLimit,
    },
    /// Progress units have changed.
    Delta {
        /// The amount that the item spec progressed by.
        delta: ProgressDelta,
    },
}
