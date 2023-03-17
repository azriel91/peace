use std::fmt::{Debug, Display};

use peace_cfg::OpCheckStatus;
use peace_resources::type_reg::untagged::{BoxDtDisplay, DataType};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::outcomes::{ItemApplyPartial, ItemApplyRt};

/// Information about an item during an `ApplyCmd` execution.
///
/// This is similar to [`ItemApplyPartial`], with most fields being
/// non-optional, and the added `state_applied` field.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemApply<State, StateDiff> {
    /// State saved on disk before the execution.
    pub state_saved: Option<State>,
    /// Current state discovered during the execution.
    pub state_current: State,
    /// Target state discovered during the execution.
    pub state_target: State,
    /// Diff between current and desired states.
    pub state_diff: StateDiff,
    /// Whether item execution was required.
    pub op_check_status: OpCheckStatus,
    /// The state that was applyd, `None` if execution was not required.
    pub state_applied: Option<State>,
}

impl<State, StateDiff> TryFrom<(ItemApplyPartial<State, StateDiff>, Option<State>)>
    for ItemApply<State, StateDiff>
{
    type Error = (ItemApplyPartial<State, StateDiff>, Option<State>);

    fn try_from(
        (partial, state_applied): (ItemApplyPartial<State, StateDiff>, Option<State>),
    ) -> Result<Self, Self::Error> {
        let ItemApplyPartial {
            state_saved,
            state_current,
            state_target,
            state_diff,
            op_check_status,
        } = partial;

        if state_current.is_some()
            && state_target.is_some()
            && state_diff.is_some()
            && op_check_status.is_some()
        {
            let (Some(state_current), Some(state_target), Some(state_diff), Some(op_check_status)) =
                (state_current, state_target, state_diff, op_check_status) else {
                    unreachable!("All are checked to be `Some` above.");
                };
            Ok(Self {
                state_saved,
                state_current,
                state_target,
                state_diff,
                op_check_status,
                state_applied,
            })
        } else {
            let partial = ItemApplyPartial {
                state_saved,
                state_current,
                state_target,
                state_diff,
                op_check_status,
            };
            Err((partial, state_applied))
        }
    }
}

impl<State, StateDiff> ItemApplyRt for ItemApply<State, StateDiff>
where
    State: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn state_saved(&self) -> Option<BoxDtDisplay> {
        self.state_saved.clone().map(BoxDtDisplay::new)
    }

    fn state_current(&self) -> BoxDtDisplay {
        BoxDtDisplay::new(self.state_current.clone())
    }

    fn state_target(&self) -> BoxDtDisplay {
        BoxDtDisplay::new(self.state_target.clone())
    }

    fn state_diff(&self) -> BoxDtDisplay {
        BoxDtDisplay::new(self.state_diff.clone())
    }

    fn op_check_status(&self) -> OpCheckStatus {
        self.op_check_status
    }

    fn state_applied(&self) -> Option<BoxDtDisplay> {
        self.state_applied.clone().map(BoxDtDisplay::new)
    }

    fn as_data_type(&self) -> &dyn DataType {
        self
    }

    fn as_data_type_mut(&mut self) -> &mut dyn DataType {
        self
    }
}
