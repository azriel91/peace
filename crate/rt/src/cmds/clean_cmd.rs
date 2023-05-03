use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow};
use peace_resources::{
    resources::ts::SetUp,
    states::{
        ts::{Cleaned, CleanedDry},
        StatesCleaned, StatesCleanedDry, StatesSaved,
    },
};
use peace_rt_model::{outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error};

use crate::cmds::sub::{ApplyCmd, ApplyFor};

#[derive(Debug)]
pub struct CleanCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> CleanCmd<E, O, PKeys>
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
    /// The grouping of item spec functions run for a `Clean` execution to work
    /// is as follows:
    ///
    /// 1. Run [`StatesDiscoverCmd::current`] for all `ItemSpec`s in the
    ///   *forward* direction.
    ///
    ///     This populates `resources` with `Current<IS::State>`, needed for
    ///     `ItemSpec::try_state_current` during `ItemSpecRt::clean_prepare`.
    ///
    /// 2. In the *reverse* direction, for each `ItemSpec` run
    ///    `ItemSpecRt::clean_prepare`, which runs:
    ///
    ///     1. `ItemSpec::try_state_current`, which resolves parameters from
    ///        the *current* state.
    ///     2. `ItemSpec::state_desired`
    ///     3. `ItemSpec::apply_check`
    ///
    /// 3. For `ItemSpec`s that return `ApplyCheck::ExecRequired`, run
    ///    `ItemSpec::apply_exec_dry`.
    ///
    /// [`apply_exec_dry`]: peace_cfg::ItemSpec::apply_exec_dry
    /// [`ItemSpec::apply_check`]: peace_cfg::ItemSpec::apply_check
    /// [`ItemSpec::apply_exec_dry`]: peace_cfg::ItemSpecRt::apply_exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn exec_dry(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_saved: &StatesSaved,
    ) -> Result<CmdOutcome<StatesCleanedDry, E>, E> {
        ApplyCmd::<E, O, PKeys, Cleaned, CleanedDry>::exec_dry(
            cmd_ctx,
            states_saved,
            ApplyFor::Clean,
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
    /// The grouping of item spec functions run for a `Clean` execution to work
    /// is as follows:
    ///
    /// 1. Run [`StatesDiscoverCmd::current`] for all `ItemSpec`s in the
    ///   *forward* direction.
    ///
    ///     This populates `resources` with `Current<IS::State>`, needed for
    ///     `ItemSpec::try_state_current` during `ItemSpecRt::clean_prepare`.
    ///
    /// 2. In the *reverse* direction, for each `ItemSpec` run
    ///    `ItemSpecRt::clean_prepare`, which runs:
    ///
    ///     1. `ItemSpec::try_state_current`, which resolves parameters from
    ///        the *current* state.
    ///     2. `ItemSpec::state_desired`
    ///     3. `ItemSpec::apply_check`
    ///
    /// 3. For `ItemSpec`s that return `ApplyCheck::ExecRequired`, run
    ///    `ItemSpec::apply_exec`.
    ///
    /// [`apply_exec`]: peace_cfg::ItemSpec::apply_exec
    /// [`ItemSpec::apply_check`]: peace_cfg::ItemSpec::apply_check
    /// [`ItemSpec::apply_exec`]: peace_cfg::ItemSpecRt::apply_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_saved: &StatesSaved,
    ) -> Result<CmdOutcome<StatesCleaned, E>, E> {
        ApplyCmd::<E, O, PKeys, Cleaned, CleanedDry>::exec(cmd_ctx, states_saved, ApplyFor::Clean)
            .await
    }
}

impl<E, O, PKeys> Default for CleanCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
