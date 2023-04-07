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
    /// Conditionally runs [`ApplyFns`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`ApplyFns::check`], and only runs
    /// [`exec_dry`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`ApplyFns::check`] for all `ItemSpec`s.
    /// 2. Run [`ApplyFns::exec_dry`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec_dry` as it may use
    /// different `Data`.
    ///
    /// [`exec_dry`]: peace_cfg::ApplyFns::exec_dry
    /// [`ApplyFns::check`]: peace_cfg::ApplyFns::check
    /// [`ApplyFns::exec_dry`]: peace_cfg::ApplyFns::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ApplyFns`]: peace_cfg::ItemSpec::ApplyFns
    pub async fn exec_dry(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_saved: &StatesSaved,
    ) -> Result<CmdOutcome<StatesCleanedDry, E>, E> {
        Ok(ApplyCmd::<E, O, PKeys, Cleaned, CleanedDry>::exec_dry(
            cmd_ctx,
            states_saved,
            ApplyFor::Clean,
        )
        .await)
    }

    /// Conditionally runs [`ApplyFns`]`::`[`exec`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`ApplyFns::check`], and only runs
    /// [`exec`] if execution is required.
    ///
    /// This function takes in a `StatesSaved`, but if you retrieve the state
    /// within the same execution, and have a `StatesCurrent`, you can turn this
    /// into `StatesSaved` by using `StatesSaved::from(states_current)` or
    /// calling the `.into()` method.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`ApplyFns::check`] for all `ItemSpec`s.
    /// 2. Run [`ApplyFns::exec`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use
    /// different `Data`.
    ///
    /// [`exec`]: peace_cfg::ApplyFns::exec
    /// [`ApplyFns::check`]: peace_cfg::ApplyFns::check
    /// [`ApplyFns::exec`]: peace_cfg::ApplyFns::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ApplyFns`]: peace_cfg::ItemSpec::ApplyFns
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
