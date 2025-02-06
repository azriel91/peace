use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::{CmdCtx, CmdCtxTypesConstrained},
    scopes::SingleProfileSingleFlow,
};
use peace_cmd_model::CmdOutcome;
use peace_resource_rt::states::StatesGoalStored;

use peace_rt_model_core::output::OutputWrite;

use crate::cmds::StatesGoalReadCmd;

/// Displays [`StatesGoal`]s from storage.
#[derive(Debug)]
pub struct StatesGoalDisplayCmd<CmdCtxTypesT>(PhantomData<CmdCtxTypesT>);

impl<CmdCtxTypesT> StatesGoalDisplayCmd<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypesConstrained,
{
    /// Displays [`StatesGoal`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'ctx, CmdCtxTypesT>>,
    ) -> Result<
        CmdOutcome<StatesGoalStored, <CmdCtxTypesT as CmdCtxTypesConstrained>::AppError>,
        <CmdCtxTypesT as CmdCtxTypesConstrained>::AppError,
    >
    where
        CmdCtxTypesT: 'ctx,
        <CmdCtxTypesT as CmdCtxTypesConstrained>::AppError:
            From<<<CmdCtxTypesT as CmdCtxTypesConstrained>::Output as OutputWrite>::Error>,
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
