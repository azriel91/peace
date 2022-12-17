use std::marker::PhantomData;

use futures::{
    stream::{StreamExt, TryStreamExt},
    TryStream,
};
use peace_cfg::OpCheckStatus;
use peace_resources::{
    internal::OpCheckStatuses,
    resources::ts::{Ensured, EnsuredDry, SetUp, WithStatesCurrentDiffs},
    states::{StatesEnsured, StatesEnsuredDry},
    Resources,
};
use peace_rt_model::{
    CmdContext, Error, FnRef, ItemSpecBoxed, ItemSpecGraph, OutputWrite, StatesTypeRegs,
};

use crate::cmds::{sub::StatesCurrentDiscoverCmd, DiffCmd};

#[derive(Debug)]
pub struct EnsureCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> EnsureCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
    O: OutputWrite<E>,
{
    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`EnsureOpSpec::check`], and only runs
    /// [`exec_dry`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`EnsureOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`EnsureOpSpec::exec_dry`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec_dry` as it may use
    /// different `Data`.
    ///
    /// [`exec_dry`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub async fn exec_dry(
        cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, EnsuredDry>, E> {
        let (workspace, item_spec_graph, output, resources, states_type_regs) =
            cmd_context.into_inner();
        let resources_result =
            Self::exec_dry_internal(item_spec_graph, resources, &states_type_regs).await;

        match resources_result {
            Ok(resources) => {
                {
                    let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
                    output.write_states_ensured_dry(&states_ensured_dry).await?;
                }
                let cmd_context = CmdContext::from((
                    workspace,
                    item_spec_graph,
                    output,
                    resources,
                    states_type_regs,
                ));
                Ok(cmd_context)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// Same as [`Self::exec_dry`], but does not change the type state, and
    /// returns [`StatesEnsured`].
    ///
    /// [`exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub(crate) async fn exec_dry_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: Resources<SetUp>,
        states_type_regs: &StatesTypeRegs,
    ) -> Result<Resources<EnsuredDry>, E> {
        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let resources = DiffCmd::<E, O>::exec_internal_with_states_current(
            item_spec_graph,
            resources,
            &states_type_regs,
        )
        .await?;
        let op_check_statuses = Self::ensure_op_spec_check(item_spec_graph, &resources).await?;
        Self::ensure_op_spec_exec_dry(item_spec_graph, &resources, &op_check_statuses).await?;

        // TODO: This fetches the real state, whereas for a dry run, it would be useful
        // to show the imagined altered state.
        let states_current = StatesCurrentDiscoverCmd::<E, O>::exec_internal_for_ensure_dry(
            item_spec_graph,
            &resources,
        )
        .await?;

        let states_ensured_dry = StatesEnsuredDry::from((states_current, &resources));
        let resources = Resources::<EnsuredDry>::from((resources, states_ensured_dry));

        Ok(resources)
    }

    async fn ensure_op_spec_exec_dry(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStatesCurrentDiffs>,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::ensure_op_spec_stream(item_spec_graph, op_check_statuses)
            .try_for_each(|item_spec| async move { item_spec.ensure_op_exec_dry(resources).await })
            .await?;
        Ok(())
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesEnsured`].
    ///
    /// In practice this runs [`EnsureOpSpec::check`], and only runs [`exec`] if
    /// execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`EnsureOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`EnsureOpSpec::exec`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use different
    /// `Data`.
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, Ensured>, E> {
        let (workspace, item_spec_graph, output, resources, states_type_regs) =
            cmd_context.into_inner();
        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let resources_result =
            Self::exec_internal(item_spec_graph, resources, &states_type_regs).await;

        match resources_result {
            Ok(resources) => {
                {
                    let states_ensured = resources.borrow::<StatesEnsured>();
                    output.write_states_ensured(&states_ensured).await?;
                }
                let cmd_context = CmdContext::from((
                    workspace,
                    item_spec_graph,
                    output,
                    resources,
                    states_type_regs,
                ));
                Ok(cmd_context)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesEnsured`].
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub(crate) async fn exec_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: Resources<SetUp>,
        states_type_regs: &StatesTypeRegs,
    ) -> Result<Resources<Ensured>, E> {
        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let mut resources = DiffCmd::<E, O>::exec_internal_with_states_current(
            item_spec_graph,
            resources,
            &states_type_regs,
        )
        .await?;
        let op_check_statuses = Self::ensure_op_spec_check(item_spec_graph, &resources).await?;
        Self::ensure_op_spec_exec(item_spec_graph, &resources, &op_check_statuses).await?;

        let states_current = StatesCurrentDiscoverCmd::<E, O>::exec_internal_for_ensure(
            item_spec_graph,
            &mut resources,
        )
        .await?;

        let states_ensured = StatesEnsured::from((states_current, &resources));
        let resources = Resources::<Ensured>::from((resources, states_ensured));

        Ok(resources)
    }

    async fn ensure_op_spec_check(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStatesCurrentDiffs>,
    ) -> Result<OpCheckStatuses, E> {
        let op_check_statuses = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .and_then(|item_spec| async move {
                let op_check_status = item_spec.ensure_op_check(resources).await?;
                Ok((item_spec.id(), op_check_status))
            })
            .try_collect::<OpCheckStatuses>()
            .await?;

        Ok(op_check_statuses)
    }

    async fn ensure_op_spec_exec(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStatesCurrentDiffs>,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::ensure_op_spec_stream(item_spec_graph, op_check_statuses)
            .try_for_each(|item_spec| async move { item_spec.ensure_op_exec(resources).await })
            .await?;
        Ok(())
    }

    fn ensure_op_spec_stream<'f>(
        item_spec_graph: &'f ItemSpecGraph<E>,
        op_check_statuses: &'f OpCheckStatuses,
    ) -> impl TryStream<Ok = FnRef<'f, ItemSpecBoxed<E>>, Error = E> {
        item_spec_graph
            .stream()
            .filter(|item_spec| {
                let exec_required = op_check_statuses
                    .get(&item_spec.id())
                    .map(|op_check_status| {
                        matches!(op_check_status, OpCheckStatus::ExecRequired { .. })
                    })
                    .unwrap_or(true); // Should be unreachable, but we just execute if we get to this state.

                async move { exec_required }
            })
            .map(Result::Ok)
    }
}
