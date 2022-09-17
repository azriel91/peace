use std::marker::PhantomData;

use peace_resources::{
    resources_type_state::{SetUp, WithStates},
    states::StatesCurrent,
};
use peace_rt_model::{CmdContext, Error};
use peace_rt_model_core::OutputWrite;

use crate::StatesCurrentReadCmd;

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
        cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<E, O, WithStates>, E> {
        let result = StatesCurrentReadCmd::<E, O>::exec_no_output(cmd_context).await;

        match result {
            Ok(mut cmd_context) => {
                {
                    let CmdContext {
                        resources, output, ..
                    } = &mut cmd_context;
                    let states_current = resources.borrow::<StatesCurrent>();

                    output.write_states_current(&states_current).await?;
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
