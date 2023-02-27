use peace_rt_model::Flow;

/// A `Flow` is not yet selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowNotSelected;

/// A `Flow` is selected.
#[derive(Debug)]
pub struct FlowSelected<E>(pub(crate) Flow<E>);
