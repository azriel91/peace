use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    resources::ts::{SetUp, WithStatesDesired},
    Resources,
};
use peace_rt_model::{params::ParamsKeys, Error};
use peace_rt_model_core::output::OutputWrite;

use crate::cmds::sub::StatesDesiredReadCmd;

/// Displays [`StatesDesired`]s from storage.
#[derive(Debug)]
pub struct StatesDesiredDisplayCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesDesiredDisplayCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    /// Displays [`StatesDesired`]s from storage.
    ///
    /// Either [`StatesDesiredDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesDesiredDiscoverCmd`]: crate::StatesDesiredDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_ctx: CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, WithStatesDesired>>, E> {
        let SingleProfileSingleFlowView {
            output,
            states_type_regs,
            resources,
            ..
        } = cmd_ctx.view();

        let states_desired_result = StatesDesiredReadCmd::<E, O, PKeys>::exec_internal(
            resources,
            states_type_regs.states_desired_type_reg(),
        )
        .await;

        match states_desired_result {
            Ok(states_desired) => {
                output.present(&states_desired).await?;

                let cmd_ctx = cmd_ctx.resources_update(|resources| {
                    Resources::<WithStatesDesired>::from((resources, states_desired))
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

impl<E, O, PKeys> Default for StatesDesiredDisplayCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
