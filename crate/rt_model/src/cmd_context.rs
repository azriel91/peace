use futures::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{Empty, SetUp},
    Resources,
};

use crate::{Error, ItemSpecGraph, Workspace};

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
/// * `TS`: Type state of `Resources`.
/// * `E`: Consumer provided error type.
///
/// [`Profile`]: peace_cfg::Profile
/// [`WorkspaceDir`]: peace::resources::paths::WorkspaceDir
/// [`PeaceDir`]: peace::resources::paths::PeaceDir
/// [`ProfileDir`]: peace::resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace::resources::paths::ProfileHistoryDir
#[derive(Debug)]
pub struct CmdContext<'ctx, TS, E> {
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'ctx Workspace,
    /// Graph of item specs.
    pub item_spec_graph: &'ctx ItemSpecGraph<E>,
    /// `Resources` in this workspace.
    pub resources: Resources<TS>,
}

impl<'ctx, E> CmdContext<'ctx, SetUp, E>
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
    ) -> Result<CmdContext<'ctx, SetUp, E>, E> {
        let mut resources = Resources::new();

        Self::insert_workspace_resources(workspace, &mut resources);
        let resources = Self::item_spec_graph_setup(item_spec_graph, resources).await?;

        Ok(CmdContext {
            workspace,
            item_spec_graph,
            resources,
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

impl<'ctx, TS, E> CmdContext<'ctx, TS, E>
where
    E: std::error::Error,
{
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

    /// Returns the underlying workspace, item spec graph, and resources.
    pub fn into_inner(self) -> (&'ctx Workspace, &'ctx ItemSpecGraph<E>, Resources<TS>) {
        let Self {
            workspace,
            item_spec_graph,
            resources,
        } = self;

        (workspace, item_spec_graph, resources)
    }

    /// Returns a reference to the workspace.
    pub fn workspace(&self) -> &Workspace {
        self.workspace
    }

    /// Returns a reference to the resources.
    pub fn resources(&self) -> &Resources<TS> {
        &self.resources
    }

    /// Returns a mutable reference to the resources.
    pub fn resources_mut(&mut self) -> &mut Resources<TS> {
        &mut self.resources
    }

    /// Returns a reference to the item spec graph.
    pub fn item_spec_graph(&self) -> &ItemSpecGraph<E> {
        self.item_spec_graph
    }
}

impl<'ctx, TS, E> From<(&'ctx Workspace, &'ctx ItemSpecGraph<E>, Resources<TS>)>
    for CmdContext<'ctx, TS, E>
{
    fn from(
        (workspace, item_spec_graph, resources): (
            &'ctx Workspace,
            &'ctx ItemSpecGraph<E>,
            Resources<TS>,
        ),
    ) -> Self {
        Self {
            workspace,
            item_spec_graph,
            resources,
        }
    }
}
