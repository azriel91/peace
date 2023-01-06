use serde::{Deserialize, Serialize};

use crate::{progress::ProgressIncrement, ItemSpecId};

/// Message to update the `OutputWrite`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProgressUpdate {
    /// ID of the item spec whose progress is updated.
    pub item_spec_id: ItemSpecId,
    /// The amount that the item spec progressed by.
    pub increment: ProgressIncrement,
}
