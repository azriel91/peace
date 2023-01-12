use std::marker::PhantomData;

use peace_resources::{
    resources::ts::{SetUp, WithStatesSaved},
    Resources,
};
use peace_rt_model::{CmdContext, Error};
use peace_rt_model_core::output::OutputWrite;

use crate::cmds::sub::StatesSavedReadCmd;

/// Displays [`StatesCurrent`]s from storage.
#[derive(Debug)]
pub struct StatesSavedDisplayCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesSavedDisplayCmd<E, O>
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
    ) -> Result<CmdContext<'_, E, O, WithStatesSaved>, E> {
        let CmdContext {
            output,
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_saved_result = StatesSavedReadCmd::<E, O>::exec_internal(
            resources,
            states_type_regs.states_current_type_reg(),
        )
        .await;

        match states_saved_result {
            Ok(states_saved) => {
                output.write_states_saved(&states_saved).await?;

                let cmd_context = CmdContext::from((cmd_context, |resources| {
                    Resources::<WithStatesSaved>::from((resources, states_saved))
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

impl<E, O> Default for StatesSavedDisplayCmd<E, O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
