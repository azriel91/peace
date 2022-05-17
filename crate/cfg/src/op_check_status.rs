/// Whether an operation needs to be executed.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCheckStatus {
    /// Operation is not in desired state.
    ExecRequired,
    /// Operation is already in desired state.
    ExecNotRequired,
}
