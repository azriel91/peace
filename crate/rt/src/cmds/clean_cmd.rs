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
    /// Conditionally runs [`ApplyOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`ApplyOpSpec::check`], and only runs
    /// [`exec_dry`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`ApplyOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`ApplyOpSpec::exec_dry`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec_dry` as it may use
    /// different `Data`.
    ///
    /// [`exec_dry`]: peace_cfg::ApplyOpSpec::exec_dry
    /// [`ApplyOpSpec::check`]: peace_cfg::ApplyOpSpec::check
    /// [`ApplyOpSpec::exec_dry`]: peace_cfg::ApplyOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ApplyOpSpec`]: peace_cfg::ItemSpec::ApplyOpSpec
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

    /// Conditionally runs [`ApplyOpSpec`]`::`[`exec`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`ApplyOpSpec::check`], and only runs
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
    /// 1. Run [`ApplyOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`ApplyOpSpec::exec`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use
    /// different `Data`.
    ///
    /// [`exec`]: peace_cfg::ApplyOpSpec::exec
    /// [`ApplyOpSpec::check`]: peace_cfg::ApplyOpSpec::check
    /// [`ApplyOpSpec::exec`]: peace_cfg::ApplyOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ApplyOpSpec`]: peace_cfg::ItemSpec::ApplyOpSpec
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
