use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesDesiredFile},
    resources::ts::{SetUp, WithStatesDesired},
    states::{ts::Desired, StatesDesired},
    Resources,
};
use peace_rt_model::{CmdContext, Error, ItemSpecGraph, StatesSerializer, Storage};

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StatesDesiredDiscoverCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesDesiredDiscoverCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Runs [`StateDesiredFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesDesired`], and will be serialized to
    /// `$flow_dir/states_desired.yaml`.
    ///
    /// If any `StateDesiredFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their
    /// desired state into `Resources`, and the successor should references
    /// it in their [`Data`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, WithStatesDesired>, E> {
        let CmdContext {
            workspace,
            item_spec_graph,
            output,
            mut resources,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            ..
        } = cmd_context;
        let states_desired = Self::exec_internal(item_spec_graph, &mut resources).await?;

        let resources = Resources::<WithStatesDesired>::from((resources, states_desired));

        let cmd_context = CmdContext::from((
            workspace,
            item_spec_graph,
            output,
            resources,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
        ));
        Ok(cmd_context)
    }

    /// Runs [`StateDesiredFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state.
    ///
    /// [`exec`]: peace_cfg::StateDesiredFnSpec::try_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub(crate) async fn exec_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &mut Resources<SetUp>,
    ) -> Result<StatesDesired, E> {
        let resources_ref = &*resources;
        let states_desired_mut = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_filter_map(|item_spec| async move {
                let state_desired = item_spec.state_desired_try_exec(resources_ref).await?;
                Ok(state_desired
                    .map(|state_desired| (item_spec.id().clone(), state_desired))
                    .map(Result::Ok)
                    .map(futures::future::ready))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut<Desired>>()
            .await?;

        let states_desired = StatesDesired::from(states_desired_mut);
        Self::serialize_internal(resources, &states_desired).await?;

        Ok(states_desired)
    }

    pub(crate) async fn serialize_internal(
        resources: &mut Resources<SetUp>,
        states_desired: &StatesDesired,
    ) -> Result<(), E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_desired, &states_desired_file).await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_desired_file);

        Ok(())
    }
}

impl<E, O> Default for StatesDesiredDiscoverCmd<E, O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
