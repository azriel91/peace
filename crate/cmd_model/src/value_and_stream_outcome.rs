use fn_graph::StreamOutcome;

/// `CmdBlock` outcome value on success, and its `StreamOutcome` if applicable.
#[derive(Debug)]
pub struct ValueAndStreamOutcome<T> {
    /// The value returned by the `CmdBlock`.
    pub value: T,
    /// If the block streams each item in its logic, then this contains the
    /// stream outcome.
    pub stream_outcome: Option<StreamOutcome<()>>,
}
