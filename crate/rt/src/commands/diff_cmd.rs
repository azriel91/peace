use std::marker::PhantomData;

use futures::StreamExt;
use peace_resources::{
    resources_type_state::{SetUp, WithStateDiffs},
    Resources, StateDiffs, StateDiffsMut, StatesDesiredRw, StatesRw,
};
use peace_rt_model::FullSpecGraph;

use crate::{StateDesiredCmd, StateNowCmd};

#[derive(Debug)]
pub struct DiffCmd<E>(PhantomData<E>);

impl<E> DiffCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`StateNowFnSpec`]` and `[`StateDesiredFnSpec`]`::`[`exec`] for
    /// each [`FullSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`] and [`StatesDesired`].
    ///
    /// If any `StateNowFnSpec` needs to read the `State` from a previous
    /// `FullSpec`, the [`StatesRw`] type should be used in
    /// [`FnSpec::Data`].
    ///
    /// Likewise, if any `StateDesiredFnSpec` needs to read the desired `State`
    /// from a previous `FullSpec`, the [`StatesDesiredRw`] type should be
    /// used in [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`States`]: peace_resources::States
    /// [`StatesRw`]: peace_resources::StatesRw
    /// [`StateNowFnSpec`]: peace_cfg::FullSpec::StateNowFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::FullSpec::StateDesiredFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        mut resources: Resources<SetUp>,
    ) -> Result<Resources<WithStateDiffs>, E> {
        StateNowCmd::exec_internal(full_spec_graph, &resources).await?;
        StateDesiredCmd::exec_internal(full_spec_graph, &resources).await?;

        let state_diffs = {
            let states_rw = resources.borrow::<StatesRw>();
            let states = &states_rw.read().await;
            let states_desired_rw = resources.borrow::<StatesDesiredRw>();
            let states_desired = &states_desired_rw.read().await;

            let state_diffs_mut = full_spec_graph
                .stream()
                .map(|full_spec| (full_spec.id(), full_spec.diff(states, states_desired)))
                .fold(
                    StateDiffsMut::new(),
                    |mut state_diffs_mut, (full_spec_id, state_diff)| async move {
                        state_diffs_mut.insert_raw(full_spec_id.clone(), state_diff);
                        state_diffs_mut
                    },
                )
                .await;

            StateDiffs::from(state_diffs_mut)
        };

        resources.insert(state_diffs);

        Ok(Resources::<WithStateDiffs>::from(resources))
    }
}
