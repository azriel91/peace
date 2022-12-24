use std::{fmt::Debug, future::IntoFuture, hash::Hash, marker::PhantomData, pin::Pin};

use futures::{Future, StreamExt, TryStreamExt};
use peace_resources::{
    internal::{FlowInitFile, ProfileInitFile, WorkspaceInitFile},
    paths::StatesSavedFile,
    resources::ts::{Empty, SetUp},
    states::StatesSaved,
    type_reg::untagged::{BoxDt, TypeReg},
    Resources,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    cmd_context_params::{FlowParams, ProfileParams, WorkspaceParams},
    CmdContext, Error, ItemSpecGraph, StatesDeserializer, StatesTypeRegs, Storage, Workspace,
    WorkspaceInitializer,
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
/// * `WorkspaceParamsK`: `WorkspaceParams` map `K` type parameter.
/// * `ProfileParamsK`: `ProfileParams` map `K` type parameter.
/// * `FlowParamsK`: `FlowParams` map `K` type parameter.
///
/// [`Profile`]: peace_cfg::Profile
/// [`WorkspaceDir`]: peace::resources::paths::WorkspaceDir
/// [`PeaceDir`]: peace::resources::paths::PeaceDir
/// [`ProfileDir`]: peace::resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace::resources::paths::ProfileHistoryDir
#[derive(Debug)]
pub struct CmdContextBuilder<'ctx, E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    WorkspaceParamsK: Debug + Eq + Hash,
    ProfileParamsK: Debug + Eq + Hash,
    FlowParamsK: Debug + Eq + Hash,
{
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Graph of item specs.
    item_spec_graph: &'ctx ItemSpecGraph<E>,
    /// `OutputWrite` to return values / errors to.
    output: &'ctx mut O,
    /// Workspace parameters.
    workspace_params: Option<WorkspaceParams<WorkspaceParamsK>>,
    /// Type registry for `WorkspaceParams` deserialization.
    workspace_params_type_reg: TypeReg<WorkspaceParamsK, BoxDt>,
    /// Profile parameters.
    profile_params: Option<ProfileParams<ProfileParamsK>>,
    /// Type registry for `ProfileParams` deserialization.
    profile_params_type_reg: TypeReg<ProfileParamsK, BoxDt>,
    /// Flow parameters.
    flow_params: Option<FlowParams<FlowParamsK>>,
    /// Type registry for `FlowParams` deserialization.
    flow_params_type_reg: TypeReg<FlowParamsK, BoxDt>,
}

impl<'ctx, E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    CmdContextBuilder<'ctx, E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error + From<Error>,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
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
            workspace_params: None,
            workspace_params_type_reg: TypeReg::new(),
            profile_params: None,
            profile_params_type_reg: TypeReg::new(),
            flow_params: None,
            flow_params_type_reg: TypeReg::new(),
        }
    }

    /// Sets the workspace parameters.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `workspace_param`: The workspace parameter to register.
    pub fn with_workspace_param<WorkspaceParam>(
        mut self,
        k: WorkspaceParamsK,
        workspace_param: WorkspaceParam,
    ) -> Self
    where
        WorkspaceParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.workspace_params_type_reg
            .register::<WorkspaceParam>(k.clone());
        self.workspace_params
            .get_or_insert_with(WorkspaceParams::new)
            .insert(k, workspace_param);
        self
    }

    /// Sets the profile parameters.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `profile_param`: The profile parameter to register.
    pub fn with_profile_param<ProfileParam>(
        mut self,
        k: ProfileParamsK,
        profile_param: ProfileParam,
    ) -> Self
    where
        ProfileParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.profile_params_type_reg
            .register::<ProfileParam>(k.clone());
        self.profile_params
            .get_or_insert_with(ProfileParams::new)
            .insert(k, profile_param);
        self
    }

    /// Sets the flow parameters.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `flow_param`: The flow parameter to register.
    pub fn with_flow_param<FlowParam>(mut self, k: FlowParamsK, flow_param: FlowParam) -> Self
    where
        FlowParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.flow_params_type_reg.register::<FlowParam>(k.clone());
        self.flow_params
            .get_or_insert_with(FlowParams::new)
            .insert(k, flow_param);
        self
    }

    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_params`: Initialization parameters for the workspace.
    /// * `profile_params`: Initialization parameters for the profile.
    /// * `flow_params`: Initialization parameters for the flow.
    pub async fn build(mut self) -> Result<CmdContext<'ctx, E, O, SetUp>, E> {
        let dirs = self.workspace.dirs();
        let storage = self.workspace.storage();
        let workspace_init_file = WorkspaceInitFile::from(dirs.peace_dir());
        let profile_init_file = ProfileInitFile::from(dirs.profile_dir());
        let flow_init_file = FlowInitFile::from(dirs.flow_dir());
        let states_saved_file = StatesSavedFile::from(dirs.flow_dir());

        // Read existing init params from storage.
        self.init_params_deserialize(
            storage,
            &workspace_init_file,
            &profile_init_file,
            &flow_init_file,
        )
        .await?;

        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_params,
            workspace_params_type_reg: _,
            profile_params,
            profile_params_type_reg: _,
            flow_params,
            flow_params_type_reg: _,
        } = self;

        // Create directories and write init parameters to storage.
        #[cfg(target_arch = "wasm32")]
        WorkspaceInitializer::dirs_initialize(storage, dirs).await?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let workspace_dir = dirs.workspace_dir();
            std::env::set_current_dir(workspace_dir).map_err(|error| Error::CurrentDirSet {
                workspace_dir: workspace_dir.clone(),
                error,
            })?;

            WorkspaceInitializer::dirs_initialize(dirs).await?;
        }

        Self::init_params_serialize(
            storage,
            workspace_params.as_ref(),
            &workspace_init_file,
            profile_params.as_ref(),
            &profile_init_file,
            flow_params.as_ref(),
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
            workspace_params,
            profile_params,
            flow_params,
        );

        // Read existing states from storage.
        let states_type_regs = Self::states_type_regs(item_spec_graph);
        let states_saved = StatesDeserializer::deserialize_saved_opt(
            storage,
            states_type_regs.states_current_type_reg(),
            &states_saved_file,
        )
        .await?
        .map(Into::<StatesSaved>::into);
        if let Some(states_saved) = states_saved {
            resources.insert(states_saved);
        }

        // Call each `ItemSpec`'s initialization function.
        let resources = Self::item_spec_graph_setup(item_spec_graph, resources).await?;

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
    ///
    /// **TODO:** Multiple Init Params Support ([#45])
    ///
    /// Implementors may wish to take in init parameters that are relevant to
    /// different `ItemSpec`s, and so each init parameter needs to be able to be
    /// mapped to multiple different data types.
    ///
    /// [#45]: https://github.com/azriel91/peace/issues/45
    fn init_params_insert(
        resources: &mut Resources<Empty>,
        workspace_params: Option<WorkspaceParams<WorkspaceParamsK>>,
        profile_params: Option<ProfileParams<ProfileParamsK>>,
        flow_params: Option<FlowParams<FlowParamsK>>,
    ) {
        if let Some(workspace_params) = workspace_params {
            resources.insert(workspace_params);
        }
        if let Some(profile_params) = profile_params {
            resources.insert(profile_params);
        }
        if let Some(flow_params) = flow_params {
            resources.insert(flow_params);
        }
    }

    async fn init_params_deserialize(
        &mut self,
        storage: &Storage,
        workspace_init_file: &WorkspaceInitFile,
        profile_init_file: &ProfileInitFile,
        flow_init_file: &FlowInitFile,
    ) -> Result<(), E> {
        if self.workspace_params.is_none() {
            self.workspace_params = WorkspaceInitializer::workspace_params_deserialize(
                storage,
                &self.workspace_params_type_reg,
                workspace_init_file,
            )
            .await?
        };
        if self.profile_params.is_none() {
            self.profile_params = WorkspaceInitializer::profile_params_deserialize(
                storage,
                &self.profile_params_type_reg,
                profile_init_file,
            )
            .await?;
        }
        if self.flow_params.is_none() {
            self.flow_params = WorkspaceInitializer::flow_params_deserialize(
                storage,
                &self.flow_params_type_reg,
                flow_init_file,
            )
            .await?;
        }

        Ok(())
    }

    /// Serializes init params to storage.
    async fn init_params_serialize(
        storage: &Storage,
        workspace_params: Option<&WorkspaceParams<WorkspaceParamsK>>,
        workspace_init_file: &WorkspaceInitFile,
        profile_params: Option<&ProfileParams<ProfileParamsK>>,
        profile_init_file: &ProfileInitFile,
        flow_params: Option<&FlowParams<FlowParamsK>>,
        flow_init_file: &FlowInitFile,
    ) -> Result<(), E> {
        if let Some(workspace_params) = workspace_params {
            WorkspaceInitializer::workspace_params_serialize(
                storage,
                workspace_params,
                workspace_init_file,
            )
            .await?;
        }
        if let Some(profile_params) = profile_params {
            WorkspaceInitializer::profile_params_serialize(
                storage,
                profile_params,
                profile_init_file,
            )
            .await?;
        }
        if let Some(flow_params) = flow_params {
            WorkspaceInitializer::flow_params_serialize(storage, flow_params, flow_init_file)
                .await?;
        }

        Ok(())
    }
}

impl<'ctx, E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    CmdContextBuilder<'ctx, E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error,
    WorkspaceParamsK: Debug + Eq + Hash,
    ProfileParamsK: Debug + Eq + Hash,
    FlowParamsK: Debug + Eq + Hash,
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

impl<'ctx, E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK> IntoFuture
    for CmdContextBuilder<'ctx, E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error + From<Error>,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    type IntoFuture = CmdContextFuture<'ctx, E, O>;
    type Output = <Self::IntoFuture as Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.build())
    }
}
