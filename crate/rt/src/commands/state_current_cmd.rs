use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{SetUp, WithStateDiffs, WithStates},
    Resources, States, StatesMut,
};
use peace_rt_model::{CmdContext, ItemSpecGraph};

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StateCurrentCmd<E>(PhantomData<E>);

impl<E> StateCurrentCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`].
    ///
    /// If any `StateCurrentFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, SetUp, E>,
    ) -> Result<CmdContext<WithStates, E>, E> {
        let (workspace, item_spec_graph, resources) = cmd_context.into_inner();
        let states = Self::exec_internal(item_spec_graph, &resources).await?;

        let resources = Resources::<WithStates>::from((resources, states));
        let cmd_context = CmdContext::from((workspace, item_spec_graph, resources));
        Ok(cmd_context)
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`States`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub(crate) async fn exec_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<States, E> {
        let states_mut = item_spec_graph
            .stream()
            .map(Result::Ok)
            .map_ok(|item_spec| async move {
                let state = item_spec.state_current_fn_exec(resources).await?;
                Ok((item_spec.id(), state))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut>()
            .await?;

        Ok(States::from(states_mut))
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`States`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub(crate) async fn exec_internal_for_ensure(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<States, E> {
        let states_mut = item_spec_graph
            .stream()
            .map(Result::Ok)
            .map_ok(|item_spec| async move {
                let state = item_spec.state_ensured_fn_exec(resources).await?;
                Ok((item_spec.id(), state))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut>()
            .await?;

        Ok(States::from(states_mut))
    }
}
