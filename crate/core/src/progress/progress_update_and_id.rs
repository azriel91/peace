use serde::{Deserialize, Serialize};

use crate::{
    progress::{ProgressMsgUpdate, ProgressUpdate},
    ItemId,
};

/// An item ID and its progress update.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProgressUpdateAndId {
    /// ID of the item whose progress is updated.
    pub item_id: ItemId,
    /// Delta update for the progress tracker.
    pub progress_update: ProgressUpdate,
    /// Whether to change the progress message.
    pub msg_update: ProgressMsgUpdate,
}
