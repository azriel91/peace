use serde::{Deserialize, Serialize};

use crate::progress::ProgressUpdateAndId;

/// Progress update that affects all `ProgressTracker`s.
///
/// This is sent at the `CmdExecution` level.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CmdProgressUpdate {
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
    /// The `CmdExecution` has received an interrupt request, but we haven't
    /// determined which items to stop.
    ///
    /// This variant broadcasts that progress trackers should indicate if their
    /// item is candidate to stop.
    InterruptPending,
    /// The `CmdExecution` has received an interrupt request, and we have
    /// determined which items will not be processed.
    ///
    /// This variant broadcasts that progress trackers should indicate if their
    /// item is candidate to stop.
    Interrupted,
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
