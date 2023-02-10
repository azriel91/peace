use std::{fmt::Debug, marker::PhantomData};

use peace_resources::{resources::ts::SetUp, Resources};
use peace_rt_model_core::cmd_context_params::{
    KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs,
};

use crate::{
    cmd::{ts::CmdContextCommon, CmdContextBuilder},
    ItemSpecGraph, StatesTypeRegs, Workspace,
};

/// Information needed to execute a command.
///
/// This includes:
///
/// * `ItemSpecGraph`: Logic to run.
/// * `Resources`: Data consumed / produced by the command.
///
/// Members of `Workspace` -- where the command should be run -- are
/// individually stored in `Resources`:
///
/// * [`FlowDir`]
/// * [`PeaceDir`]
/// * [`Profile`]
/// * [`ProfileDir`]
/// * [`ProfileHistoryDir`]
/// * [`WorkspaceDir`]
///
/// # Type Parameters
///
/// * `E`: Consumer provided error type.
/// * `O`: [`OutputWrite`] to return values / errors to.
/// * `WorkspaceParamsK`: [`WorkspaceParams`] map `K` type parameter.
/// * `ProfileParamsK`: [`ProfileParams`] map `K` type parameter.
/// * `FlowParamsK`: [`FlowParams`] map `K` type parameter.
///
/// # Design
///
/// * [`WorkspaceParams`], [`ProfileParams`], and [`FlowParams`]' types must be
///   specified, if they are to be deserialized.
///
/// * Notably, [`ProfileParams`] and [`FlowParams`] *may* be different for
///   different profiles and flows.
///
///     If they are different, then it makes it impossible to deserialize them
///     for a given `CmdContext`. We could constrain the params types to be a
///     superset of all profile/flow params, which essentially is making them
///     the same umbrella type.
///
///     This should be feasible for [`ProfileParams`], as profiles are intended
///     to be logically separate copies of the same managed items. Production
///     profiles may require more parameters, but the parameter type can be the
///     same.
///
///     However, [`FlowParams`] being different per flow is a fair assumption.
///     This means cross profile inspections of the same flow is achievable --
///     the same [`FlowParams`] type and [`ItemSpecGraph`] can prepare the
///     [`TypeReg`]istries to deserialize the [`FlowParamsFile`],
///     [`StatesSavedFile`], and [`StatesDesiredFile`].
///
/// * A [`Profile`] is needed when there are [`ProfileParams`] to store, as it
///   is used to calculate the [`ProfileDir`] to store the params.
///
/// * A [`FlowId`] is needed when there are [`FlowParams`] to store, or an
///   [`ItemSpecGraph`] to execute, as it is used calculate the [`FlowDir`] to
///   store the params, or read or write [`States`].
///
/// * You should be able to list profiles, read profile params, and list flows,
///   without needing to have either a profile or a flow.
///
/// * For [`States`] from all flows to be deserializable, there must be a type
///   registry with *all* item specs' `State` registered. This is a maintenance
///   cost for implementors, but unavoidable if that kind of functionality is
///   desired.
///
/// [`FlowDir`]: peace_resources::paths::FlowDir
/// [`FlowParams`]: crate::cmd_context_params::FlowParams
/// [`OutputWrite`]: peace_rt_model_core::OutputWrite
/// [`PeaceDir`]: peace_resources::paths::PeaceDir
/// [`Profile`]: peace_cfg::Profile
/// [`ProfileDir`]: peace_resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace_resources::paths::ProfileHistoryDir
/// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
/// [`States`]: peace_resources::States
/// [`WorkspaceDir`]: peace_resources::paths::WorkspaceDir
/// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
#[derive(Debug)]
pub struct CmdContext<'ctx, E, O, TS, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'ctx Workspace,
    /// Graph of item specs.
    pub item_spec_graph: &'ctx ItemSpecGraph<E>,
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: &'ctx mut O,
    /// `Resources` in this workspace.
    pub resources: Resources<TS>,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    pub params_type_regs: ParamsTypeRegs<PKeys>,
    /// Type registries to deserialize `StatesSavedFile` and
    /// `StatesDesiredFile`.
    pub states_type_regs: StatesTypeRegs,
    /// Multi-progress to track progress of each operation execution.
    #[cfg(feature = "output_progress")]
    pub cmd_progress_tracker: crate::CmdProgressTracker,
    /// Prevents instantiation not through builder.
    pub(crate) marker: PhantomData<()>,
}

impl<'ctx, E, O> CmdContext<'ctx, E, O, SetUp, ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>
where
    E: std::error::Error + From<crate::Error>,
{
    /// Returns a builder for the command context.
    ///
    /// # Parameters
    ///
    /// * `workspace`: Defines how to discover the workspace.
    /// * `item_spec_graph`: Logic to run in the command.
    /// * `output`: [`OutputWrite`] to return values or errors.
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub fn builder(
        workspace: &'ctx Workspace,
        item_spec_graph: &'ctx ItemSpecGraph<E>,
        output: &'ctx mut O,
    ) -> CmdContextBuilder<
        'ctx,
        E,
        O,
        CmdContextCommon,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdContextBuilder::new(workspace, item_spec_graph, output)
    }
}

#[cfg(not(feature = "output_progress"))]
type CmdContextFields<'ctx, E, O, TS, PKeys> = (
    &'ctx Workspace,
    &'ctx ItemSpecGraph<E>,
    &'ctx mut O,
    Resources<TS>,
    ParamsTypeRegs<PKeys>,
    StatesTypeRegs,
);

#[cfg(feature = "output_progress")]
type CmdContextFields<'ctx, E, O, TS, PKeys> = (
    &'ctx Workspace,
    &'ctx ItemSpecGraph<E>,
    &'ctx mut O,
    Resources<TS>,
    ParamsTypeRegs<PKeys>,
    StatesTypeRegs,
    crate::CmdProgressTracker,
);

impl<'ctx, E, O, TS, PKeys> CmdContext<'ctx, E, O, TS, PKeys>
where
    E: std::error::Error,
    PKeys: ParamsKeys + 'static,
{
    /// Returns the underlying data.
    pub fn into_inner(self) -> CmdContextFields<'ctx, E, O, TS, PKeys> {
        let Self {
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            marker: _,
        } = self;

        (
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
        )
    }

    /// Returns a reference to the workspace.
    pub fn workspace(&self) -> &Workspace {
        self.workspace
    }

    /// Returns a reference to the item spec graph.
    pub fn item_spec_graph(&self) -> &ItemSpecGraph<E> {
        self.item_spec_graph
    }

    /// Returns a reference to the output.
    pub fn output(&self) -> &O {
        &*self.output
    }

    /// Returns a mutable reference to the output.
    pub fn output_mut(&mut self) -> &mut O {
        self.output
    }

    /// Returns a reference to the resources.
    pub fn resources(&self) -> &Resources<TS> {
        &self.resources
    }

    /// Returns a mutable reference to the resources.
    pub fn resources_mut(&mut self) -> &mut Resources<TS> {
        &mut self.resources
    }

    /// Returns a reference to the params type registries
    pub fn params_type_regs(&self) -> &ParamsTypeRegs<PKeys> {
        &self.params_type_regs
    }

    /// Returns a reference to the states type registries
    pub fn states_type_regs(&self) -> &StatesTypeRegs {
        &self.states_type_regs
    }
}

impl<'ctx, E, O, TS, PKeys> From<CmdContextFields<'ctx, E, O, TS, PKeys>>
    for CmdContext<'ctx, E, O, TS, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    fn from(
        #[cfg(not(feature = "output_progress"))] (
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
        ): CmdContextFields<'ctx, E, O, TS, PKeys>,
        #[cfg(feature = "output_progress")] (
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            cmd_progress_tracker,
        ): CmdContextFields<'ctx, E, O, TS, PKeys>,
    ) -> Self {
        Self {
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, TS0, TS1, PKeys, F> From<(CmdContext<'ctx, E, O, TS0, PKeys>, F)>
    for CmdContext<'ctx, E, O, TS1, PKeys>
where
    E: std::error::Error,
    PKeys: ParamsKeys + 'static,
    F: FnOnce(Resources<TS0>) -> Resources<TS1>,
{
    fn from((cmd_context_ts0, f): (CmdContext<'ctx, E, O, TS0, PKeys>, F)) -> Self {
        #[cfg(not(feature = "output_progress"))]
        let (workspace, item_spec_graph, output, resources, params_type_regs, states_type_regs) =
            cmd_context_ts0.into_inner();
        #[cfg(feature = "output_progress")]
        let (
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            cmd_progress_tracker,
        ) = cmd_context_ts0.into_inner();
        let resources: Resources<TS1> = f(resources);

        Self {
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            marker: PhantomData,
        }
    }
}
