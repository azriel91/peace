use crate::ProgressLimit;

/// Whether an operation needs to be executed.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCheckStatus {
    /// Operation is not in desired state.
    ExecRequired {
        /// Unit of measurement and limit to indicate progress.
        progress_limit: ProgressLimit,
    },
    /// Operation is already in desired state.
    ExecNotRequired,
}
