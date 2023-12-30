use serde::{Deserialize, Serialize};

/// Progress update that affects all `ProgressTracker`s.
///
/// This is sent at the `CmdExecution` level.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CmdProgressUpdate {
    /// `CmdExecution` has been interrupted, we should indicate this on all
    /// unstarted progress bars.
    Interrupt,
    /// We are in between `CmdBlock`s, set all progress bars to `ExecPending`.
    ResetToPending,
}
