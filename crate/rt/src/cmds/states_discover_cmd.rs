use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_cmd_rt::{CmdBlockWrapper, CmdExecution};
use peace_resources::{
    paths::{FlowDir, StatesCurrentFile, StatesGoalFile},
    resources::ts::SetUp,
    states::{StatesCurrent, StatesGoal},
    Resources,
};
use peace_rt_model::{
    outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error, ItemGraph, Storage,
};

use crate::cmd_blocks::StatesDiscoverCmdBlock;

#[cfg(feature = "output_progress")]
use peace_cfg::progress::{ProgressComplete, ProgressStatus};

pub struct StatesDiscoverCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> Debug for StatesDiscoverCmd<E, O, PKeys> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StatesDiscoverCmd").field(&self.0).finish()
    }
}

impl<E, O, PKeys> StatesDiscoverCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + Sync + Unpin + 'static,
    O: OutputWrite<E>,
    PKeys: ParamsKeys + 'static,
{
    /// Runs [`try_state_current`] for each [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], and will be serialized to
    /// `$flow_dir/states_current.yaml`.
    ///
    /// If any `state_current` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using [`Current<T>`]
    /// where `T` us the predecessor's state. Peace will have automatically
    /// inserted it into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// This function will always serialize states to storage.
    ///
    /// [`Current<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Current.html
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    pub async fn current(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesCurrent, E>, E> {
        Self::current_with(cmd_ctx, true).await
    }

    /// Runs [`try_state_current`] for each [`Item`].
    ///
    /// See [`Self::current`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// # Parameters
    ///
    /// * `cmd_ctx`: Information needed to execute a command.
    /// * `serialize_to_storage`: Whether to write states to storage after
    ///   discovery.
    ///
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    pub async fn current_with(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        serialize_to_storage: bool,
    ) -> Result<CmdOutcome<StatesCurrent, E>, E> {
        let mut cmd_execution = CmdExecution::<StatesCurrent, _, _>::builder()
            .with_cmd_block(CmdBlockWrapper::new(
                StatesDiscoverCmdBlock::current(),
                StatesCurrent::from,
            ))
            .build();

        let cmd_outcome = cmd_execution.exec(cmd_ctx).await?;

        let CmdOutcome {
            value: states_current,
            errors: _,
        } = &cmd_outcome;

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.view();
        let (item_graph, resources) = (flow.graph(), resources);

        if serialize_to_storage {
            Self::serialize_current(item_graph, resources, states_current).await?;
        }

        #[cfg(feature = "output_progress")]
        Self::progress_bar_mark_complete_on_ok(cmd_ctx, &cmd_outcome);

        Ok(cmd_outcome)
    }

    /// Runs [`try_state_goal`] for each [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesGoal`], and will be serialized to
    /// `$flow_dir/states_goal.yaml`.
    ///
    /// If any `state_goal` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using [`Goal<T>`]
    /// where `T` us the predecessor's state. Peace will have automatically
    /// inserted it into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// This function will always serialize states to storage.
    ///
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Goal<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Goal.html
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_goal`]: peace_cfg::Item::try_state_goal
    pub async fn goal(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesGoal, E>, E> {
        Self::goal_with(cmd_ctx, true).await
    }

    /// Runs [`try_state_goal`] for each [`Item`].
    ///
    /// See [`Self::goal`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// # Parameters
    ///
    /// * `cmd_ctx`: Information needed to execute a command.
    /// * `serialize_to_storage`: Whether to write states to storage after
    ///   discovery.
    ///
    /// [`try_state_goal`]: peace_cfg::Item::try_state_goal
    pub async fn goal_with(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        serialize_to_storage: bool,
    ) -> Result<CmdOutcome<StatesGoal, E>, E> {
        let mut cmd_execution = CmdExecution::<StatesGoal, _, _>::builder()
            .with_cmd_block(CmdBlockWrapper::new(
                StatesDiscoverCmdBlock::goal(),
                StatesGoal::from,
            ))
            .build();

        let cmd_outcome = cmd_execution.exec(cmd_ctx).await?;

        let CmdOutcome {
            value: states_goal,
            errors: _,
        } = &cmd_outcome;

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.view();
        let (item_graph, resources) = (flow.graph(), resources);

        if serialize_to_storage {
            Self::serialize_goal(item_graph, resources, states_goal).await?;
        }

        #[cfg(feature = "output_progress")]
        Self::progress_bar_mark_complete_on_ok(cmd_ctx, &cmd_outcome);

        Ok(cmd_outcome)
    }

    /// Runs [`try_state_current`] and [`try_state_goal`]` for each
    /// [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesGoal`], and states will be serialized
    /// to `$flow_dir/states_current.yaml` and
    /// `$flow_dir/states_goal.yaml`.
    ///
    /// If any `state_current` function needs to read the `State` from a
    /// previous `Item`, the predecessor should insert a copy / clone of
    /// their state into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// If any `state_goal` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using
    /// [`Goal<T>`] where `T` us the predecessor's state. Peace will have
    /// automatically inserted it into `Resources`, and the successor should
    /// references it in their [`Data`].
    ///
    /// This function will always serialize states to storage.
    ///
    /// [`Current<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Current.html
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Goal<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Goal.html
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    /// [`try_state_goal`]: peace_cfg::Item::try_state_goal
    pub async fn current_and_goal(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<(StatesCurrent, StatesGoal), E>, E> {
        Self::current_and_goal_with(cmd_ctx, true).await
    }

    /// Runs [`try_state_current`] and [`try_state_goal`]` for each
    /// [`Item`].
    ///
    /// See [`Self::goal`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// # Parameters
    ///
    /// * `cmd_ctx`: Information needed to execute a command.
    /// * `serialize_to_storage`: Whether to write states to storage after
    ///   discovery.
    ///
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    /// [`try_state_goal`]: peace_cfg::Item::try_state_goal
    pub async fn current_and_goal_with(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        serialize_to_storage: bool,
    ) -> Result<CmdOutcome<(StatesCurrent, StatesGoal), E>, E> {
        let mut cmd_execution = CmdExecution::<(StatesCurrent, StatesGoal), _, _>::builder()
            .with_cmd_block(CmdBlockWrapper::new(
                StatesDiscoverCmdBlock::current_and_goal(),
                |states_current_and_goal_mut| {
                    let (states_current_mut, states_goal_mut) = states_current_and_goal_mut;

                    (
                        StatesCurrent::from(states_current_mut),
                        StatesGoal::from(states_goal_mut),
                    )
                },
            ))
            .with_execution_outcome_fetch(|resources| {
                let states_current = resources.remove::<StatesCurrent>().unwrap_or_else(|| {
                    let states_current = tynm::type_name::<StatesCurrent>();
                    panic!("Expected `{states_current}` to exist in `Resources`");
                });
                let states_goal = resources.remove::<StatesGoal>().unwrap_or_else(|| {
                    let states_goal = tynm::type_name::<StatesGoal>();
                    panic!("Expected `{states_goal}` to exist in `Resources`");
                });

                (states_current, states_goal)
            })
            .build();

        let cmd_outcome = cmd_execution.exec(cmd_ctx).await?;

        let CmdOutcome {
            value: (states_current, states_goal),
            errors: _,
        } = &cmd_outcome;

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.view();
        let (item_graph, resources) = (flow.graph(), resources);

        if serialize_to_storage {
            Self::serialize_current(item_graph, resources, states_current).await?;
            Self::serialize_goal(item_graph, resources, states_goal).await?;
        }

        #[cfg(feature = "output_progress")]
        Self::progress_bar_mark_complete_on_ok(cmd_ctx, &cmd_outcome);

        Ok(cmd_outcome)
    }

    // TODO: This duplicates a bit of code with `ApplyCmd`.
    async fn serialize_current(
        item_graph: &ItemGraph<E>,
        resources: &mut Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, item_graph, states_current, &states_current_file)
            .await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(())
    }

    async fn serialize_goal(
        item_graph: &ItemGraph<E>,
        resources: &mut Resources<SetUp>,
        states_goal: &StatesGoal,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_goal_file = StatesGoalFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, item_graph, states_goal, &states_goal_file).await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_goal_file);

        Ok(())
    }

    #[cfg(feature = "output_progress")]
    fn progress_bar_mark_complete_on_ok<T>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        cmd_outcome: &CmdOutcome<T, E>,
    ) {
        cmd_ctx
            .cmd_progress_tracker_mut()
            .progress_trackers_mut()
            .iter_mut()
            .filter_map(|(item_id, progress_tracker)| {
                if cmd_outcome.errors.contains_key(item_id) {
                    None
                } else {
                    Some(progress_tracker)
                }
            })
            .for_each(|progress_tracker| {
                progress_tracker
                    .set_progress_status(ProgressStatus::Complete(ProgressComplete::Success))
            })
    }
}

impl<E, O, PKeys> Default for StatesDiscoverCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
