use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::CmdCtx,
    scopes::{
        SingleProfileSingleFlow, SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput,
    },
};
use peace_resources::{resources::ts::SetUp, states::StatesDesired};
use peace_rt_model::{params::ParamsKeys, Error};
use peace_rt_model_core::output::OutputWrite;

use crate::cmds::StatesDesiredReadCmd;

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
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesDesired, E> {
        let SingleProfileSingleFlowViewAndOutput {
            output,
            cmd_view:
                SingleProfileSingleFlowView {
                    states_type_reg,
                    resources,
                    ..
                },
            ..
        } = cmd_ctx.view_and_output();

        let states_desired_result =
            StatesDesiredReadCmd::<E, O, PKeys>::deserialize_internal(resources, states_type_reg)
                .await;

        match states_desired_result {
            Ok(states_desired) => {
                output.present(&states_desired).await?;
                Ok(states_desired)
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
