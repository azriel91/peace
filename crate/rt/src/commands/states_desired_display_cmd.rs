use std::marker::PhantomData;

use peace_resources::{
    resources_type_state::{SetUp, WithStatesDesired},
    states::StatesDesired,
};
use peace_rt_model::{CmdContext, Error};
use peace_rt_model_core::OutputWrite;

use crate::StatesDesiredReadCmd;

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
        cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<E, O, WithStatesDesired>, E> {
        let result = StatesDesiredReadCmd::<E, O>::exec_no_output(cmd_context).await;

        match result {
            Ok(mut cmd_context) => {
                {
                    let CmdContext {
                        resources, output, ..
                    } = &mut cmd_context;
                    let states_desired = resources.borrow::<StatesDesired>();

                    output.write_states_desired(&states_desired).await?;
                }

                Ok(cmd_context)
            }
            Err((mut cmd_context, e)) => {
                let output = cmd_context.output_mut();
                output.write_err(&e).await?;

                Err(e)
            }
        }
    }
}
