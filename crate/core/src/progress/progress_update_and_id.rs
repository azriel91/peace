use serde::{Deserialize, Serialize};

use crate::progress::{ProgressMsgUpdate, ProgressUpdate};

/// An item ID and its progress update.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProgressUpdateAndId<ItemIdT> {
    /// ID of the item whose progress is updated.
    pub item_id: ItemIdT,
    /// Delta update for the progress tracker.
    pub progress_update: ProgressUpdate,
    /// Whether to change the progress message.
    pub msg_update: ProgressMsgUpdate,
}
