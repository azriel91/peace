use std::marker::PhantomData;

use futures::StreamExt;
use peace_resources::{
    resources_type_state::{SetUp, WithStateDiffs},
    Resources, StateDiffs, StateDiffsMut,
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
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStateDiffs>, E> {
        let states = StateNowCmd::exec_internal(full_spec_graph, &resources).await?;
        let states_desired = StateDesiredCmd::exec_internal(full_spec_graph, &resources).await?;

        let states_ref = &states;
        let states_desired_ref = &states_desired;
        let state_diffs = {
            let state_diffs_mut = full_spec_graph
                .stream()
                .map(|full_spec| {
                    (
                        full_spec.id(),
                        full_spec.diff(states_ref, states_desired_ref),
                    )
                })
                .collect::<StateDiffsMut>()
                .await;

            StateDiffs::from(state_diffs_mut)
        };

        Ok(Resources::<WithStateDiffs>::from((
            resources,
            states,
            states_desired,
            state_diffs,
        )))
    }
}
