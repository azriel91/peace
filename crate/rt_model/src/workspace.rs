use futures::{StreamExt, TryStreamExt};
use peace_cfg::Profile;
use peace_resources::{
    resources_type_state::{Empty, SetUp},
    Resources,
};

use crate::{Error, ItemSpecGraph, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the `peace` tool runs in.
///
/// # Type Parameters
///
/// * `TS`: Type state of `Resources`.
/// * `E`: Consumer provided error type.
#[derive(Debug)]
pub struct Workspace<TS, E>
where
    E: std::error::Error,
{
    /// `Resources` in this workspace.
    resources: Resources<TS>,
    /// Graph of item specs.
    item_spec_graph: ItemSpecGraph<E>,
}

impl<TS, E> Workspace<TS, E>
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

    /// Returns the underlying resources and item spec graph.
    pub fn into_inner(self) -> (Resources<TS>, ItemSpecGraph<E>) {
        let Self {
            resources,
            item_spec_graph,
        } = self;

        (resources, item_spec_graph)
    }

    /// Returns a reference to the workspace's resources.
    pub fn resources(&self) -> &Resources<TS> {
        &self.resources
    }

    /// Returns a mutable reference to the workspace's resources.
    pub fn resources_mut(&mut self) -> &mut Resources<TS> {
        &mut self.resources
    }

    /// Returns a reference to the workspace's item spec graph.
    pub fn item_spec_graph(&self) -> &ItemSpecGraph<E> {
        &self.item_spec_graph
    }
}

impl<E> Workspace<SetUp, E>
where
    E: std::error::Error + From<Error>,
{
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    pub async fn init(
        workspace_spec: &WorkspaceSpec,
        profile: Profile,
        item_spec_graph: ItemSpecGraph<E>,
    ) -> Result<Workspace<SetUp, E>, E> {
        let workspace_dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile)?;

        // TODO: ensure directories exist

        let (workspace_dir, peace_dir, profile_dir, profile_history_dir) =
            workspace_dirs.into_inner();
        let mut resources = Resources::new();
        resources.insert(workspace_dir);
        resources.insert(peace_dir);
        resources.insert(profile_dir);
        resources.insert(profile_history_dir);

        let resources = Self::item_spec_graph_setup(&item_spec_graph, resources).await?;

        Ok(Workspace {
            resources,
            item_spec_graph,
        })
    }
}

impl<TS, E> From<(Resources<TS>, ItemSpecGraph<E>)> for Workspace<TS, E>
where
    E: std::error::Error,
{
    fn from((resources, item_spec_graph): (Resources<TS>, ItemSpecGraph<E>)) -> Self {
        Self {
            resources,
            item_spec_graph,
        }
    }
}
