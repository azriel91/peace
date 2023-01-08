use serde::{Deserialize, Serialize};

use crate::{progress::ProgressUpdate, ItemSpecId};

/// An item spec ID and its progress update.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProgressUpdateAndId {
    /// ID of the item spec whose progress is updated.
    pub item_spec_id: ItemSpecId,
    /// Message to update the `OutputWrite`.
    pub progress_update: ProgressUpdate,
}
