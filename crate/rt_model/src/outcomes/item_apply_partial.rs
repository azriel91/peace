use std::fmt::{Debug, Display};

use peace_cfg::ApplyCheck;
use peace_resource_rt::type_reg::untagged::{BoxDtDisplay, DataType};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::outcomes::ItemApplyPartialRt;

/// Information about an item during an `ApplyCmd` execution.
///
/// # Design Note
///
/// 1. `ApplyCmd` calls the following function for each item.
///
///     - [`Item::state_current`]
///     - [`Item::state_goal`] or [`Item::state_clean`]
///     - [`Item::state_diff`]
///     - [`ApplyFns::check`]
///     - [`ApplyFns::exec`]
///     - [`Item::state_current`]
///
/// 2. Each function call *may* fail.
/// 3. If we have an enum representing the state after each function call, we
///    have to duplicate the earlier fields per variant.
///
/// It is not likely to be error prone or too unergonomic to store each field as
/// optional.
///
/// [`Item::state_current`]: peace_cfg::Item::state_current
/// [`Item::state_goal`]: peace_cfg::Item::state_goal
/// [`Item::state_diff`]: peace_cfg::Item::state_diff
/// [`ApplyFns::check`]: peace_cfg::Item::ApplyFns
/// [`ApplyFns::exec`]: peace_cfg::Item::ApplyFns
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemApplyPartial<State, StateDiff> {
    /// Current state stored on disk before the execution.
    pub state_current_stored: Option<State>,
    /// Current state discovered during the execution.
    pub state_current: Option<State>,
    /// Target state discovered during the execution.
    pub state_target: Option<State>,
    /// Diff between current and goal states.
    pub state_diff: Option<StateDiff>,
    /// Whether item execution is required.
    pub apply_check: Option<ApplyCheck>,
}

impl<State, StateDiff> ItemApplyPartial<State, StateDiff> {
    /// Returns a new `ItemApplyPartial` with all fields set to `None`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<State, StateDiff> Default for ItemApplyPartial<State, StateDiff> {
    fn default() -> Self {
        Self {
            state_current_stored: None,
            state_current: None,
            state_target: None,
            state_diff: None,
            apply_check: None,
        }
    }
}

impl<State, StateDiff> ItemApplyPartialRt for ItemApplyPartial<State, StateDiff>
where
    State: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn state_current_stored(&self) -> Option<BoxDtDisplay> {
        self.state_current_stored.clone().map(BoxDtDisplay::new)
    }

    fn state_current(&self) -> Option<BoxDtDisplay> {
        self.state_current.clone().map(BoxDtDisplay::new)
    }

    fn state_target(&self) -> Option<BoxDtDisplay> {
        self.state_target.clone().map(BoxDtDisplay::new)
    }

    fn state_diff(&self) -> Option<BoxDtDisplay> {
        self.state_diff.clone().map(BoxDtDisplay::new)
    }

    fn apply_check(&self) -> Option<ApplyCheck> {
        self.apply_check
    }

    fn as_data_type(&self) -> &dyn DataType {
        self
    }

    fn as_data_type_mut(&mut self) -> &mut dyn DataType {
        self
    }
}
