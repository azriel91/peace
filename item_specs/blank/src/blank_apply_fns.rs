use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{ApplyCheck, FnCtx};

use crate::{BlankData, BlankError, BlankParams, BlankState, BlankStateDiff};

/// ApplyFns for the blank state.
#[derive(Debug)]
pub struct BlankApplyFns<Id>(PhantomData<Id>);

impl<Id> BlankApplyFns<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn apply_check(
        _params: &BlankParams<Id>,
        _data: BlankData<'_, Id>,
        _state_current: &BlankState,
        _state_desired: &BlankState,
        diff: &BlankStateDiff,
    ) -> Result<ApplyCheck, BlankError> {
        let apply_check = match *diff {
            BlankStateDiff::InSync { .. } => ApplyCheck::ExecNotRequired,
            BlankStateDiff::Added { .. } | BlankStateDiff::OutOfSync { .. } => {
                #[cfg(not(feature = "output_progress"))]
                {
                    ApplyCheck::ExecRequired
                }
                #[cfg(feature = "output_progress")]
                {
                    let progress_limit = ProgressLimit::Steps(1);
                    ApplyCheck::ExecRequired { progress_limit }
                }
            }
        };

        Ok(apply_check)
    }

    pub async fn apply_dry(
        _fn_ctx: FnCtx<'_>,
        _params: &BlankParams<Id>,
        _data: BlankData<'_, Id>,
        _state_current: &BlankState,
        state_desired: &BlankState,
        _diff: &BlankStateDiff,
    ) -> Result<BlankState, BlankError> {
        Ok(*state_desired)
    }

    pub async fn apply(
        _fn_ctx: FnCtx<'_>,
        _params: &BlankParams<Id>,
        mut data: BlankData<'_, Id>,
        _state_current: &BlankState,
        state_desired: &BlankState,
        _diff: &BlankStateDiff,
    ) -> Result<BlankState, BlankError> {
        let params = data.params_mut();
        **params.dest_mut() = Some(**params.src());

        Ok(*state_desired)
    }
}
