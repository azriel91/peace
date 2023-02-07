use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesSavedFile},
    resources::ts::{SetUp, WithStatesCurrent},
    states::{ts::Current, StatesCurrent},
    Resources,
};
use peace_rt_model::{cmd::CmdContext, Error, ItemSpecGraph, Storage};
use serde::{de::DeserializeOwned, Serialize};

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StatesCurrentDiscoverCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>(
    PhantomData<(E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK)>,
);

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    StatesCurrentDiscoverCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error + From<Error> + Send,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Runs [`StateCurrentFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], and will be serialized to
    /// `$flow_dir/states_saved.yaml`.
    ///
    /// If any `StateCurrentFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`Data`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, E, O, SetUp, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
    ) -> Result<
        CmdContext<'_, E, O, WithStatesCurrent, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
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
        let states = Self::exec_internal(item_spec_graph, &mut resources).await?;

        let resources = Resources::<WithStatesCurrent>::from((resources, states));

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

    /// Runs [`StateCurrentFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesCurrent`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub(crate) async fn exec_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &mut Resources<SetUp>,
    ) -> Result<StatesCurrent, E> {
        let resources_ref = &*resources;
        let states_mut = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_filter_map(|item_spec| async move {
                let state = item_spec.state_current_try_exec(resources_ref).await?;
                Ok(state
                    .map(|state| (item_spec.id().clone(), state))
                    .map(Result::Ok)
                    .map(futures::future::ready))
            })
            // TODO: do we need this?
            // If not, we can remove the `Ok` and `future::ready` mappings above.
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut<Current>>()
            .await?;

        let states = StatesCurrent::from(states_mut);
        Self::serialize_internal(resources, &states).await?;

        Ok(states)
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesCurrent`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub(crate) async fn exec_internal_for_clean_dry(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStatesCurrent>,
    ) -> Result<StatesCurrent, E> {
        let states_mut = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_filter_map(|item_spec| async move {
                let state = item_spec.state_cleaned_try_exec(resources).await?;
                Ok(state
                    .map(|state| (item_spec.id().clone(), state))
                    .map(Result::Ok)
                    .map(futures::future::ready))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut<Current>>()
            .await?;

        let states = StatesCurrent::from(states_mut);
        // We don't serialize states to disk as this is for a dry run.

        Ok(states)
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesCurrent`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub(crate) async fn exec_internal_for_clean(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &mut Resources<WithStatesCurrent>,
    ) -> Result<StatesCurrent, E> {
        let resources_ref = &*resources;
        let states_mut = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_filter_map(|item_spec| async move {
                let state = item_spec.state_cleaned_try_exec(resources_ref).await?;
                Ok(state
                    .map(|state| (item_spec.id().clone(), state))
                    .map(Result::Ok)
                    .map(futures::future::ready))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut<Current>>()
            .await?;

        let states = StatesCurrent::from(states_mut);
        Self::serialize_internal(resources, &states).await?;

        Ok(states)
    }

    // TODO: This duplicates a bit of code with `EnsureCmd`.
    async fn serialize_internal<TS>(
        resources: &mut Resources<TS>,
        states_current: &StatesCurrent,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_saved_file = StatesSavedFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_current, &states_saved_file).await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_saved_file);

        Ok(())
    }
}

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK> Default
    for StatesCurrentDiscoverCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
{
    fn default() -> Self {
        Self(PhantomData)
    }
}
