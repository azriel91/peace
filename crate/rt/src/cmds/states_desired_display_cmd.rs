use std::marker::PhantomData;

use peace_resources::{
    resources::ts::{SetUp, WithStatesDesired},
    Resources,
};
use peace_rt_model::{CmdContext, Error};
use peace_rt_model_core::OutputWrite;

use crate::cmds::sub::StatesDesiredReadCmd;

/// Displays [`StatesDesired`]s from storage.
#[derive(Debug)]
pub struct StatesDesiredDisplayCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesDesiredDisplayCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
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
        mut cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, WithStatesDesired>, E> {
        let CmdContext {
            output,
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_desired_result = StatesDesiredReadCmd::<E, O>::exec_internal(
            resources,
            states_type_regs.states_desired_type_reg(),
        )
        .await;

        match states_desired_result {
            Ok(states_desired) => {
                output.write_states_desired(&states_desired).await?;

                let cmd_context = CmdContext::from((cmd_context, |resources| {
                    Resources::<WithStatesDesired>::from((resources, states_desired))
                }));
                Ok(cmd_context)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }
}

impl<E, O> Default for StatesDesiredDisplayCmd<E, O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
