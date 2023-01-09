use serde::{Deserialize, Serialize};

use crate::progress::ProgressComplete;

/// Status of the execution.
///
/// # Implementation Note
///
/// The following variant is possible conceptually, but not applicable to the
/// Peace framework:
///
/// `Stopped`: Task is not running, but can be started.
///
/// This is not applicable because Peace uses runtime borrowing to manage state,
/// and a stopped task has potentially altered data non-atomically, so locking
/// the data is not useful, and unlocking the data may cause undefined behaviour
/// due to reasoning over inconsistent state.
///
/// For rate limiting tasks, the task in its entirety would be held back.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgressStatus {
    /// Execution is pending a predecessor's execution.
    ExecPending,
    /// Execution is in progress.
    ///
    /// This status is best conveyed alongside additional information:
    ///
    /// * **Units total:** Unknown (spinner) / known (progress bar).
    /// * **Units current**
    /// * **Operation:** State current / desired / diff discovery,
    ///   `EnsureOpSpec::exec`.
    ///
    ///     Certain operations will not be applicable, e.g. when `StateCurrent`
    ///     is feature gated, then the operation won't be available when the
    ///     feature is not enabled.
    Running,
    /// Progress updates have not been received for a given period.
    ///
    /// Item spec implementations are responsible for sending progress updates,
    /// but if there are no progress updates, or an identical "it's running"
    /// progress update is continuously received, then Peace may determine that
    /// the task may have stalled, and user attention is required.
    ///
    /// Peace may also provide a hook for implementors to output a suggestion to
    /// the user.
    RunningStalled,
    /// Task is pending user input.
    UserPending,
    /// Task has completed.
    ///
    /// This status is best conveyed alongside additional information:
    ///
    /// * **Completion Status**: Success, Failed.
    /// * **Operation:** State current / desired / diff discovery,
    ///   `EnsureOpSpec::exec`.
    Complete(ProgressComplete),
}
