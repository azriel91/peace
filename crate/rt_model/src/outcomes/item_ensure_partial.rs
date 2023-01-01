use std::fmt::{Debug, Display};

use peace_cfg::{state::Placeholder, OpCheckStatus, State};
use peace_resources::type_reg::untagged::{DataType, DataTypeDisplay};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::outcomes::ItemEnsurePartialRt;

/// Information about an item during an `EnsureCmd` execution.
///
/// # Design Note
///
/// 1. `EnsureCmd` calls the following function for each item.
///
///     - [`StateCurrentFnSpec::exec`]
///     - [`StateDesiredFnSpec::exec`]
///     - [`StateDiffFnSpec::exec`]
///     - [`EnsureOpSpec::check`]
///     - [`EnsureOpSpec::exec`]
///     - [`StateCurrentFnSpec::exec`]
///
/// 2. Each function call *may* fail.
/// 3. If we have an enum representing the state after each function call, we
/// have to duplicate the earlier fields per variant.
///
/// It is not likely to be error prone or too unergonomic to store each field as
/// optional.
///
/// [`StateCurrentFnSpec::exec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
/// [`StateDesiredFnSpec::exec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
/// [`StateDiffFnSpec::exec`]: peace_cfg::ItemSpec::StateDiffFnSpec
/// [`EnsureOpSpec::check`]: peace_cfg::ItemSpec::EnsureOpSpec
/// [`EnsureOpSpec::exec`]: peace_cfg::ItemSpec::EnsureOpSpec
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemEnsurePartial<StateLogical, StatePhysical, StateDiff> {
    /// State saved on disk before the execution.
    pub state_saved: Option<State<StateLogical, StatePhysical>>,
    /// Current state discovered during the execution.
    pub state_current: Option<State<StateLogical, StatePhysical>>,
    /// Desired state discovered during the execution.
    pub state_desired: Option<State<StateLogical, Placeholder>>,
    /// Diff between current and desired states.
    pub state_diff: Option<StateDiff>,
    /// Whether item execution is required.
    pub op_check_status: Option<OpCheckStatus>,
}

impl<StateLogical, StatePhysical, StateDiff>
    ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>
{
    /// Returns a new `ItemEnsurePartial` with all fields set to `None`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<StateLogical, StatePhysical, StateDiff> Default
    for ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>
{
    fn default() -> Self {
        Self {
            state_saved: None,
            state_current: None,
            state_desired: None,
            state_diff: None,
            op_check_status: None,
        }
    }
}

impl<StateLogical, StatePhysical, StateDiff> ItemEnsurePartialRt
    for ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>
where
    StateLogical: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn state_saved(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.state_saved
            .clone()
            .map(|state_saved| Box::new(state_saved) as Box<dyn DataTypeDisplay>)
    }

    fn state_current(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.state_current
            .clone()
            .map(|state_current| Box::new(state_current) as Box<dyn DataTypeDisplay>)
    }

    fn state_desired(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.state_desired
            .clone()
            .map(|state_desired| Box::new(state_desired) as Box<dyn DataTypeDisplay>)
    }

    fn state_diff(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.state_diff
            .clone()
            .map(|state_diff| Box::new(state_diff) as Box<dyn DataTypeDisplay>)
    }

    fn op_check_status(&self) -> Option<OpCheckStatus> {
        self.op_check_status
    }

    fn as_data_type(&self) -> &dyn DataType {
        self
    }

    fn as_data_type_mut(&mut self) -> &mut dyn DataType {
        self
    }
}
