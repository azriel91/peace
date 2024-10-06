use serde::{Deserialize, Serialize};

use crate::progress::ProgressUpdateAndId;

/// Progress update that affects all `ProgressTracker`s.
///
/// This is sent at the `CmdExecution` level.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CmdProgressUpdate {
    /// A `CmdBlock` has started.
    CmdBlockStart {
        /// The type of interactions the `CmdBlock` has with the
        /// `ItemLocation`s.
        cmd_block_item_interaction_type: CmdBlockItemInteractionType,
    },
    /// `ProgressUpdateAndId` for a single item.
    ///
    /// # Design Note
    ///
    /// This isn't a tuple newtype as `serde_yaml` `0.9` is unable to serialize
    /// newtype enum variants.
    Item {
        /// The update.
        progress_update_and_id: ProgressUpdateAndId,
    },
    /// `CmdExecution` has been interrupted, we should indicate this on all
    /// unstarted progress bars.
    Interrupt,
    /// We are in between `CmdBlock`s, set all progress bars to `ExecPending`.
    ResetToPending,
}

impl From<ProgressUpdateAndId> for CmdProgressUpdate {
    fn from(progress_update_and_id: ProgressUpdateAndId) -> Self {
        Self::Item {
            progress_update_and_id,
        }
    }
}
