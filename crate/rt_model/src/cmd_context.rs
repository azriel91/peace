use futures::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{Empty, SetUp},
    Resources,
};

use crate::{Error, ItemSpecGraph, StatesTypeRegs, Workspace};

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
}

impl<'ctx, E, O> CmdContext<'ctx, E, O, SetUp>
where
    E: std::error::Error + From<Error>,
{
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace`: Defines how to discover the workspace.
    /// * `item_spec_graph`: Logic to run in the command.
    pub async fn init(
        workspace: &'ctx Workspace,
        item_spec_graph: &'ctx ItemSpecGraph<E>,
        output: &'ctx mut O,
    ) -> Result<CmdContext<'ctx, E, O, SetUp>, E> {
        let mut resources = Resources::new();

        Self::insert_workspace_resources(workspace, &mut resources);
        let resources = Self::item_spec_graph_setup(item_spec_graph, resources).await?;
        let states_type_regs = Self::states_type_regs(item_spec_graph);

        Ok(CmdContext {
            workspace,
            item_spec_graph,
            output,
            resources,
            states_type_regs,
        })
    }

    /// Inserts workspace directory resources into the `Resources` map.
    fn insert_workspace_resources(workspace: &Workspace, resources: &mut Resources<Empty>) {
        let (workspace_dirs, profile, flow_id, storage) = workspace.clone().into_inner();
        let (workspace_dir, peace_dir, profile_dir, profile_history_dir, flow_dir) =
            workspace_dirs.into_inner();

        resources.insert(workspace_dir);
        resources.insert(peace_dir);
        resources.insert(profile_dir);
        resources.insert(profile_history_dir);
        resources.insert(flow_dir);

        resources.insert(profile);
        resources.insert(flow_id);
        resources.insert(storage);
    }
}

impl<'ctx, E, O, TS> CmdContext<'ctx, E, O, TS>
where
    E: std::error::Error,
{
    /// Registers each item spec's `State` and `StateLogical` for
    /// deserialization.
    fn states_type_regs(item_spec_graph: &ItemSpecGraph<E>) -> StatesTypeRegs {
        item_spec_graph
            .iter()
            .fold(StatesTypeRegs::new(), |mut states_type_regs, item_spec| {
                item_spec.state_register(&mut states_type_regs);

                states_type_regs
            })
    }

    async fn item_spec_graph_setup(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: Resources<Empty>,
    ) -> Result<Resources<SetUp>, E> {
        let resources = item_spec_graph
            .stream()
            .map(Ok::<_, E>)
            .try_fold(resources, |mut resources, item_spec| async move {
                item_spec.setup(&mut resources).await?;
                Ok(resources)
            })
            .await?;

        Ok(Resources::<SetUp>::from(resources))
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
        }
    }
}
