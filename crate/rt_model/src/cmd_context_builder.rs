use std::{fmt, future::IntoFuture, marker::PhantomData, pin::Pin};

use futures::{Future, StreamExt, TryStreamExt};
use peace_resources::{
    internal::{FlowInitFile, ProfileInitFile, WorkspaceInitFile},
    resources::ts::{Empty, SetUp},
    Resources,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    CmdContext, Error, ItemSpecGraph, StatesTypeRegs, Storage, Workspace, WorkspaceInitializer,
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
/// [`Profile`]: peace_cfg::Profile
/// [`WorkspaceDir`]: peace::resources::paths::WorkspaceDir
/// [`PeaceDir`]: peace::resources::paths::PeaceDir
/// [`ProfileDir`]: peace::resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace::resources::paths::ProfileHistoryDir
#[derive(Debug)]
pub struct CmdContextBuilder<'ctx, E, O, WorkspaceInit, ProfileInit, FlowInit> {
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Graph of item specs.
    item_spec_graph: &'ctx ItemSpecGraph<E>,
    /// `OutputWrite` to return values / errors to.
    output: &'ctx mut O,
    /// Workspace initialization parameters.
    workspace_init_params: Option<WorkspaceInit>,
    /// Profile initialization parameters.
    profile_init_params: Option<ProfileInit>,
    /// Flow initialization parameters.
    flow_init_params: Option<FlowInit>,
}

impl<'ctx, E, O> CmdContextBuilder<'ctx, E, O, (), (), ()>
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
    pub fn new(
        workspace: &'ctx Workspace,
        item_spec_graph: &'ctx ItemSpecGraph<E>,
        output: &'ctx mut O,
    ) -> Self {
        Self {
            workspace,
            item_spec_graph,
            output,
            workspace_init_params: None,
            profile_init_params: None,
            flow_init_params: None,
        }
    }
}

impl<'ctx, E, O, ProfileInit, FlowInit> CmdContextBuilder<'ctx, E, O, (), ProfileInit, FlowInit>
where
    E: std::error::Error,
    ProfileInit: Clone + fmt::Debug + Send + Sync + 'static,
    FlowInit: Clone + fmt::Debug + Send + Sync + 'static,
{
    /// Sets the workspace initialization parameters.
    ///
    /// The init param is optional in case the init parameters should be loaded
    /// from storage.
    ///
    /// Type state enforces that this can only be set once.
    ///
    /// # Parameters
    ///
    /// * `workspace_init_next`: The parameters to initialize the workspace.
    pub fn with_workspace_init<WorkspaceInitNext>(
        self,
        workspace_init_next: Option<WorkspaceInitNext>,
    ) -> CmdContextBuilder<'ctx, E, O, WorkspaceInitNext, ProfileInit, FlowInit>
    where
        WorkspaceInitNext: Clone + fmt::Debug + Send + Sync + 'static,
    {
        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_init_params: _,
            profile_init_params,
            flow_init_params,
        } = self;

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_init_params: workspace_init_next,
            profile_init_params,
            flow_init_params,
        }
    }
}

impl<'ctx, E, O, WorkspaceInit, FlowInit> CmdContextBuilder<'ctx, E, O, WorkspaceInit, (), FlowInit>
where
    E: std::error::Error,
    WorkspaceInit: Clone + fmt::Debug + Send + Sync + 'static,
    FlowInit: Clone + fmt::Debug + Send + Sync + 'static,
{
    /// Sets the profile initialization parameters.
    ///
    /// The init param is optional in case the init parameters should be loaded
    /// from storage.
    ///
    /// Type state enforces that this can only be set once.
    ///
    /// # Parameters
    ///
    /// * `profile_init_next`: The parameters to initialize the profile.
    pub fn with_profile_init<ProfileInitNext>(
        self,
        profile_init_next: Option<ProfileInitNext>,
    ) -> CmdContextBuilder<'ctx, E, O, WorkspaceInit, ProfileInitNext, FlowInit>
    where
        ProfileInitNext: Clone + fmt::Debug + Send + Sync + 'static,
    {
        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_init_params,
            profile_init_params: _,
            flow_init_params,
        } = self;

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_init_params,
            profile_init_params: profile_init_next,
            flow_init_params,
        }
    }
}

impl<'ctx, E, O, WorkspaceInit, ProfileInit>
    CmdContextBuilder<'ctx, E, O, WorkspaceInit, ProfileInit, ()>
where
    E: std::error::Error,
    WorkspaceInit: Clone + fmt::Debug + Send + Sync + 'static,
    ProfileInit: Clone + fmt::Debug + Send + Sync + 'static,
{
    /// Sets the flow initialization parameters.
    ///
    /// The init param is optional in case the init parameters should be loaded
    /// from storage.
    ///
    /// Type state enforces that this can only be set once.
    ///
    /// # Parameters
    ///
    /// * `flow_init_next`: The parameters to initialize the flow.
    pub fn with_flow_init<FlowInitNext>(
        self,
        flow_init_next: Option<FlowInitNext>,
    ) -> CmdContextBuilder<'ctx, E, O, WorkspaceInit, ProfileInit, FlowInitNext>
    where
        FlowInitNext: Clone + fmt::Debug + Send + Sync + 'static,
    {
        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_init_params,
            profile_init_params,
            flow_init_params: _,
        } = self;

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_init_params,
            profile_init_params,
            flow_init_params: flow_init_next,
        }
    }
}

impl<'ctx, E, O, WorkspaceInit, ProfileInit, FlowInit>
    CmdContextBuilder<'ctx, E, O, WorkspaceInit, ProfileInit, FlowInit>
where
    E: std::error::Error + From<Error>,
    WorkspaceInit: Clone + fmt::Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileInit: Clone + fmt::Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowInit: Clone + fmt::Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_init_params`: Initialization parameters for the workspace.
    /// * `profile_init_params`: Initialization parameters for the profile.
    /// * `flow_init_params`: Initialization parameters for the flow.
    pub async fn build(self) -> Result<CmdContext<'ctx, E, O, SetUp>, E> {
        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            mut workspace_init_params,
            mut profile_init_params,
            mut flow_init_params,
        } = self;

        let dirs = workspace.dirs();
        let storage = workspace.storage();
        let workspace_init_file = WorkspaceInitFile::from(dirs.peace_dir());
        let profile_init_file = ProfileInitFile::from(dirs.profile_dir());
        let flow_init_file = FlowInitFile::from(dirs.flow_dir());

        // Read existing init params from storage.
        Self::init_params_deserialize(
            storage,
            &mut workspace_init_params,
            &workspace_init_file,
            &mut profile_init_params,
            &profile_init_file,
            &mut flow_init_params,
            &flow_init_file,
        )
        .await?;

        // Create directories and write init parameters to storage.
        #[cfg(target_arch = "wasm32")]
        WorkspaceInitializer::<WorkspaceInit, ProfileInit, FlowInit>::dirs_initialize(
            storage, dirs,
        )
        .await?;
        #[cfg(not(target_arch = "wasm32"))]
        WorkspaceInitializer::<WorkspaceInit, ProfileInit, FlowInit>::dirs_initialize(dirs).await?;
        Self::init_params_serialize(
            storage,
            workspace_init_params.as_ref(),
            &workspace_init_file,
            profile_init_params.as_ref(),
            &profile_init_file,
            flow_init_params.as_ref(),
            &flow_init_file,
        )
        .await?;

        // Track items in memory.
        let mut resources = Resources::new();
        Self::workspace_dirs_insert(&mut resources, workspace);
        resources.insert(workspace_init_file);
        resources.insert(profile_init_file);
        resources.insert(flow_init_file);
        Self::init_params_insert(
            &mut resources,
            workspace_init_params,
            profile_init_params,
            flow_init_params,
        );

        // Call each `ItemSpec`'s initialization function.
        let resources = Self::item_spec_graph_setup(item_spec_graph, resources).await?;
        let states_type_regs = Self::states_type_regs(item_spec_graph);

        Ok(CmdContext {
            workspace,
            item_spec_graph,
            output,
            resources,
            states_type_regs,
            marker: PhantomData,
        })
    }

    /// Inserts workspace directory resources into the `Resources` map.
    fn workspace_dirs_insert(resources: &mut Resources<Empty>, workspace: &Workspace) {
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

    /// Inserts init params into the `Resources` map.
    fn init_params_insert(
        resources: &mut Resources<Empty>,
        workspace_init_params: Option<WorkspaceInit>,
        profile_init_params: Option<ProfileInit>,
        flow_init_params: Option<FlowInit>,
    ) {
        if let Some(workspace_init_params) = workspace_init_params {
            resources.insert(workspace_init_params);
        }
        if let Some(profile_init_params) = profile_init_params {
            resources.insert(profile_init_params);
        }
        if let Some(flow_init_params) = flow_init_params {
            resources.insert(flow_init_params);
        }
    }

    async fn init_params_deserialize(
        storage: &Storage,
        workspace_init_params: &mut Option<WorkspaceInit>,
        workspace_init_file: &WorkspaceInitFile,
        profile_init_params: &mut Option<ProfileInit>,
        profile_init_file: &ProfileInitFile,
        flow_init_params: &mut Option<FlowInit>,
        flow_init_file: &FlowInitFile,
    ) -> Result<(), E> {
        if workspace_init_params.is_none() {
            *workspace_init_params =
            WorkspaceInitializer::<WorkspaceInit, ProfileInit, FlowInit>::workspace_init_params_deserialize(storage, workspace_init_file)
                .await?
        };
        if profile_init_params.is_none() {
            *profile_init_params =
            WorkspaceInitializer::<WorkspaceInit, ProfileInit, FlowInit>::profile_init_params_deserialize(storage, profile_init_file).await?;
        }
        if flow_init_params.is_none() {
            *flow_init_params =
            WorkspaceInitializer::<WorkspaceInit, ProfileInit, FlowInit>::flow_init_params_deserialize(storage, flow_init_file).await?;
        }

        Ok(())
    }

    /// Serializes init params to storage.
    async fn init_params_serialize(
        storage: &Storage,
        workspace_init_params: Option<&WorkspaceInit>,
        workspace_init_file: &WorkspaceInitFile,
        profile_init_params: Option<&ProfileInit>,
        profile_init_file: &ProfileInitFile,
        flow_init_params: Option<&FlowInit>,
        flow_init_file: &FlowInitFile,
    ) -> Result<(), E> {
        if let Some(workspace_init_params) = workspace_init_params {
            WorkspaceInitializer::<WorkspaceInit, ProfileInit, FlowInit>::workspace_init_params_serialize(
                storage,
                workspace_init_params,
                workspace_init_file,
            )
            .await?;
        }
        if let Some(profile_init_params) = profile_init_params {
            WorkspaceInitializer::<WorkspaceInit, ProfileInit, FlowInit>::profile_init_params_serialize(
                storage,
                profile_init_params,
                profile_init_file,
            )
            .await?;
        }
        if let Some(flow_init_params) = flow_init_params {
            WorkspaceInitializer::<WorkspaceInit, ProfileInit, FlowInit>::flow_init_params_serialize(
                storage,
                flow_init_params,
                flow_init_file,
            )
            .await?;
        }

        Ok(())
    }
}

impl<'ctx, E, O, WorkspaceInit, ProfileInit, FlowInit>
    CmdContextBuilder<'ctx, E, O, WorkspaceInit, ProfileInit, FlowInit>
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
}

/// Future that returns the `CmdContext`.
///
/// This is boxed since [TAIT] is not yet available.
///
/// [TAIT]: https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html
pub type CmdContextFuture<'ctx, E, O> =
    Pin<Box<dyn Future<Output = Result<CmdContext<'ctx, E, O, SetUp>, E>> + 'ctx>>;

impl<'ctx, E, O, WorkspaceInit, ProfileInit, FlowInit> IntoFuture
    for CmdContextBuilder<'ctx, E, O, WorkspaceInit, ProfileInit, FlowInit>
where
    E: std::error::Error + From<Error>,
    WorkspaceInit: Clone + fmt::Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileInit: Clone + fmt::Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowInit: Clone + fmt::Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    type IntoFuture = CmdContextFuture<'ctx, E, O>;
    type Output = <Self::IntoFuture as Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.build())
    }
}
