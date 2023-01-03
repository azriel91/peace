use std::marker::PhantomData;

use peace_resources::{resources::ts::SetUp, Resources};

use crate::{
    cmd_context_builder::KeyUnknown, CmdContextBuilder, ItemSpecGraph, StatesTypeRegs, Workspace,
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
/// * [`Profile`]
/// * [`WorkspaceDir`]
/// * [`PeaceDir`]
/// * [`ProfileDir`]
/// * [`ProfileHistoryDir`]
///
/// # Type Parameters
///
/// * `E`: Consumer provided error type.
/// * `O`: [`OutputWrite`] to return values / errors to.
/// * `PO`: [`ProgressOutputWrite`] to write progress information to.
/// * `TS`: Type state of `Resources`.
///
/// [`Profile`]: peace_cfg::Profile
/// [`WorkspaceDir`]: peace_resources::paths::WorkspaceDir
/// [`PeaceDir`]: peace_resources::paths::PeaceDir
/// [`ProfileDir`]: peace_resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace_resources::paths::ProfileHistoryDir
/// [`OutputWrite`]: peace_rt_model_core::OutputWrite
/// [`ProgressOutputWrite`]: peace_rt_model_core::ProgressOutputWrite
#[derive(Debug)]
pub struct CmdContext<'ctx, E, O, PO, TS> {
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'ctx Workspace,
    /// Graph of item specs.
    pub item_spec_graph: &'ctx ItemSpecGraph<E>,
    /// [`OutputWrite`] to return values / errors to.
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: &'ctx mut O,
    /// [`ProgressOutputWrite`] to write progress information to.
    ///
    /// [`ProgressOutputWrite`]: peace_rt_model_core::ProgressOutputWrite
    pub progress_output: &'ctx mut PO,
    /// `Resources` in this workspace.
    pub resources: Resources<TS>,
    /// Type registries to deserialize `StatesSavedFile` and
    /// `StatesDesiredFile`.
    pub states_type_regs: StatesTypeRegs,
    /// Prevents instantiation not through builder.
    pub(crate) marker: PhantomData<()>,
}

impl<'ctx, E, O, PO> CmdContext<'ctx, E, O, PO, SetUp>
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
    /// * `progress_output`: [`ProgressOutputWrite`] to write progress
    ///   information to.
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    /// [`ProgressOutputWrite`]: peace_rt_model_core::ProgressOutputWrite
    pub fn builder(
        workspace: &'ctx Workspace,
        item_spec_graph: &'ctx ItemSpecGraph<E>,
        output: &'ctx mut O,
        progress_output: &'ctx mut PO,
    ) -> CmdContextBuilder<'ctx, E, O, PO, KeyUnknown, KeyUnknown, KeyUnknown> {
        CmdContextBuilder::new(workspace, item_spec_graph, output, progress_output)
    }
}

impl<'ctx, E, O, PO, TS> CmdContext<'ctx, E, O, PO, TS>
where
    E: std::error::Error,
{
    /// Returns the underlying data.
    pub fn into_inner(
        self,
    ) -> (
        &'ctx Workspace,
        &'ctx ItemSpecGraph<E>,
        &'ctx mut O,
        &'ctx mut PO,
        Resources<TS>,
        StatesTypeRegs,
    ) {
        let Self {
            workspace,
            item_spec_graph,
            output,
            progress_output,
            states_type_regs,
            resources,
            marker: _,
        } = self;

        (
            workspace,
            item_spec_graph,
            output,
            progress_output,
            resources,
            states_type_regs,
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

    /// Returns a reference to the progress output.
    pub fn progress_output(&self) -> &PO {
        &*self.progress_output
    }

    /// Returns a mutable reference to the progress output.
    pub fn progress_output_mut(&mut self) -> &mut PO {
        self.progress_output
    }

    /// Returns a reference to the resources.
    pub fn resources(&self) -> &Resources<TS> {
        &self.resources
    }

    /// Returns a mutable reference to the resources.
    pub fn resources_mut(&mut self) -> &mut Resources<TS> {
        &mut self.resources
    }
}

impl<'ctx, E, O, PO, TS>
    From<(
        &'ctx Workspace,
        &'ctx ItemSpecGraph<E>,
        &'ctx mut O,
        &'ctx mut PO,
        Resources<TS>,
        StatesTypeRegs,
    )> for CmdContext<'ctx, E, O, PO, TS>
{
    fn from(
        (workspace, item_spec_graph, output, progress_output, resources, states_type_regs): (
            &'ctx Workspace,
            &'ctx ItemSpecGraph<E>,
            &'ctx mut O,
            &'ctx mut PO,
            Resources<TS>,
            StatesTypeRegs,
        ),
    ) -> Self {
        Self {
            workspace,
            item_spec_graph,
            output,
            progress_output,
            resources,
            states_type_regs,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, PO, TS0, TS1, F> From<(CmdContext<'ctx, E, O, PO, TS0>, F)>
    for CmdContext<'ctx, E, O, PO, TS1>
where
    E: std::error::Error,
    F: FnOnce(Resources<TS0>) -> Resources<TS1>,
{
    fn from((cmd_context_ts0, f): (CmdContext<'ctx, E, O, PO, TS0>, F)) -> Self {
        let (workspace, item_spec_graph, output, progress_output, resources, states_type_regs) =
            cmd_context_ts0.into_inner();
        let resources: Resources<TS1> = f(resources);

        Self {
            workspace,
            item_spec_graph,
            output,
            progress_output,
            resources,
            states_type_regs,
            marker: PhantomData,
        }
    }
}
