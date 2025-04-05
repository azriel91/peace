use std::{fmt::Debug, marker::PhantomData};

use peace_cmd_ctx::{CmdCtxSpsf, CmdCtxTypes};
use peace_cmd_model::CmdOutcome;
use peace_resource_rt::states::StatesGoalStored;

use peace_rt_model_core::output::OutputWrite;

use crate::cmds::StatesGoalReadCmd;

/// Displays [`StatesGoal`]s from storage.
#[derive(Debug)]
pub struct StatesGoalDisplayCmd<CmdCtxTypesT>(PhantomData<CmdCtxTypesT>);

#[cfg(not(feature = "error_reporting"))]
impl<CmdCtxTypesT> StatesGoalDisplayCmd<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Displays [`StatesGoal`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtxSpsf<'ctx, CmdCtxTypesT>,
    ) -> Result<
        CmdOutcome<StatesGoalStored, <CmdCtxTypesT as CmdCtxTypes>::AppError>,
        <CmdCtxTypesT as CmdCtxTypes>::AppError,
    >
    where
        CmdCtxTypesT: 'ctx,
    {
        let states_goal_stored_result = StatesGoalReadCmd::exec(cmd_ctx).await;
        let output = cmd_ctx.output_mut();

        match states_goal_stored_result {
            Ok(states_goal_cmd_outcome) => {
                if let Some(states_goal) = states_goal_cmd_outcome.value() {
                    output.present(states_goal).await?;
                }

                Ok(states_goal_cmd_outcome)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }
}

// Pending: <https://github.com/rust-lang/rust/issues/115590>
#[cfg(feature = "error_reporting")]
impl<CmdCtxTypesT> StatesGoalDisplayCmd<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
    <CmdCtxTypesT as CmdCtxTypes>::AppError: miette::Diagnostic,
{
    /// Displays [`StatesGoal`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtxSpsf<'ctx, CmdCtxTypesT>,
    ) -> Result<
        CmdOutcome<StatesGoalStored, <CmdCtxTypesT as CmdCtxTypes>::AppError>,
        <CmdCtxTypesT as CmdCtxTypes>::AppError,
    >
    where
        CmdCtxTypesT: 'ctx,
    {
        let states_goal_stored_result = StatesGoalReadCmd::exec(cmd_ctx).await;
        let output = cmd_ctx.output_mut();

        match states_goal_stored_result {
            Ok(states_goal_cmd_outcome) => {
                if let Some(states_goal) = states_goal_cmd_outcome.value() {
                    output.present(states_goal).await?;
                }

                Ok(states_goal_cmd_outcome)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }
}

impl<CmdCtxTypesT> Default for StatesGoalDisplayCmd<CmdCtxTypesT> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
