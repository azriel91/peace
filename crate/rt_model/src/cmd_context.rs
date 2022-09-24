use std::marker::PhantomData;

use peace_resources::Resources;

use crate::{CmdContextBuilder, ItemSpecGraph, StatesTypeRegs, Workspace};

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
/// * `O`: `OutputWrite` to return values / errors to.
/// * `WorkspaceInit`: Parameters to initialize the workspace.
///
///     These are parameters common to the workspace. Examples:
///
///     - Organization username.
///     - Repository URL for multiple environments.
///
///     This may be `()` if there are no parameters common to the workspace.
///
/// * `ProfileInit`: Parameters to initialize the profile.
///
///     These are parameters specific to a profile, but common to flows within
///     that profile. Examples:
///
///     - Environment specific credentials.
///     - URL to publish / download an artifact.
///
///     This may be `()` if there are no profile specific parameters.
///
/// * `FlowInit`: Parameters to initialize the flow.
///
///     These are parameters specific to a flow. Examples:
///
///     - Configuration to skip warnings for the particular flow.
///
///     This may be `()` if there are no flow specific parameters.
///
/// * `TS`: Type state of `Resources`.
///
/// [`Profile`]: peace_cfg::Profile
/// [`WorkspaceDir`]: peace::resources::paths::WorkspaceDir
/// [`PeaceDir`]: peace::resources::paths::PeaceDir
/// [`ProfileDir`]: peace::resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace::resources::paths::ProfileHistoryDir
#[derive(Debug)]
pub struct CmdContext<'ctx, E, O, TS> {
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'ctx Workspace,
    /// Graph of item specs.
    pub item_spec_graph: &'ctx ItemSpecGraph<E>,
    /// `OutputWrite` to return values / errors to.
    pub output: &'ctx mut O,
    /// `Resources` in this workspace.
    pub resources: Resources<TS>,
    /// Type registries to deserialize `StatesCurrentFile` and
    /// `StatesDesiredFile`.
    pub states_type_regs: StatesTypeRegs,
    /// Prevents instantiation not through builder.
    pub(crate) marker: PhantomData<()>,
}

impl<'ctx, E, O, TS> CmdContext<'ctx, E, O, TS>
where
    E: std::error::Error,
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
    ) -> CmdContextBuilder<'ctx, E, O, (), (), ()> {
        CmdContextBuilder::new(workspace, item_spec_graph, output)
    }

    /// Returns the underlying data.
    pub fn into_inner(
        self,
    ) -> (
        &'ctx Workspace,
        &'ctx ItemSpecGraph<E>,
        &'ctx mut O,
        Resources<TS>,
        StatesTypeRegs,
    ) {
        let Self {
            workspace,
            item_spec_graph,
            output,
            states_type_regs,
            resources,
            marker: _,
        } = self;

        (
            workspace,
            item_spec_graph,
            output,
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

    /// Returns a reference to the resources.
    pub fn resources(&self) -> &Resources<TS> {
        &self.resources
    }

    /// Returns a mutable reference to the resources.
    pub fn resources_mut(&mut self) -> &mut Resources<TS> {
        &mut self.resources
    }
}

impl<'ctx, E, O, TS>
    From<(
        &'ctx Workspace,
        &'ctx ItemSpecGraph<E>,
        &'ctx mut O,
        Resources<TS>,
        StatesTypeRegs,
    )> for CmdContext<'ctx, E, O, TS>
{
    fn from(
        (workspace, item_spec_graph, output, resources, states_type_regs): (
            &'ctx Workspace,
            &'ctx ItemSpecGraph<E>,
            &'ctx mut O,
            Resources<TS>,
            StatesTypeRegs,
        ),
    ) -> Self {
        Self {
            workspace,
            item_spec_graph,
            output,
            resources,
            states_type_regs,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, TS0, TS1, F> From<(CmdContext<'ctx, E, O, TS0>, F)> for CmdContext<'ctx, E, O, TS1>
where
    E: std::error::Error,
    F: FnOnce(Resources<TS0>) -> Resources<TS1>,
{
    fn from((cmd_context_ts0, f): (CmdContext<'ctx, E, O, TS0>, F)) -> Self {
        let (workspace, item_spec_graph, output, resources, states_type_regs) =
            cmd_context_ts0.into_inner();
        let resources: Resources<TS1> = f(resources);

        Self {
            workspace,
            item_spec_graph,
            output,
            resources,
            states_type_regs,
            marker: PhantomData,
        }
    }
}
