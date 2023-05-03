use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow};
use peace_resources::{
    resources::ts::SetUp,
    states::{
        ts::{Ensured, EnsuredDry},
        StatesEnsured, StatesEnsuredDry, StatesSaved,
    },
};
use peace_rt_model::{outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error};

use crate::cmds::sub::{ApplyCmd, ApplyFor};

#[derive(Debug)]
pub struct EnsureCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> EnsureCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    /// Conditionally runs [`ItemSpec::apply_exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`ItemSpec::apply_check`], and only runs
    /// [`apply_exec_dry`] if execution is required.
    ///
    /// # Design
    ///
    /// The grouping of item spec functions run for an `Ensure` execution to
    /// work is as follows:
    ///
    /// 1. For each `ItemSpec` run `ItemSpecRt::ensure_prepare`, which runs:
    ///
    ///     1. `ItemSpec::state_current`
    ///     2. `ItemSpec::state_desired`
    ///     3. `ItemSpec::apply_check`
    ///
    /// 2. For `ItemSpec`s that return `ApplyCheck::ExecRequired`, run
    ///    `ItemSpec::apply_exec_dry`.
    ///
    /// [`apply_exec_dry`]: peace_cfg::ItemSpec::apply_exec_dry
    /// [`ItemSpec::apply_check`]: peace_cfg::ItemSpec::apply_check
    /// [`ItemSpec::apply_exec_dry`]: peace_cfg::ItemSpecRt::apply_exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn exec_dry(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_saved: &StatesSaved,
    ) -> Result<CmdOutcome<StatesEnsuredDry, E>, E> {
        ApplyCmd::<E, O, PKeys, Ensured, EnsuredDry>::exec_dry(
            cmd_ctx,
            states_saved,
            ApplyFor::Ensure,
        )
        .await
    }

    /// Conditionally runs [`ItemSpec::apply_exec`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`ItemSpec::apply_check`], and only runs
    /// [`apply_exec`] if execution is required.
    ///
    /// # Design
    ///
    /// The grouping of item spec functions run for an `Ensure` execution to
    /// work is as follows:
    ///
    /// 1. For each `ItemSpec` run `ItemSpecRt::ensure_prepare`, which runs:
    ///
    ///     1. `ItemSpec::state_current`
    ///     2. `ItemSpec::state_desired`
    ///     3. `ItemSpec::apply_check`
    ///
    /// 2. For `ItemSpec`s that return `ApplyCheck::ExecRequired`, run
    ///    `ItemSpec::apply_exec`.
    ///
    /// [`apply_exec`]: peace_cfg::ItemSpec::apply_exec
    /// [`ItemSpec::apply_check`]: peace_cfg::ItemSpec::apply_check
    /// [`ItemSpec::apply_exec`]: peace_cfg::ItemSpecRt::apply_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_saved: &StatesSaved,
    ) -> Result<CmdOutcome<StatesEnsured, E>, E> {
        ApplyCmd::<E, O, PKeys, Ensured, EnsuredDry>::exec(cmd_ctx, states_saved, ApplyFor::Ensure)
            .await
    }
}

impl<E, O, PKeys> Default for EnsureCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
