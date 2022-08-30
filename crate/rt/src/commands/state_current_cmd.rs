use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    dir::ProfileDir,
    resources_type_state::{SetUp, WithStateDiffs, WithStates},
    Resources, States, StatesMut,
};
use peace_rt_model::{CmdContext, Error, ItemSpecGraph, Storage};

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StateCurrentCmd<E>(PhantomData<E>);

impl<E> StateCurrentCmd<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// File name of the states file.
    pub const STATES_CURRENT_FILE: &'static str = "states.yaml";

    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`], and will be serialized to `{profile_dir}/states.yaml`.
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
        Self::serialize_internal(&resources).await?;

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
            .map(Result::<_, E>::Ok)
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
            .map(Result::<_, E>::Ok)
            .map_ok(|item_spec| async move {
                let state = item_spec.state_ensured_fn_exec(resources).await?;
                Ok((item_spec.id(), state))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut>()
            .await?;

        Ok(States::from(states_mut))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn serialize_internal(resources: &Resources<WithStates>) -> Result<(), E> {
        let profile_dir = resources.borrow::<ProfileDir>();
        let states = resources.borrow::<States>();
        let storage = resources.borrow::<Storage>();
        let states_file_path = profile_dir.join(Self::STATES_CURRENT_FILE);

        storage
            .write_with_sync_api("states_file_write".to_string(), &states_file_path, |file| {
                serde_yaml::to_writer(file, &*states).map_err(Error::StatesSerialize)
            })
            .await?;

        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) async fn serialize_internal(resources: &Resources<WithStates>) -> Result<(), E> {
        let profile_dir = resources.borrow::<ProfileDir>();
        let states = resources.borrow::<States>();
        let storage = resources.borrow::<Storage>();
        let states_file_path = profile_dir.join(Self::STATES_CURRENT_FILE);

        let states_serialized = serde_yaml::to_string(&*states).map_err(Error::StatesSerialize)?;
        let states_file_path = states_file_path.to_string_lossy();
        storage.set_item(&states_file_path, &states_serialized)?;

        Ok(())
    }
}
