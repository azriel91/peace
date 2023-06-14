use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow, CmdIndependence};
use peace_resources::{
    resources::ts::SetUp,
    states::{
        ts::{Ensured, EnsuredDry},
        StatesEnsured, StatesEnsuredDry,
    },
};
use peace_rt_model::{outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error};

use crate::cmds::{
    sub::{ApplyCmd, ApplyFor},
    ApplyStoredStateSync,
};

#[derive(Debug)]
pub struct EnsureCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> EnsureCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    /// Conditionally runs [`Item::apply_exec_dry`] for each
    /// [`Item`].
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
    /// [`apply_exec_dry`]: peace_cfg::Item::apply_exec_dry
    /// [`Item::apply_check`]: peace_cfg::Item::apply_check
    /// [`Item::apply_exec_dry`]: peace_cfg::ItemRt::apply_exec_dry
    /// [`Item`]: peace_cfg::Item
    pub async fn exec_dry(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesEnsuredDry, E>, E> {
        ApplyCmd::<E, O, PKeys, Ensured, EnsuredDry>::exec_dry(cmd_ctx, ApplyFor::Ensure).await
    }

    /// Runs [`Item::apply_exec_dry`] for each [`Item`], with [`state_goal`]
    /// as the target state.
    ///
    /// See [`Self::exec_dry`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// [`Item`]: peace_cfg::Item
    /// [`state_goal`]: peace_cfg::Item::state_goal
    pub async fn exec_dry_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        apply_stored_state_sync: ApplyStoredStateSync,
    ) -> Result<CmdOutcome<StatesEnsuredDry, E>, E> {
        ApplyCmd::<E, O, PKeys, Ensured, EnsuredDry>::exec_dry_with(
            cmd_independence,
            ApplyFor::Ensure,
            apply_stored_state_sync,
        )
        .await
    }

    /// Conditionally runs [`Item::apply_exec`] for each
    /// [`Item`].
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
    /// [`apply_exec`]: peace_cfg::Item::apply_exec
    /// [`Item::apply_check`]: peace_cfg::Item::apply_check
    /// [`Item::apply_exec`]: peace_cfg::ItemRt::apply_exec
    /// [`Item`]: peace_cfg::Item
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesEnsured, E>, E> {
        ApplyCmd::<E, O, PKeys, Ensured, EnsuredDry>::exec(cmd_ctx, ApplyFor::Ensure).await
    }

    /// Runs [`Item::apply_exec`] for each [`Item`], with [`state_goal`] as
    /// the target state.
    ///
    /// See [`Self::exec`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// [`Item`]: peace_cfg::Item
    /// [`state_goal`]: peace_cfg::Item::state_goal
    pub async fn exec_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        apply_stored_state_sync: ApplyStoredStateSync,
    ) -> Result<CmdOutcome<StatesEnsured, E>, E> {
        ApplyCmd::<E, O, PKeys, Ensured, EnsuredDry>::exec_with(
            cmd_independence,
            ApplyFor::Ensure,
            apply_stored_state_sync,
        )
        .await
    }
}

impl<E, O, PKeys> Default for EnsureCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
