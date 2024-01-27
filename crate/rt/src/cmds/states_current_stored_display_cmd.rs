use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::{CmdCtx, CmdCtxTypeParamsConstrained},
    scopes::SingleProfileSingleFlow,
};
use peace_cmd_model::CmdOutcome;
use peace_resources::{resources::ts::SetUp, states::StatesCurrentStored};
use peace_rt_model_core::output::OutputWrite;

use crate::cmds::StatesCurrentReadCmd;

/// Displays [`StatesCurrent`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentStoredDisplayCmd<CmdCtxTypeParamsT>(PhantomData<CmdCtxTypeParamsT>);

impl<CmdCtxTypeParamsT> StatesCurrentStoredDisplayCmd<CmdCtxTypeParamsT>
where
    CmdCtxTypeParamsT: CmdCtxTypeParamsConstrained,
{
    /// Displays [`StatesCurrentStored`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'ctx, CmdCtxTypeParamsT, SetUp>>,
    ) -> Result<
        CmdOutcome<
            StatesCurrentStored,
            <CmdCtxTypeParamsT as CmdCtxTypeParamsConstrained>::AppError,
        >,
        <CmdCtxTypeParamsT as CmdCtxTypeParamsConstrained>::AppError,
    >
    where
        CmdCtxTypeParamsT: 'ctx,
    {
        let states_current_stored_result = StatesCurrentReadCmd::exec(cmd_ctx).await;
        let output = cmd_ctx.output_mut();

        match states_current_stored_result {
            Ok(states_current_cmd_outcome) => {
                if let Some(states_current_stored) = states_current_cmd_outcome.value() {
                    output.present(states_current_stored).await?;
                }
                Ok(states_current_cmd_outcome)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }
}

impl<CmdCtxTypeParamsT> Default for StatesCurrentStoredDisplayCmd<CmdCtxTypeParamsT> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
