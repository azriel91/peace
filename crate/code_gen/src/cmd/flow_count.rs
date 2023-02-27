/// Number of flows accessed by this command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCount {
    /// No flows are accessed.
    None,
    /// One flow is accessed.
    One,
}
