use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{FnCtx, OpCheckStatus};

use crate::{BlankData, BlankError, BlankState, BlankStateDiff};

/// ApplyFns for the blank state.
#[derive(Debug)]
pub struct BlankApplyFns<Id>(PhantomData<Id>);

impl<Id> BlankApplyFns<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn apply_check(
        _blank_data: BlankData<'_, Id>,
        _state_current: &BlankState,
        _state_desired: &BlankState,
        diff: &BlankStateDiff,
    ) -> Result<OpCheckStatus, BlankError> {
        let op_check_status = match *diff {
            BlankStateDiff::InSync { .. } => OpCheckStatus::ExecNotRequired,
            BlankStateDiff::Added { .. } | BlankStateDiff::OutOfSync { .. } => {
                #[cfg(not(feature = "output_progress"))]
                {
                    OpCheckStatus::ExecRequired
                }
                #[cfg(feature = "output_progress")]
                {
                    let progress_limit = ProgressLimit::Steps(1);
                    OpCheckStatus::ExecRequired { progress_limit }
                }
            }
        };

        Ok(op_check_status)
    }

    pub async fn apply_dry(
        _fn_ctx: FnCtx<'_>,
        _blank_data: BlankData<'_, Id>,
        _state_current: &BlankState,
        state_desired: &BlankState,
        _diff: &BlankStateDiff,
    ) -> Result<BlankState, BlankError> {
        Ok(*state_desired)
    }

    pub async fn apply(
        _fn_ctx: FnCtx<'_>,
        mut blank_data: BlankData<'_, Id>,
        _state_current: &BlankState,
        state_desired: &BlankState,
        _diff: &BlankStateDiff,
    ) -> Result<BlankState, BlankError> {
        let params = blank_data.params_mut();
        **params.dest_mut() = Some(**params.src());

        Ok(*state_desired)
    }
}
