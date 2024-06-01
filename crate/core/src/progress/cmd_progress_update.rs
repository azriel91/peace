use serde::{Deserialize, Serialize};

use crate::progress::ProgressUpdateAndId;

/// Progress update that affects all `ProgressTracker`s.
///
/// This is sent at the `CmdExecution` level.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CmdProgressUpdate {
    /// `ProgressUpdateAndId` for a single step.
    ///
    /// # Design Note
    ///
    /// This isn't a tuple newtype as `serde_yaml` `0.9` is unable to serialize
    /// newtype enum variants.
    Step {
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
        Self::Step {
            progress_update_and_id,
        }
    }
}
