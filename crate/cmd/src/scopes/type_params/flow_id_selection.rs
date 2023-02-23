use peace_core::FlowId;

/// A `FlowId` is not yet selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowIdNotSelected;

/// A `FlowId` is selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowIdSelected(pub(crate) FlowId);
