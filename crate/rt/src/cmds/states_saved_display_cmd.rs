use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::{CmdCtx, CmdCtxView},
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    resources::ts::{SetUp, WithStatesSaved},
    Resources,
};
use peace_rt_model::{cmd_context_params::ParamsKeys, Error};
use peace_rt_model_core::output::OutputWrite;

use crate::cmds::sub::StatesSavedReadCmd;

/// Displays [`StatesCurrent`]s from storage.
#[derive(Debug)]
pub struct StatesSavedDisplayCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesSavedDisplayCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    /// Displays [`StatesSaved`]s from storage.
    ///
    /// Either [`StatesSavedDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesSavedDiscoverCmd`]: crate::StatesSavedDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec_v2(
        mut cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, WithStatesSaved>, PKeys>, E> {
        let CmdCtxView { output, scope, .. } = cmd_ctx.view();
        let SingleProfileSingleFlowView {
            states_type_regs,
            resources,
            ..
        } = scope.view();

        let states_saved_result = StatesSavedReadCmd::<E, O, PKeys>::exec_internal(
            resources,
            states_type_regs.states_current_type_reg(),
        )
        .await;

        match states_saved_result {
            Ok(states_saved) => {
                output.present(&states_saved).await?;

                let cmd_ctx = cmd_ctx.resources_update(|resources| {
                    Resources::<WithStatesSaved>::from((resources, states_saved))
                });
                Ok(cmd_ctx)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }
}

impl<E, O, PKeys> Default for StatesSavedDisplayCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
