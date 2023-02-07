use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use peace_resources::{
    resources::ts::{SetUp, WithStatesCurrentAndDesired},
    Resources,
};
use peace_rt_model::{cmd::CmdContext, Error};
use serde::{de::DeserializeOwned, Serialize};

use crate::cmds::sub::{StatesCurrentDiscoverCmd, StatesDesiredDiscoverCmd};

#[derive(Debug)]
pub struct StatesDiscoverCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>(
    PhantomData<(E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK)>,
);

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    StatesDiscoverCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error + From<Error> + Send,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Runs [`StateCurrentFnSpec`]` and
    /// `[`StateDesiredFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesDesired`], and will be serialized to
    /// `{profile_dir}/states.yaml` and `{profile_dir}/states_desired.yaml`.
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StatesCurrent`]: peace_resources::StatesCurrent
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, E, O, SetUp, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
    ) -> Result<
        CmdContext<
            '_,
            E,
            O,
            WithStatesCurrentAndDesired,
            WorkspaceParamsK,
            ProfileParamsK,
            FlowParamsK,
        >,
        E,
    > {
        let CmdContext {
            workspace,
            item_spec_graph,
            output,
            mut resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            ..
        } = cmd_context;
        let states_current = StatesCurrentDiscoverCmd::<
            E,
            O,
            WorkspaceParamsK,
            ProfileParamsK,
            FlowParamsK,
        >::exec_internal(item_spec_graph, &mut resources)
        .await?;
        let states_desired = StatesDesiredDiscoverCmd::<
            E,
            O,
            WorkspaceParamsK,
            ProfileParamsK,
            FlowParamsK,
        >::exec_internal(item_spec_graph, &mut resources)
        .await?;

        let resources = Resources::<WithStatesCurrentAndDesired>::from((
            resources,
            states_current,
            states_desired,
        ));

        let cmd_context = CmdContext::from((
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
        ));
        Ok(cmd_context)
    }
}

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK> Default
    for StatesDiscoverCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
{
    fn default() -> Self {
        Self(PhantomData)
    }
}
