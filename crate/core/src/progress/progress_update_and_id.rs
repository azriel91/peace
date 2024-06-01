use serde::{Deserialize, Serialize};

use crate::{
    progress::{ProgressMsgUpdate, ProgressUpdate},
    StepId,
};

/// An step ID and its progress update.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProgressUpdateAndId {
    /// ID of the step whose progress is updated.
    pub step_id: StepId,
    /// Delta update for the progress tracker.
    pub progress_update: ProgressUpdate,
    /// Whether to change the progress message.
    pub msg_update: ProgressMsgUpdate,
}
