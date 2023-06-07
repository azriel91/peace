use std::{fmt::Debug, marker::PhantomData};

use futures::FutureExt;
use peace_cfg::{ApplyCheck, FnCtx, ItemId};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_params::ParamsSpecs;
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesCurrentFile, StatesGoalFile},
    resources::ts::SetUp,
    states::{ts::Goal, States, StatesCurrent, StatesCurrentStored, StatesGoal},
    Resources,
};
use peace_rt_model::{
    outcomes::{CmdOutcome, ItemApplyBoxed, ItemApplyPartialBoxed},
    output::OutputWrite,
    params::ParamsKeys,
    Error, ItemBoxed, ItemRt, Storage,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    cmds::{cmd_ctx_internal::CmdIndependence, CmdBase, StatesDiscoverCmd},
    BUFFERED_FUTURES_MAX,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::{
            progress::{
                ProgressComplete,
                ProgressMsgUpdate,
                ProgressSender,
                ProgressUpdate,
                ProgressUpdateAndId,
            },
        };
        use tokio::sync::mpsc::Sender;
    }
}

#[derive(Debug)]
pub struct ApplyCmd<E, O, PKeys, StatesTsApply, StatesTsApplyDry>(
    PhantomData<(E, O, PKeys, StatesTsApply, StatesTsApplyDry)>,
);

impl<E, O, PKeys, StatesTsApply, StatesTsApplyDry>
    ApplyCmd<E, O, PKeys, StatesTsApply, StatesTsApplyDry>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
    StatesTsApply: Debug + Send + Sync + 'static,
    StatesTsApplyDry: Debug + Send + Sync + 'static,
    States<StatesTsApply>: From<StatesCurrent> + Send + Sync + 'static,
    States<StatesTsApplyDry>: From<StatesCurrent> + Send + Sync + 'static,
{
    /// Conditionally runs [`Item::apply_exec_dry`] for each [`Item`].
    ///
    /// In practice this runs [`Item::apply_check`], and only runs
    /// [`apply_exec_dry`] if execution is required.
    ///
    /// # Design
    ///
    /// The grouping of item functions run for an `Ensure` execution to
    /// work is as follows:
    ///
    /// 1. For each `Item` run `ItemRt::ensure_prepare`, which runs:
    ///
    ///     1. `Item::state_current`
    ///     2. `Item::state_goal`
    ///     3. `Item::apply_check`
    ///
    /// 2. For `Item`s that return `ApplyCheck::ExecRequired`, run
    ///    `Item::apply_exec_dry`.
    ///
    /// The grouping of item functions run for a `Clean` execution to work
    /// is as follows:
    ///
    /// 1. Run [`StatesDiscoverCmd::current`] for all `Item`s in the
    ///   *forward* direction.
    ///
    ///     This populates `resources` with `Current<IS::State>`, needed for
    ///     `Item::try_state_current` during `ItemRt::clean_prepare`.
    ///
    /// 2. In the *reverse* direction, for each `Item` run
    ///    `ItemRt::clean_prepare`, which runs:
    ///
    ///     1. `Item::try_state_current`, which resolves parameters from
    ///        the *current* state.
    ///     2. `Item::state_goal`
    ///     3. `Item::apply_check`
    ///
    /// 3. For `Item`s that return `ApplyCheck::ExecRequired`, run
    ///    `Item::apply_exec_dry`.
    ///
    /// [`apply_exec_dry`]: peace_cfg::Item::apply_exec_dry
    /// [`Item::apply_check`]: peace_cfg::Item::apply_check
    /// [`Item::apply_exec_dry`]: peace_cfg::ItemRt::apply_exec_dry
    /// [`Item`]: peace_cfg::Item
    pub async fn exec_dry(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_current_stored: &StatesCurrentStored,
        apply_for: ApplyFor,
    ) -> Result<CmdOutcome<States<StatesTsApplyDry>, E>, E> {
        Self::exec_dry_with(
            &mut CmdIndependence::Standalone { cmd_ctx },
            states_current_stored,
            apply_for,
        )
        .await
    }

    /// Conditionally runs [`Item::apply_exec_dry`] for each [`Item`].
    ///
    /// See [`Self::exec_dry`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    pub async fn exec_dry_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        states_current_stored: &StatesCurrentStored,
        apply_for: ApplyFor,
    ) -> Result<CmdOutcome<States<StatesTsApplyDry>, E>, E> {
        Self::exec_internal(cmd_independence, states_current_stored, apply_for, true)
            .await
            .map(|cmd_outcome| cmd_outcome.map(|(states_applied, _states_goal)| states_applied))
    }

    /// Conditionally runs [`Item::apply_exec`] for each [`Item`].
    ///
    /// In practice this runs [`Item::apply_check`], and only runs
    /// [`apply_exec`] if execution is required.
    ///
    /// # Design
    ///
    /// The grouping of item functions run for an `Ensure` execution to
    /// work is as follows:
    ///
    /// 1. For each `Item` run `ItemRt::ensure_prepare`, which runs:
    ///
    ///     1. `Item::state_current`
    ///     2. `Item::state_goal`
    ///     3. `Item::apply_check`
    ///
    /// 2. For `Item`s that return `ApplyCheck::ExecRequired`, run
    ///    `Item::apply_exec`.
    ///
    /// The grouping of item functions run for a `Clean` execution to work
    /// is as follows:
    ///
    /// 1. Run [`StatesDiscoverCmd::current`] for all `Item`s in the
    ///   *forward* direction.
    ///
    ///     This populates `resources` with `Current<IS::State>`, needed for
    ///     `Item::try_state_current` during `ItemRt::clean_prepare`.
    ///
    /// 2. In the *reverse* direction, for each `Item` run
    ///    `ItemRt::clean_prepare`, which runs:
    ///
    ///     1. `Item::try_state_current`, which resolves parameters from
    ///        the *current* state.
    ///     2. `Item::state_goal`
    ///     3. `Item::apply_check`
    ///
    /// 3. For `Item`s that return `ApplyCheck::ExecRequired`, run
    ///    `Item::apply_exec`.
    ///
    /// [`apply_exec`]: peace_cfg::Item::apply_exec
    /// [`Item::apply_check`]: peace_cfg::Item::apply_check
    /// [`Item::apply_exec`]: peace_cfg::ItemRt::apply_exec
    /// [`Item`]: peace_cfg::Item
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_current_stored: &StatesCurrentStored,
        apply_for: ApplyFor,
    ) -> Result<CmdOutcome<States<StatesTsApply>, E>, E> {
        Self::exec_with(
            &mut CmdIndependence::Standalone { cmd_ctx },
            states_current_stored,
            apply_for,
        )
        .await
    }

    /// Conditionally runs [`Item::apply_exec`] for each [`Item`].
    ///
    /// See [`Self::exec`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    pub async fn exec_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        states_current_stored: &StatesCurrentStored,
        apply_for: ApplyFor,
    ) -> Result<CmdOutcome<States<StatesTsApply>, E>, E> {
        let CmdOutcome {
            value: (states_applied, states_goal),
            errors,
        } = Self::exec_internal(cmd_independence, states_current_stored, apply_for, false).await?;

        let resources = match cmd_independence {
            CmdIndependence::Standalone { cmd_ctx } => cmd_ctx.resources(),
            CmdIndependence::SubCmd { cmd_view, .. } => cmd_view.resources,
            #[cfg(feature = "output_progress")]
            CmdIndependence::SubCmdWithProgress { cmd_view, .. } => cmd_view.resources,
        };

        Self::serialize_saved(resources, &states_applied).await?;

        match apply_for {
            ApplyFor::Ensure => {
                Self::serialize_goal(resources, &states_goal).await?;
            }
            ApplyFor::Clean => {}
        };

        let cmd_outcome = CmdOutcome {
            value: states_applied,
            errors,
        };
        Ok(cmd_outcome)
    }

    /// Conditionally runs [`ApplyFns`]`::`[`exec`] for each [`Item`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`States<StatesTsApply>`].
    ///
    /// [`exec`]: peace_cfg::ApplyFns::exec
    /// [`Item`]: peace_cfg::Item
    /// [`ApplyFns`]: peace_cfg::Item::ApplyFns
    async fn exec_internal<StatesTs>(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        states_current_stored: &StatesCurrentStored,
        apply_for: ApplyFor,
        dry_run: bool,
    ) -> Result<CmdOutcome<(States<StatesTs>, StatesGoal), E>, E>
    where
        StatesTs: Debug + Send + 'static,
    {
        // `StatesTsApply` represents the states of items *after* this cmd has run,
        // even if no change occurs. This means it should begin as `StatesCurrentStored`
        // or `StatesCurrent`, and updated when a new state has been applied and
        // re-discovered.
        //
        // Notably, the initial `StatesCurrentStored` / `StatesCurrent` may not contain
        // a state for items whose state cannot be discovered, e.g. a file on a
        // remote server, when the remote server doesn't exist.
        let states_applied_mut =
            StatesMut::<StatesTs>::from((*states_current_stored).clone().into_inner());
        let states_goal_mut = StatesMut::<Goal>::new();
        let outcome = (states_applied_mut, states_goal_mut);

        let cmd_outcome = CmdBase::exec(
            cmd_independence,
            outcome,
            |cmd_view, #[cfg(feature = "output_progress")] progress_tx, outcomes_tx| {
                async move {
                    // TODO: compare `StatesCurrentStored` and `StatesCurrent` by delegating the
                    // equality check to `ItemWrapper`.

                    let apply_for_internal = match apply_for {
                        ApplyFor::Ensure => ApplyForInternal::Ensure,
                        ApplyFor::Clean => {
                            let states_current_outcome =
                                StatesDiscoverCmd::<E, O, PKeys>::current_with(
                                    #[cfg(not(feature = "output_progress"))]
                                    &mut CmdIndependence::SubCmd { cmd_view },
                                    #[cfg(feature = "output_progress")]
                                    &mut CmdIndependence::SubCmdWithProgress {
                                        cmd_view,
                                        progress_tx: progress_tx.clone(),
                                    },
                                )
                                .await;
                            let states_current_outcome = match states_current_outcome {
                                Ok(states_current_outcome) => states_current_outcome,
                                Err(error) => {
                                    outcomes_tx
                                        .send(ApplyExecOutcome::DiscoverCmdError { error })
                                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                                    return;
                                }
                            };
                            if states_current_outcome.is_err() {
                                let outcome = states_current_outcome.map(|states_current| {
                                    (
                                        StatesMut::<StatesTs>::from(states_current.into_inner()),
                                        StatesMut::<Goal>::new(),
                                    )
                                });
                                outcomes_tx
                                    .send(ApplyExecOutcome::DiscoverOutcomeError { outcome })
                                    .expect("unreachable: `outcomes_rx` is in a sibling task.");
                                return;
                            }

                            let CmdOutcome {
                                value: states_current,
                                errors: _,
                            } = states_current_outcome;

                            ApplyForInternal::Clean { states_current }
                        }
                    };
                    let apply_for_internal = &apply_for_internal;

                    let SingleProfileSingleFlowView {
                        flow,
                        params_specs,
                        resources,
                        ..
                    } = cmd_view;

                    let item_graph = flow.graph();

                    let resources_ref = &*resources;
                    match apply_for {
                        ApplyFor::Ensure => {
                            let (Ok(()) | Err(())) = item_graph
                                .try_for_each_concurrent(BUFFERED_FUTURES_MAX, |item| {
                                    Self::item_apply_exec(
                                        params_specs,
                                        resources_ref,
                                        apply_for_internal,
                                        #[cfg(feature = "output_progress")]
                                        progress_tx,
                                        outcomes_tx,
                                        item,
                                        dry_run,
                                    )
                                })
                                .await
                                .map_err(|_vec_units: Vec<()>| ());
                        }
                        ApplyFor::Clean => {
                            let (Ok(()) | Err(())) = item_graph
                                .try_for_each_concurrent_rev(BUFFERED_FUTURES_MAX, |item| {
                                    Self::item_apply_exec(
                                        params_specs,
                                        resources_ref,
                                        apply_for_internal,
                                        #[cfg(feature = "output_progress")]
                                        progress_tx,
                                        outcomes_tx,
                                        item,
                                        dry_run,
                                    )
                                })
                                .await
                                .map_err(|_vec_units: Vec<()>| ());
                        }
                    }
                }
                .boxed_local()
            },
            |cmd_outcome, apply_exec_outcome| {
                let CmdOutcome {
                    value: (states_applied_mut, states_goal_mut),
                    errors,
                } = cmd_outcome;
                match apply_exec_outcome {
                    ApplyExecOutcome::DiscoverCmdError { error } => return Err(error),
                    ApplyExecOutcome::DiscoverOutcomeError { mut outcome } => {
                        std::mem::swap(&mut outcome.value.0, states_applied_mut);
                        std::mem::swap(&mut outcome.value.1, states_goal_mut);
                    }
                    ApplyExecOutcome::ItemApply(item_apply_outcome) => match item_apply_outcome {
                        ItemApplyOutcome::PrepareFail {
                            item_id,
                            item_apply_partial,
                            error,
                        } => {
                            errors.insert(item_id.clone(), error);

                            // Save `state_target` (which is state_goal) if we are not cleaning
                            // up.
                            match apply_for {
                                ApplyFor::Ensure => {
                                    if let Some(state_goal) = item_apply_partial.state_target() {
                                        states_goal_mut.insert_raw(item_id, state_goal);
                                    }
                                }
                                ApplyFor::Clean => {}
                            }
                        }
                        ItemApplyOutcome::Success {
                            item_id,
                            item_apply,
                        } => {
                            if let Some(state_applied) = item_apply.state_applied() {
                                states_applied_mut.insert_raw(item_id.clone(), state_applied);
                            } else {
                                // Item was already in the goal state.
                                // No change to saved state.
                            }

                            // Save `state_target` (which is state_goal) if we are not cleaning
                            // up.
                            match apply_for {
                                ApplyFor::Ensure => {
                                    let state_goal = item_apply.state_target();
                                    states_goal_mut.insert_raw(item_id, state_goal);
                                }
                                ApplyFor::Clean => {}
                            }
                        }
                        ItemApplyOutcome::Fail {
                            item_id,
                            item_apply,
                            error,
                        } => {
                            errors.insert(item_id.clone(), error);
                            if let Some(state_applied) = item_apply.state_applied() {
                                states_applied_mut.insert_raw(item_id.clone(), state_applied);
                            }

                            // Save `state_target` (which is state_goal) if we are not cleaning
                            // up.
                            match apply_for {
                                ApplyFor::Ensure => {
                                    let state_goal = item_apply.state_target();
                                    states_goal_mut.insert_raw(item_id, state_goal);
                                }
                                ApplyFor::Clean => {}
                            }
                        }
                    },
                }

                Ok(())
            },
        )
        .await?;

        // TODO: Should we run `StatesCurrentFn` again?
        //
        // i.e. is it part of `ApplyFns::exec`'s contract to return the state.
        //
        // * It may be duplication of code.
        // * `FileDownloadItem` needs to know the ETag from the last request, which:
        //     - in `StatesCurrentFn` comes from `StatesCurrent`
        //     - in `ApplyCmd` comes from `StatesTsApply`
        // * `ShCmdItem` doesn't return the state in the apply script, so in the item we
        //   run the state current script after the apply exec script.

        let cmd_outcome = cmd_outcome.map(|(states_applied_mut, states_goal_mut)| {
            let states_applied: States<StatesTs> = states_applied_mut.into();
            let states_goal: StatesGoal = states_goal_mut.into();

            (states_applied, states_goal)
        });

        Ok(cmd_outcome)
    }

    ///
    /// # Implementation Note
    ///
    /// Tried passing through the function to execute instead of a `dry_run`
    /// parameter, but couldn't convince the compiler that the lifetimes match
    /// up:
    ///
    /// ```rust,ignore
    /// async fn item_apply_exec<F, Fut>(
    ///     resources: &Resources<SetUp>,
    ///     outcomes_tx: &UnboundedSender<ItemApplyOutcome<E>>,
    ///     item: FnRef<'_, ItemBoxed<E>>,
    ///     f: F,
    /// ) -> bool
    /// where
    ///     F: (Fn(&dyn ItemRt<E>, fn_ctx: OpCtx<'_>, &Resources<SetUp>, &mut ItemApplyBoxed) -> Fut) + Copy,
    ///     Fut: Future<Output = Result<(), E>>,
    /// ```
    async fn item_apply_exec<StatesTs>(
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        apply_for_internal: &ApplyForInternal,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
        outcomes_tx: &UnboundedSender<ApplyExecOutcome<E, StatesTs>>,
        item: &ItemBoxed<E>,
        dry_run: bool,
    ) -> Result<(), ()>
    where
        StatesTs: Debug + Send,
    {
        let apply_fn = if dry_run {
            ItemRt::apply_exec_dry
        } else {
            ItemRt::apply_exec
        };

        let item_id = item.id();
        let fn_ctx = FnCtx::new(
            item_id,
            #[cfg(feature = "output_progress")]
            ProgressSender::new(item_id, progress_tx),
        );
        let item_apply = match apply_for_internal {
            ApplyForInternal::Ensure => {
                ItemRt::ensure_prepare(&**item, params_specs, resources, fn_ctx).await
            }
            ApplyForInternal::Clean { states_current } => {
                ItemRt::clean_prepare(&**item, states_current, params_specs, resources).await
            }
        };

        match item_apply {
            Ok(mut item_apply) => {
                match item_apply.apply_check() {
                    #[cfg(not(feature = "output_progress"))]
                    ApplyCheck::ExecRequired => {}
                    #[cfg(feature = "output_progress")]
                    ApplyCheck::ExecRequired { progress_limit } => {
                        // Update `OutputWrite`s with progress limit.
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_id: item_id.clone(),
                            progress_update: ProgressUpdate::Limit(progress_limit),
                            msg_update: ProgressMsgUpdate::Set(String::from("in progress")),
                        });
                    }
                    ApplyCheck::ExecNotRequired => {
                        #[cfg(feature = "output_progress")]
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_id: item_id.clone(),
                            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
                            msg_update: ProgressMsgUpdate::Set(String::from("nothing to do!")),
                        });

                        // TODO: write test for this case
                        // In case of an interrupt or power failure, we may not have written states
                        // to disk.
                        outcomes_tx
                            .send(ApplyExecOutcome::ItemApply(ItemApplyOutcome::Success {
                                item_id: item.id().clone(),
                                item_apply,
                            }))
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        // short-circuit
                        return Ok(());
                    }
                }
                match apply_fn(&**item, params_specs, resources, fn_ctx, &mut item_apply).await {
                    Ok(()) => {
                        // apply succeeded

                        #[cfg(feature = "output_progress")]
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_id: item_id.clone(),
                            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
                            msg_update: ProgressMsgUpdate::Set(String::from("done!")),
                        });

                        outcomes_tx
                            .send(ApplyExecOutcome::ItemApply(ItemApplyOutcome::Success {
                                item_id: item.id().clone(),
                                item_apply,
                            }))
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        Ok(())
                    }
                    Err(error) => {
                        // apply failed

                        #[cfg(feature = "output_progress")]
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_id: item_id.clone(),
                            progress_update: ProgressUpdate::Complete(ProgressComplete::Fail),
                            msg_update: ProgressMsgUpdate::Set(
                                error
                                    .source()
                                    .map(|source| format!("{source}"))
                                    .unwrap_or_else(|| format!("{error}")),
                            ),
                        });

                        outcomes_tx
                            .send(ApplyExecOutcome::ItemApply(ItemApplyOutcome::Fail {
                                item_id: item.id().clone(),
                                item_apply,
                                error,
                            }))
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        // we should stop processing.
                        Err(())
                    }
                }
            }
            Err((error, item_apply_partial)) => {
                #[cfg(feature = "output_progress")]
                let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                    item_id: item.id().clone(),
                    progress_update: ProgressUpdate::Complete(ProgressComplete::Fail),
                    msg_update: ProgressMsgUpdate::Set(
                        error
                            .source()
                            .map(|source| format!("{source}"))
                            .unwrap_or_else(|| format!("{error}")),
                    ),
                });

                outcomes_tx
                    .send(ApplyExecOutcome::ItemApply(ItemApplyOutcome::PrepareFail {
                        item_id: item.id().clone(),
                        item_apply_partial,
                        error,
                    }))
                    .expect("unreachable: `outcomes_rx` is in a sibling task.");

                Err(())
            }
        }
    }

    // TODO: This duplicates a bit of code with `StatesDiscoverCmd`,
    async fn serialize_saved(
        resources: &Resources<SetUp>,
        states_applied: &States<StatesTsApply>,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_applied, &states_current_file).await?;

        drop(flow_dir);
        drop(storage);

        Ok(())
    }

    async fn serialize_goal(
        resources: &Resources<SetUp>,
        states_goal: &StatesGoal,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_goal_file = StatesGoalFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_goal, &states_goal_file).await?;

        drop(flow_dir);
        drop(storage);

        Ok(())
    }
}

impl<E, O, PKeys, StatesTsApply, StatesTsApplyDry> Default
    for ApplyCmd<E, O, PKeys, StatesTsApply, StatesTsApplyDry>
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

///
///
/// For cleaning up items, current states are discovered to populate `Resources`
/// with the current state.
#[derive(Debug)]
enum ApplyExecOutcome<E, StatesTs> {
    /// Error occurred during state current discovery.
    ///
    /// This variant is when the error is due to the command logic failing,
    /// rather than an error from an item's discovery.
    ///
    /// For cleaning up items, current states are discovered to populate
    /// `Resources` with the current state.
    DiscoverCmdError {
        /// The error from state current discovery.
        error: E,
    },
    /// Error discovering current state for items.
    DiscoverOutcomeError {
        /// Outcome of state discovery.
        outcome: CmdOutcome<(StatesMut<StatesTs>, StatesMut<Goal>), E>,
    },
    /// An item apply outcome.
    ItemApply(ItemApplyOutcome<E>),
}

#[derive(Debug)]
enum ItemApplyOutcome<E> {
    /// Error occurred when discovering current state, goal states, state
    /// diff, or `ApplyCheck`.
    PrepareFail {
        item_id: ItemId,
        item_apply_partial: ItemApplyPartialBoxed,
        error: E,
    },
    /// Ensure execution succeeded.
    Success {
        item_id: ItemId,
        item_apply: ItemApplyBoxed,
    },
    /// Ensure execution failed.
    Fail {
        item_id: ItemId,
        item_apply: ItemApplyBoxed,
        error: E,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ApplyFor {
    Ensure,
    Clean,
}

#[derive(Debug)]
enum ApplyForInternal {
    Ensure,
    Clean { states_current: StatesCurrent },
}
