use std::{fmt::Debug, marker::PhantomData};

use peace_cmd_ctx::{CmdCtxSpsf, CmdCtxTypes};
use peace_cmd_model::CmdOutcome;
use peace_resource_rt::states::StatesCurrentStored;
use peace_rt_model_core::output::OutputWrite;

use crate::cmds::StatesCurrentReadCmd;

/// Displays [`StatesCurrent`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentStoredDisplayCmd<CmdCtxTypesT>(PhantomData<CmdCtxTypesT>);

#[cfg(not(feature = "error_reporting"))]
impl<CmdCtxTypesT> StatesCurrentStoredDisplayCmd<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Displays [`StatesCurrentStored`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtxSpsf<'ctx, CmdCtxTypesT>,
    ) -> Result<
        CmdOutcome<StatesCurrentStored, <CmdCtxTypesT as CmdCtxTypes>::AppError>,
        <CmdCtxTypesT as CmdCtxTypes>::AppError,
    >
    where
        CmdCtxTypesT: 'ctx,
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

// Pending: <https://github.com/rust-lang/rust/issues/115590>
#[cfg(feature = "error_reporting")]
impl<CmdCtxTypesT> StatesCurrentStoredDisplayCmd<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
    <CmdCtxTypesT as CmdCtxTypes>::AppError: miette::Diagnostic,
{
    /// Displays [`StatesCurrentStored`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtxSpsf<'ctx, CmdCtxTypesT>,
    ) -> Result<
        CmdOutcome<StatesCurrentStored, <CmdCtxTypesT as CmdCtxTypes>::AppError>,
        <CmdCtxTypesT as CmdCtxTypes>::AppError,
    >
    where
        CmdCtxTypesT: 'ctx,
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

impl<CmdCtxTypesT> Default for StatesCurrentStoredDisplayCmd<CmdCtxTypesT> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
