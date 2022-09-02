use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesCurrentFile},
    resources_type_state::{SetUp, WithStateDiffs, WithStates},
    states::{ts::Current, StatesCurrent},
    Resources,
};
use peace_rt_model::{CmdContext, Error, ItemSpecGraph, Storage};

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StateCurrentCmd<E>(PhantomData<E>);

impl<E> StateCurrentCmd<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], and will be serialized to `{flow_dir}/states.yaml`.
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
        let (workspace, item_spec_graph, mut resources) = cmd_context.into_inner();
        let states = Self::exec_internal(item_spec_graph, &mut resources).await?;

        let resources = Resources::<WithStates>::from((resources, states));

        let cmd_context = CmdContext::from((workspace, item_spec_graph, resources));
        Ok(cmd_context)
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesCurrent`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
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
            .map_ok(|item_spec| async move {
                let state = item_spec.state_current_fn_exec(resources_ref).await?;
                Ok((item_spec.id(), state))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut<Current>>()
            .await?;

        let states = StatesCurrent::from(states_mut);
        Self::serialize_internal(resources, &states).await?;

        Ok(states)
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesCurrent`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub(crate) async fn exec_internal_for_ensure(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<StatesCurrent, E> {
        let states_mut = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .map_ok(|item_spec| async move {
                let state = item_spec.state_ensured_fn_exec(resources).await?;
                Ok((item_spec.id(), state))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut<Current>>()
            .await?;

        Ok(StatesCurrent::from(states_mut))
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn serialize_internal(
        resources: &mut Resources<SetUp>,
        states: &StatesCurrent,
    ) -> Result<(), E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        storage
            .write_with_sync_api(
                "states_file_write".to_string(),
                &states_current_file,
                |file| serde_yaml::to_writer(file, states).map_err(Error::StatesSerialize),
            )
            .await?;
        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    async fn serialize_internal(
        resources: &mut Resources<SetUp>,
        states: &StatesCurrent,
    ) -> Result<(), E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        let states_serialized = serde_yaml::to_string(&*states).map_err(Error::StatesSerialize)?;
        let states_current_file_str = states_current_file.to_string_lossy();
        storage.set_item(&states_current_file_str, &states_serialized)?;
        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(())
    }
}
