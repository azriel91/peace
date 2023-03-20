use serde::{Deserialize, Serialize};

use crate::{
    progress::{ProgressMsgUpdate, ProgressUpdate},
    ItemSpecId,
};

/// An item spec ID and its progress update.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProgressUpdateAndId {
    /// ID of the item spec whose progress is updated.
    pub item_spec_id: ItemSpecId,
    /// Delta update for the progress tracker.
    pub progress_update: ProgressUpdate,
    /// Whether to change the progress message.
    pub msg_update: ProgressMsgUpdate,
}
