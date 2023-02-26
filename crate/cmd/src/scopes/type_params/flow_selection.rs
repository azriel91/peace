use peace_core::FlowId;

/// A `FlowId` is not yet selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowNotSelected;

/// A `FlowId` is selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowSelected(pub(crate) FlowId);
