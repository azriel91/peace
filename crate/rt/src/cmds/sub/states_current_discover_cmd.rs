use std::{fmt::Debug, marker::PhantomData};

use futures::stream::{StreamExt, TryStreamExt};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesSavedFile},
    resources::ts::{SetUp, WithStatesCurrent},
    states::{ts::Current, StatesCurrent},
    Resources,
};
use peace_rt_model::{cmd_context_params::ParamsKeys, Error, ItemSpecGraph, Storage};

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StatesCurrentDiscoverCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesCurrentDiscoverCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
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
        mut cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, WithStatesCurrent>, PKeys>, E> {
        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();

        let states = Self::exec_internal(flow.graph(), resources).await?;
        let cmd_ctx = cmd_ctx.resources_update(|resources| {
            Resources::<WithStatesCurrent>::from((resources, states))
        });
        Ok(cmd_ctx)
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

impl<E, O, PKeys> Default for StatesCurrentDiscoverCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
