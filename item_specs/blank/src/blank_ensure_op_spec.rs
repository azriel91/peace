use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, OpCtx};

use crate::{BlankData, BlankError, BlankState, BlankStateDiff};

/// Ensure OpSpec for the blank state.
#[derive(Debug)]
pub struct BlankEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for BlankEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = BlankData<'op, Id>;
    type Error = BlankError;
    type State = BlankState;
    type StateDiff = BlankStateDiff;

    async fn check(
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

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _blank_data: BlankData<'_, Id>,
        _state_current: &BlankState,
        state_desired: &BlankState,
        _diff: &BlankStateDiff,
    ) -> Result<BlankState, BlankError> {
        Ok(*state_desired)
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
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
