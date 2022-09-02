use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    dir::FlowDir,
    internal::StatesMut,
    resources_type_state::{SetUp, WithStatesDesired},
    states::{ts::Desired, StatesDesired},
    Resources,
};
use peace_rt_model::{CmdContext, Error, ItemSpecGraph, Storage};

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StateDesiredCmd<E>(PhantomData<E>);

impl<E> StateDesiredCmd<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// File name of the states file.
    pub const STATES_DESIRED_FILE: &'static str = "states_desired.yaml";

    /// Runs [`StateDesiredFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesDesired`], and will be serialized to
    /// `{profile_dir}/states_desired.yaml`.
    ///
    /// If any `StateDesiredFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their
    /// desired state into `Resources`, and the successor should references
    /// it in their [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::StateDesiredFnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, SetUp, E>,
    ) -> Result<CmdContext<WithStatesDesired, E>, E> {
        let (workspace, item_spec_graph, resources) = cmd_context.into_inner();
        let states_desired = Self::exec_internal(item_spec_graph, &resources).await?;

        let resources = Resources::<WithStatesDesired>::from((resources, states_desired));

        let cmd_context = CmdContext::from((workspace, item_spec_graph, resources));
        Ok(cmd_context)
    }

    /// Runs [`StateDesiredFnSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state.
    ///
    /// [`exec`]: peace_cfg::StateDesiredFnSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub(crate) async fn exec_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<StatesDesired, E> {
        let states_desired_mut = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .map_ok(|item_spec| async move {
                let state_desired = item_spec.state_desired_fn_exec(resources).await?;
                Ok((item_spec.id(), state_desired))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut<Desired>>()
            .await?;

        let states_desired = StatesDesired::from(states_desired_mut);
        Self::serialize_internal(resources, &states_desired).await?;

        Ok(states_desired)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn serialize_internal(
        resources: &Resources<SetUp>,
        states_desired: &StatesDesired,
    ) -> Result<(), E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file_path = flow_dir.join(Self::STATES_DESIRED_FILE);

        storage
            .write_with_sync_api(
                "states_desired_file_write".to_string(),
                &states_desired_file_path,
                |file| {
                    serde_yaml::to_writer(file, states_desired)
                        .map_err(Error::StatesDesiredSerialize)
                },
            )
            .await?;

        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) async fn serialize_internal(
        resources: &Resources<SetUp>,
        states_desired: &StatesDesired,
    ) -> Result<(), E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file_path = flow_dir.join(Self::STATES_DESIRED_FILE);

        let states_serialized =
            serde_yaml::to_string(states_desired).map_err(Error::StatesDesiredSerialize)?;
        let states_desired_file_path = states_desired_file_path.to_string_lossy();
        storage.set_item(&states_desired_file_path, &states_serialized)?;

        Ok(())
    }
}
