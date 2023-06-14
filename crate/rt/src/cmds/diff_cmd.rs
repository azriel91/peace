use std::{fmt::Debug, marker::PhantomData};

use futures::{FutureExt, StreamExt, TryStreamExt};
use peace_cfg::{ItemId, Profile};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{
        MultiProfileSingleFlow, MultiProfileSingleFlowView, SingleProfileSingleFlow,
        SingleProfileSingleFlowView,
    },
    CmdIndependence,
};
use peace_params::ParamsSpecs;
use peace_resources::{
    internal::StateDiffsMut,
    resources::ts::SetUp,
    states::{
        StateDiffs, States, StatesCurrent, StatesCurrentStored, StatesGoal, StatesGoalStored,
    },
    type_reg::untagged::{BoxDtDisplay, TypeMap},
    Resources,
};
use peace_rt_model::{outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error, Flow};

use crate::cmds::{CmdBase, StatesCurrentReadCmd, StatesDiscoverCmd, StatesGoalReadCmd};

pub use self::{diff_info_spec::DiffInfoSpec, diff_state_spec::DiffStateSpec};

mod diff_info_spec;
mod diff_state_spec;

#[derive(Debug)]
pub struct DiffCmd<'cmd, E, O, PKeys, Scope>(PhantomData<(E, &'cmd O, PKeys, Scope)>);

impl<'cmd, E, O, PKeys>
    DiffCmd<'cmd, E, O, PKeys, SingleProfileSingleFlow<'cmd, E, O, PKeys, SetUp>>
where
    E: std::error::Error + From<Error> + Send + 'static,
    O: OutputWrite<E> + 'cmd,
    PKeys: ParamsKeys + 'static,
{
    /// Returns the [`state_diff`]`s between the stored current and goal
    /// states.
    ///
    /// Both current and goal states must have been discovered prior to
    /// running this. See [`StatesDiscoverCmd::current_and_goal`].
    ///
    /// This is equivalent to calling:
    ///
    /// ```rust,ignore
    /// DiffCmd::diff(cmd_ctx, DiffStateSpec::CurrentStored, DiffStateSpec::GoalStored).await?;
    /// ```
    ///
    /// [`state_diff`]: peace_cfg::Item::state_diff
    /// [`StatesDiscoverCmd::current_and_goal`]: crate::cmds::StatesDiscoverCmd::current_and_goal
    pub async fn diff_stored(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StateDiffs, E> {
        Self::diff_stored_with(&mut cmd_ctx.as_standalone()).await
    }

    /// Returns the [`state_diff`]`s between the stored current and goal
    /// states.
    ///
    /// See [`Self::diff_stored`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    pub async fn diff_stored_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
    ) -> Result<StateDiffs, E> {
        Self::diff_with(
            cmd_independence,
            DiffStateSpec::CurrentStored,
            DiffStateSpec::GoalStored,
        )
        .await
        .map(|cmd_outcome| cmd_outcome.value)
    }

    /// Returns the [`state_diff`]`s between two states.
    ///
    /// Both current and goal states must have been discovered prior to
    /// running this. See [`StatesDiscoverCmd::current_and_goal`].
    ///
    /// [`state_diff`]: peace_cfg::Item::state_diff
    /// [`StatesDiscoverCmd::current_and_goal`]: crate::cmds::StatesDiscoverCmd::current_and_goal
    pub async fn diff(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        diff_state_spec_a: DiffStateSpec,
        diff_state_spec_b: DiffStateSpec,
    ) -> Result<CmdOutcome<StateDiffs, E>, E> {
        Self::diff_with(
            &mut cmd_ctx.as_standalone(),
            diff_state_spec_a,
            diff_state_spec_b,
        )
        .await
    }

    /// Returns the [`state_diff`]`s between two states.
    ///
    /// Both current and goal states must have been discovered prior to
    /// running this. See [`StatesDiscoverCmd::current_and_goal`].
    ///
    /// [`state_diff`]: peace_cfg::Item::state_diff
    /// [`StatesDiscoverCmd::current_and_goal`]: crate::cmds::StatesDiscoverCmd::current_and_goal
    pub async fn diff_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        diff_state_spec_a: DiffStateSpec,
        diff_state_spec_b: DiffStateSpec,
    ) -> Result<CmdOutcome<StateDiffs, E>, E> {
        let states_a = {
            let states_a_outcome =
                Self::states_retrieve(cmd_independence, diff_state_spec_a).await?;
            if states_a_outcome.is_err() {
                return Ok(states_a_outcome.map(|_| StateDiffs::new()));
            }
            states_a_outcome.value
        };

        let states_b = {
            let states_b_outcome =
                Self::states_retrieve(cmd_independence, diff_state_spec_b).await?;
            if states_b_outcome.is_err() {
                return Ok(states_b_outcome.map(|_| StateDiffs::new()));
            }
            states_b_outcome.value
        };

        // The actual diff calculation should be fast, so we don't render progress.
        CmdBase::oneshot(cmd_independence, move |cmd_view| {
            async move {
                let SingleProfileSingleFlowView {
                    flow,
                    params_specs,
                    resources,
                    ..
                } = cmd_view;

                Self::diff_any(flow, params_specs, resources, &states_a, &states_b)
                    .await
                    .map(CmdOutcome::new)
            }
            .boxed_local()
        })
        .await
    }

    async fn states_retrieve(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        diff_state_spec: DiffStateSpec,
    ) -> Result<CmdOutcome<TypeMap<ItemId, BoxDtDisplay>, E>, E> {
        match diff_state_spec {
            DiffStateSpec::Current => {
                let states_outcome = CmdBase::exec(
                    cmd_independence,
                    StatesCurrent::new(),
                    |cmd_view, #[cfg(feature = "output_progress")] progress_tx, outcomes_tx| {
                        async move {
                            #[cfg(not(feature = "output_progress"))]
                            let mut cmd_independence: CmdIndependence<
                                '_,
                                '_,
                                '_,
                                E,
                                O,
                                PKeys,
                            > = CmdIndependence::SubCmd { cmd_view };
                            #[cfg(feature = "output_progress")]
                            let mut cmd_independence: CmdIndependence<
                                '_,
                                '_,
                                '_,
                                E,
                                O,
                                PKeys,
                            > = CmdIndependence::SubCmdWithProgress {
                                cmd_view,
                                progress_tx: progress_tx.clone(),
                            };

                            let states_current_outcome =
                                StatesDiscoverCmd::current_with(&mut cmd_independence, true).await;
                            match states_current_outcome {
                                Ok(states_current_outcome) => {
                                    outcomes_tx
                                        .send(DiffExecOutcome::DiscoverOutcome {
                                            outcome: states_current_outcome,
                                        })
                                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                                }
                                Err(error) => {
                                    outcomes_tx
                                        .send(DiffExecOutcome::DiscoverExecError { error })
                                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                                }
                            }
                        }
                        .boxed_local()
                    },
                    |cmd_outcome, diff_exec_outcome| match diff_exec_outcome {
                        DiffExecOutcome::DiscoverExecError { error } => Err(error),
                        DiffExecOutcome::DiscoverOutcome { mut outcome } => {
                            std::mem::swap(cmd_outcome, &mut outcome);
                            Ok(())
                        }
                    },
                )
                .await?;

                let states_outcome = if states_outcome.is_err() {
                    states_outcome.map(|_| TypeMap::<ItemId, BoxDtDisplay>::new_typed())
                } else {
                    states_outcome.map(StatesCurrent::into_inner)
                };

                Ok(states_outcome)
            }

            DiffStateSpec::CurrentStored => CmdBase::oneshot(cmd_independence, |cmd_view| {
                async move {
                    let mut cmd_independence: CmdIndependence<'_, '_, '_, E, O, PKeys> =
                        CmdIndependence::SubCmd { cmd_view };
                    StatesCurrentReadCmd::exec_with(&mut cmd_independence).await
                }
                .boxed_local()
            })
            .await
            .map(StatesCurrentStored::into_inner)
            .map(CmdOutcome::new),

            DiffStateSpec::Goal => {
                let states_outcome = CmdBase::exec(
                    cmd_independence,
                    StatesGoal::new(),
                    |cmd_view, #[cfg(feature = "output_progress")] progress_tx, outcomes_tx| {
                        async move {
                            #[cfg(not(feature = "output_progress"))]
                            let mut cmd_independence: CmdIndependence<
                                '_,
                                '_,
                                '_,
                                E,
                                O,
                                PKeys,
                            > = CmdIndependence::SubCmd { cmd_view };
                            #[cfg(feature = "output_progress")]
                            let mut cmd_independence: CmdIndependence<
                                '_,
                                '_,
                                '_,
                                E,
                                O,
                                PKeys,
                            > = CmdIndependence::SubCmdWithProgress {
                                cmd_view,
                                progress_tx: progress_tx.clone(),
                            };

                            let states_goal_outcome =
                                StatesDiscoverCmd::goal_with(&mut cmd_independence, true).await;
                            match states_goal_outcome {
                                Ok(states_goal_outcome) => {
                                    outcomes_tx
                                        .send(DiffExecOutcome::DiscoverOutcome {
                                            outcome: states_goal_outcome,
                                        })
                                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                                }
                                Err(error) => {
                                    outcomes_tx
                                        .send(DiffExecOutcome::DiscoverExecError { error })
                                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                                }
                            }
                        }
                        .boxed_local()
                    },
                    |cmd_outcome, diff_exec_outcome| match diff_exec_outcome {
                        DiffExecOutcome::DiscoverExecError { error } => Err(error),
                        DiffExecOutcome::DiscoverOutcome { mut outcome } => {
                            std::mem::swap(cmd_outcome, &mut outcome);
                            Ok(())
                        }
                    },
                )
                .await?;

                let states_outcome = if states_outcome.is_err() {
                    states_outcome.map(|_| TypeMap::<ItemId, BoxDtDisplay>::new_typed())
                } else {
                    states_outcome.map(StatesGoal::into_inner)
                };

                Ok(states_outcome)
            }

            DiffStateSpec::GoalStored => CmdBase::oneshot(cmd_independence, |cmd_view| {
                async move {
                    let mut cmd_independence: CmdIndependence<'_, '_, '_, E, O, PKeys> =
                        CmdIndependence::SubCmd { cmd_view };
                    StatesGoalReadCmd::exec_with(&mut cmd_independence).await
                }
                .boxed_local()
            })
            .await
            .map(StatesGoalStored::into_inner)
            .map(CmdOutcome::new),
        }
    }
}

impl<'cmd, E, O, PKeys> DiffCmd<'cmd, E, O, PKeys, MultiProfileSingleFlow<'cmd, E, O, PKeys, SetUp>>
where
    E: std::error::Error + From<Error> + Send + 'static,
    O: OutputWrite<E> + 'cmd,
    PKeys: ParamsKeys + 'static,
{
    /// Returns the [`state_diff`]`s between the stored current states of two
    /// profiles.
    ///
    /// Both profiles' current states must have been discovered prior to
    /// running this. See [`StatesDiscoverCmd::current`].
    ///
    /// [`state_diff`]: peace_cfg::Item::state_diff
    /// [`StatesDiscoverCmd::current`]: crate::cmds::StatesDiscoverCmd::current
    pub async fn diff_current_stored(
        cmd_ctx: &mut CmdCtx<MultiProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        profile_a: &Profile,
        profile_b: &Profile,
    ) -> Result<StateDiffs, E> {
        let MultiProfileSingleFlowView {
            flow,
            profiles,
            profile_to_params_specs,
            profile_to_states_current_stored,
            resources,
            ..
        } = cmd_ctx.view();

        let params_specs = profile_to_params_specs
            .get(profile_a)
            .or_else(|| profile_to_params_specs.get(profile_b));
        let params_specs = if let Some(Some(params_specs)) = params_specs {
            params_specs
        } else {
            Err(Error::ParamsSpecsNotDefinedForDiff {
                profile_a: profile_a.clone(),
                profile_b: profile_b.clone(),
            })?
        };
        let states_a = profile_to_states_current_stored
            .get(profile_a)
            .ok_or_else(|| {
                let profile = profile_a.clone();
                let profiles_in_scope = profiles.to_vec();
                Error::ProfileNotInScope {
                    profile,
                    profiles_in_scope,
                }
            })?
            .as_ref()
            .ok_or_else(|| {
                let profile = profile_a.clone();
                Error::ProfileStatesCurrentNotDiscovered { profile }
            })?;
        let states_b = profile_to_states_current_stored
            .get(profile_b)
            .ok_or_else(|| {
                let profile = profile_b.clone();
                let profiles_in_scope = profiles.to_vec();
                Error::ProfileNotInScope {
                    profile,
                    profiles_in_scope,
                }
            })?
            .as_ref()
            .ok_or_else(|| {
                let profile = profile_b.clone();
                Error::ProfileStatesCurrentNotDiscovered { profile }
            })?;

        Self::diff_any(flow, params_specs, resources, states_a, states_b).await
    }
}

impl<'cmd, E, O, PKeys, Scope> DiffCmd<'cmd, E, O, PKeys, Scope>
where
    E: std::error::Error + From<Error> + Send + 'static,
    O: OutputWrite<E> + 'cmd,
    PKeys: ParamsKeys + 'static,
{
    /// Returns the [`state_diff`]` for each [`Item`].
    ///
    /// This does not take in `CmdCtx` as it may be used by both
    /// `SingleProfileSingleFlow` and `MultiProfileSingleFlow`
    /// commands.
    ///
    /// [`Item`]: peace_cfg::Item
    /// [`state_diff`]: peace_cfg::Item::state_diff
    pub async fn diff_any(
        flow: &Flow<E>,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        states_a: &TypeMap<ItemId, BoxDtDisplay>,
        states_b: &TypeMap<ItemId, BoxDtDisplay>,
    ) -> Result<StateDiffs, E> {
        let state_diffs = {
            let state_diffs_mut = flow
                .graph()
                .stream()
                .map(Result::<_, E>::Ok)
                .try_filter_map(|item| async move {
                    let state_diff_opt = item
                        .state_diff_exec(params_specs, resources, states_a, states_b)
                        .await?;

                    Ok(state_diff_opt.map(|state_diff| (item.id().clone(), state_diff)))
                })
                .try_collect::<StateDiffsMut>()
                .await?;

            StateDiffs::from(state_diffs_mut)
        };

        Ok(state_diffs)
    }
}

impl<'cmd, E, O, PKeys, Scope> Default for DiffCmd<'cmd, E, O, PKeys, Scope> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Sub-outcomes of discover execution.
#[derive(Debug)]
pub enum DiffExecOutcome<E, StatesTs> {
    /// An error occurred in the state discovery command implementation.
    ///
    /// This variant is when the error is due to the command logic failing,
    /// rather than an error from an item's discovery.
    DiscoverExecError {
        /// The error from state discovery.
        error: E,
    },
    /// Outcome of the discover command.
    ///
    /// This may be successful or contain an error.
    DiscoverOutcome {
        /// Outcome of state discovery.
        outcome: CmdOutcome<States<StatesTs>, E>,
    },
}
