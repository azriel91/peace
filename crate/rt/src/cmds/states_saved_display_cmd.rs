use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use peace_resources::{
    resources::ts::{SetUp, WithStatesSaved},
    Resources,
};
use peace_rt_model::{cmd::CmdContext, Error};
use peace_rt_model_core::output::OutputWrite;
use serde::{de::DeserializeOwned, Serialize};

use crate::cmds::sub::StatesSavedReadCmd;

/// Displays [`StatesCurrent`]s from storage.
#[derive(Debug)]
pub struct StatesSavedDisplayCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>(
    PhantomData<(E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK)>,
);

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    StatesSavedDisplayCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error + From<Error> + Send,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
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
        mut cmd_context: CmdContext<'_, E, O, SetUp, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
    ) -> Result<
        CmdContext<'_, E, O, WithStatesSaved, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
        E,
    > {
        let CmdContext {
            output,
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_saved_result = StatesSavedReadCmd::<
            E,
            O,
            WorkspaceParamsK,
            ProfileParamsK,
            FlowParamsK,
        >::exec_internal(
            resources, states_type_regs.states_current_type_reg()
        )
        .await;

        match states_saved_result {
            Ok(states_saved) => {
                output.present(&states_saved).await?;

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

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK> Default
    for StatesSavedDisplayCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
{
    fn default() -> Self {
        Self(PhantomData)
    }
}
