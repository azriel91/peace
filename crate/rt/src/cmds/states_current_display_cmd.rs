use std::marker::PhantomData;

use peace_resources::{
    resources::ts::{SetUp, WithStates},
    Resources,
};
use peace_rt_model::{CmdContext, Error};
use peace_rt_model_core::OutputWrite;

use crate::cmds::sub::StatesCurrentReadCmd;

/// Displays [`StatesCurrent`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentDisplayCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesCurrentDisplayCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
    O: OutputWrite<E>,
{
    /// Displays [`StatesCurrent`]s from storage.
    ///
    /// Either [`StatesCurrentDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesCurrentDiscoverCmd`]: crate::StatesCurrentDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, WithStates>, E> {
        let CmdContext {
            output,
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_current_result = StatesCurrentReadCmd::<E, O>::exec_internal(
            resources,
            states_type_regs.states_current_type_reg(),
        )
        .await;

        match states_current_result {
            Ok(states_current) => {
                output.write_states_current(&states_current).await?;

                let cmd_context = CmdContext::from((cmd_context, |resources| {
                    Resources::<WithStates>::from((resources, states_current))
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

impl<E, O> Default for StatesCurrentDisplayCmd<E, O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
