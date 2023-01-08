use serde::{Deserialize, Serialize};

use crate::{
    progress::{ProgressDelta, ProgressLimit},
    ItemSpecId,
};

/// Message to update the `OutputWrite`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgressUpdate {
    /// Progress limit has been discovered.
    Limit {
        /// ID of the item spec whose progress is updated.
        item_spec_id: ItemSpecId,
        /// The amount that the item spec progressed by.
        limit: ProgressLimit,
    },
    /// Progress units have changed.
    Delta {
        /// ID of the item spec whose progress is updated.
        item_spec_id: ItemSpecId,
        /// The amount that the item spec progressed by.
        delta: ProgressDelta,
    },
}
