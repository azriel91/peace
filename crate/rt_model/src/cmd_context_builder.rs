use std::{fmt::Debug, future::IntoFuture, hash::Hash, marker::PhantomData, pin::Pin};

use fn_graph::resman::Resource;
use futures::{Future, StreamExt, TryStreamExt};
use peace_resources::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    paths::StatesSavedFile,
    resources::ts::{Empty, SetUp},
    states::StatesSaved,
    type_reg::untagged::{BoxDt, TypeReg},
    Resources,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use std::collections::HashMap;

        use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget};
        use peace_cfg::progress::ProgressTracker;
        use tokio::sync::mpsc;

        use crate::CmdProgressTracker;
    }
}

use crate::{
    cmd_context_params::{FlowParams, ProfileParams, WorkspaceParams},
    CmdContext, Error, ItemSpecGraph, StatesSerializer, StatesTypeRegs, Storage, Workspace,
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
/// * `O`: [`OutputWrite`] to return values / errors to.
/// * `WorkspaceParamsK`: `WorkspaceParams` map `K` type parameter.
/// * `ProfileParamsK`: `ProfileParams` map `K` type parameter.
/// * `FlowParamsK`: `FlowParams` map `K` type parameter.
///
/// [`Profile`]: peace_cfg::Profile
/// [`WorkspaceDir`]: peace_resources::paths::WorkspaceDir
/// [`PeaceDir`]: peace_resources::paths::PeaceDir
/// [`ProfileDir`]: peace_resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace_resources::paths::ProfileHistoryDir
/// [`OutputWrite`]: peace_rt_model_core::OutputWrite
#[derive(Debug)]
pub struct CmdContextBuilder<
    'ctx,
    E,
    O,
    WorkspaceParamsKMaybe = KeyUnknown,
    ProfileParamsKMaybe = KeyUnknown,
    FlowParamsKMaybe = KeyUnknown,
> where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Graph of item specs.
    item_spec_graph: &'ctx ItemSpecGraph<E>,
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    output: &'ctx mut O,
    /// Workspace parameters.
    workspace_params: Option<WorkspaceParams<WorkspaceParamsKMaybe::Key>>,
    /// Type registry for `WorkspaceParams` deserialization.
    workspace_params_type_reg: TypeReg<WorkspaceParamsKMaybe::Key, BoxDt>,
    /// Profile parameters.
    profile_params: Option<ProfileParams<ProfileParamsKMaybe::Key>>,
    /// Type registry for `ProfileParams` deserialization.
    profile_params_type_reg: TypeReg<ProfileParamsKMaybe::Key, BoxDt>,
    /// Flow parameters.
    flow_params: Option<FlowParams<FlowParamsKMaybe::Key>>,
    /// Type registry for `FlowParams` deserialization.
    flow_params_type_reg: TypeReg<FlowParamsKMaybe::Key, BoxDt>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeyUnknown;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeyKnown<K>(PhantomData<K>);

pub trait KeyMaybe {
    type Key: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static;
}

impl KeyMaybe for KeyUnknown {
    type Key = ();
}

impl<K> KeyMaybe for KeyKnown<K>
where
    K: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    type Key = K;
}

impl<'ctx, E, O> CmdContextBuilder<'ctx, E, O, KeyUnknown, KeyUnknown, KeyUnknown>
where
    E: std::error::Error + From<Error>,
{
    /// Returns a builder for the command context.
    ///
    /// # Parameters
    ///
    /// * `workspace`: Defines how to discover the workspace.
    /// * `item_spec_graph`: Logic to run in the command.
    /// * `output`: [`OutputWrite`] to return values or errors. information to.
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
}

impl<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>
where
    E: std::error::Error + From<Error>,
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    #[cfg(feature = "output_progress")]
    /// Maximum number of progress messages to buffer.
    const PROGRESS_COUNT_MAX: usize = 256;

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
        let workspace_params_file = WorkspaceParamsFile::from(dirs.peace_dir());
        let profile_params_file = ProfileParamsFile::from(dirs.profile_dir());
        let flow_params_file = FlowParamsFile::from(dirs.flow_dir());
        let states_saved_file = StatesSavedFile::from(dirs.flow_dir());

        // Read existing init params from storage.
        self.init_params_deserialize(
            storage,
            &workspace_params_file,
            &profile_params_file,
            &flow_params_file,
        )
        .await?;

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

        self.init_params_serialize(
            storage,
            &workspace_params_file,
            &profile_params_file,
            &flow_params_file,
        )
        .await?;

        // Track items in memory.
        let mut resources = Resources::new();
        self.init_params_insert(&mut resources);
        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_params: _,
            workspace_params_type_reg: _,
            profile_params: _,
            profile_params_type_reg: _,
            flow_params: _,
            flow_params_type_reg: _,
        } = self;

        Self::workspace_dirs_insert(&mut resources, workspace);
        resources.insert(workspace_params_file);
        resources.insert(profile_params_file);
        resources.insert(flow_params_file);

        // Read existing states from storage.
        let states_type_regs = Self::states_type_regs(item_spec_graph);
        let states_saved = StatesSerializer::deserialize_saved_opt(
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

        #[cfg(feature = "output_progress")]
        let cmd_progress_tracker = {
            let (progress_tx, progress_rx) = mpsc::channel(Self::PROGRESS_COUNT_MAX);
            let multi_progress = MultiProgress::with_draw_target(ProgressDrawTarget::hidden());
            let progress_trackers = item_spec_graph.iter_insertion().fold(
                HashMap::with_capacity(item_spec_graph.node_count()),
                |mut progress_trackers, item_spec| {
                    let progress_bar = multi_progress.add(ProgressBar::hidden());
                    let progress_tracker = ProgressTracker::new(
                        item_spec.id().clone(),
                        progress_bar,
                        progress_tx.clone(),
                    );
                    progress_trackers.insert(item_spec.id().clone(), progress_tracker);
                    progress_trackers
                },
            );

            CmdProgressTracker::new(multi_progress, progress_rx, progress_trackers)
        };

        Ok(CmdContext {
            workspace,
            item_spec_graph,
            output,
            resources,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
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
    fn init_params_insert(&mut self, resources: &mut Resources<Empty>) {
        // TODO: we need to insert the raw type, right now we're inserting the box
        if let Some(workspace_params) = self.workspace_params.as_mut() {
            workspace_params
                .drain(..)
                .for_each(|(_key, workspace_param)| {
                    let workspace_param = workspace_param.into_inner().upcast();
                    let type_id = Resource::type_id(&*workspace_param);
                    resources.insert_raw(type_id, workspace_param);
                });
        }
        if let Some(profile_params) = self.profile_params.as_mut() {
            profile_params.drain(..).for_each(|(_key, profile_param)| {
                let profile_param = profile_param.into_inner().upcast();
                let type_id = Resource::type_id(&*profile_param);
                resources.insert_raw(type_id, profile_param);
            });
        }
        if let Some(flow_params) = self.flow_params.as_mut() {
            flow_params.drain(..).for_each(|(_key, flow_param)| {
                let flow_param = flow_param.into_inner().upcast();
                let type_id = Resource::type_id(&*flow_param);
                resources.insert_raw(type_id, flow_param);
            });
        }
    }

    async fn init_params_deserialize(
        &mut self,
        storage: &Storage,
        workspace_params_file: &WorkspaceParamsFile,
        profile_params_file: &ProfileParamsFile,
        flow_params_file: &FlowParamsFile,
    ) -> Result<(), E> {
        macro_rules! params_deserialize_and_merge {
            ($params:ident, $params_type_reg:ident, $params_deserialize_fn:ident, $init_file:ident) => {
                let params_deserialized = WorkspaceInitializer::$params_deserialize_fn(
                    storage,
                    &self.$params_type_reg,
                    $init_file,
                )
                .await?;
                match (self.$params.as_mut(), params_deserialized) {
                    (Some(params), Some(params_deserialized)) => {
                        // Merge `params` on top of `params_deserialized`.
                        // or, copy `params_deserialized` to `params` where
                        // there isn't a value.

                        params_deserialized
                            .into_inner()
                            .into_inner()
                            .into_iter()
                            .for_each(|(key, param)| {
                                if !params.contains_key(&key) {
                                    params.insert_raw(key, param);
                                }
                            });
                    }
                    (None, Some(params_deserialized)) => self.$params = Some(params_deserialized),
                    (Some(_), None) => {}
                    (None, None) => {}
                }
            };
        }

        params_deserialize_and_merge!(
            workspace_params,
            workspace_params_type_reg,
            workspace_params_deserialize,
            workspace_params_file
        );
        params_deserialize_and_merge!(
            profile_params,
            profile_params_type_reg,
            profile_params_deserialize,
            profile_params_file
        );
        params_deserialize_and_merge!(
            flow_params,
            flow_params_type_reg,
            flow_params_deserialize,
            flow_params_file
        );

        Ok(())
    }

    /// Serializes init params to storage.
    async fn init_params_serialize(
        &self,
        storage: &Storage,
        workspace_params_file: &WorkspaceParamsFile,
        profile_params_file: &ProfileParamsFile,
        flow_params_file: &FlowParamsFile,
    ) -> Result<(), E> {
        if let Some(workspace_params) = self.workspace_params.as_ref() {
            WorkspaceInitializer::workspace_params_serialize(
                storage,
                workspace_params,
                workspace_params_file,
            )
            .await?;
        }
        if let Some(profile_params) = self.profile_params.as_ref() {
            WorkspaceInitializer::profile_params_serialize(
                storage,
                profile_params,
                profile_params_file,
            )
            .await?;
        }
        if let Some(flow_params) = self.flow_params.as_ref() {
            WorkspaceInitializer::flow_params_serialize(storage, flow_params, flow_params_file)
                .await?;
        }

        Ok(())
    }
}

impl<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>
where
    E: std::error::Error,
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
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

impl<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe> IntoFuture
    for CmdContextBuilder<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>
where
    E: std::error::Error + From<Error>,
    WorkspaceParamsKMaybe: KeyMaybe + 'ctx,
    ProfileParamsKMaybe: KeyMaybe + 'ctx,
    FlowParamsKMaybe: KeyMaybe + 'ctx,
{
    type IntoFuture = CmdContextFuture<'ctx, E, O>;
    type Output = <Self::IntoFuture as Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.build())
    }
}

// Crazy stuff for ergonomic API usage

impl<'ctx, E, O, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<'ctx, E, O, KeyUnknown, ProfileParamsKMaybe, FlowParamsKMaybe>
where
    E: std::error::Error + From<Error>,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Adds a workspace parameter.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `workspace_param`: The workspace parameter to register.
    pub fn with_workspace_param<WorkspaceParamsK, WorkspaceParam>(
        self,
        k: WorkspaceParamsK,
        workspace_param: Option<WorkspaceParam>,
    ) -> CmdContextBuilder<
        'ctx,
        E,
        O,
        KeyKnown<WorkspaceParamsK>,
        ProfileParamsKMaybe,
        FlowParamsKMaybe,
    >
    where
        WorkspaceParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
        WorkspaceParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let Self {
            workspace,
            item_spec_graph,
            output,
            workspace_params: _,
            workspace_params_type_reg: _,
            profile_params,
            profile_params_type_reg,
            flow_params,
            flow_params_type_reg,
        } = self;

        let mut workspace_params_type_reg = TypeReg::<WorkspaceParamsK, BoxDt>::new();
        workspace_params_type_reg.register::<WorkspaceParam>(k.clone());
        let mut workspace_params = WorkspaceParams::<WorkspaceParamsK>::new();
        if let Some(workspace_param) = workspace_param {
            workspace_params.insert(k, workspace_param);
        }

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_params: Some(workspace_params),
            workspace_params_type_reg,
            profile_params,
            profile_params_type_reg,
            flow_params,
            flow_params_type_reg,
        }
    }
}

impl<'ctx, E, O, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<'ctx, E, O, KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>
where
    E: std::error::Error + From<Error>,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Adds a workspace parameter.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `workspace_param`: The workspace parameter to register.
    pub fn with_workspace_param<WorkspaceParam>(
        mut self,
        k: WorkspaceParamsK,
        workspace_param: Option<WorkspaceParam>,
    ) -> CmdContextBuilder<
        'ctx,
        E,
        O,
        KeyKnown<WorkspaceParamsK>,
        ProfileParamsKMaybe,
        FlowParamsKMaybe,
    >
    where
        WorkspaceParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.workspace_params_type_reg
            .register::<WorkspaceParam>(k.clone());
        if let (Some(workspace_params), Some(workspace_param)) =
            (self.workspace_params.as_mut(), workspace_param)
        {
            workspace_params.insert(k, workspace_param);
        }

        self
    }
}

impl<'ctx, E, O, WorkflowParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<'ctx, E, O, WorkflowParamsKMaybe, KeyUnknown, FlowParamsKMaybe>
where
    E: std::error::Error + From<Error>,
    WorkflowParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Adds a profile parameter.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `profile_param`: The profile parameter to register.
    pub fn with_profile_param<ProfileParamsK, ProfileParam>(
        self,
        k: ProfileParamsK,
        profile_param: Option<ProfileParam>,
    ) -> CmdContextBuilder<
        'ctx,
        E,
        O,
        WorkflowParamsKMaybe,
        KeyKnown<ProfileParamsK>,
        FlowParamsKMaybe,
    >
    where
        ProfileParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
        ProfileParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let Self {
            workspace,
            item_spec_graph,
            output,
            workspace_params,
            workspace_params_type_reg,
            profile_params: _,
            profile_params_type_reg: _,
            flow_params,
            flow_params_type_reg,
        } = self;

        let mut profile_params_type_reg = TypeReg::<ProfileParamsK, BoxDt>::new();
        profile_params_type_reg.register::<ProfileParam>(k.clone());
        let mut profile_params = ProfileParams::<ProfileParamsK>::new();
        if let Some(profile_param) = profile_param {
            profile_params.insert(k, profile_param);
        }

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_params,
            workspace_params_type_reg,
            profile_params: Some(profile_params),
            profile_params_type_reg,
            flow_params,
            flow_params_type_reg,
        }
    }
}

impl<'ctx, E, O, WorkflowParamsKMaybe, ProfileParamsK, FlowParamsKMaybe>
    CmdContextBuilder<'ctx, E, O, WorkflowParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>
where
    E: std::error::Error + From<Error>,
    WorkflowParamsKMaybe: KeyMaybe,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Adds a profile parameter.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `profile_param`: The profile parameter to register.
    pub fn with_profile_param<ProfileParam>(
        mut self,
        k: ProfileParamsK,
        profile_param: Option<ProfileParam>,
    ) -> CmdContextBuilder<
        'ctx,
        E,
        O,
        WorkflowParamsKMaybe,
        KeyKnown<ProfileParamsK>,
        FlowParamsKMaybe,
    >
    where
        ProfileParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.profile_params_type_reg
            .register::<ProfileParam>(k.clone());
        if let (Some(profile_params), Some(profile_param)) =
            (self.profile_params.as_mut(), profile_param)
        {
            profile_params.insert(k, profile_param);
        }

        self
    }
}

impl<'ctx, E, O, WorkflowParamsKMaybe, ProfileParamsKMaybe>
    CmdContextBuilder<'ctx, E, O, WorkflowParamsKMaybe, ProfileParamsKMaybe, KeyUnknown>
where
    E: std::error::Error + From<Error>,
    WorkflowParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
{
    /// Adds a flow parameter.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `flow_param`: The flow parameter to register.
    pub fn with_flow_param<FlowParamsK, FlowParam>(
        self,
        k: FlowParamsK,
        flow_param: Option<FlowParam>,
    ) -> CmdContextBuilder<
        'ctx,
        E,
        O,
        WorkflowParamsKMaybe,
        ProfileParamsKMaybe,
        KeyKnown<FlowParamsK>,
    >
    where
        FlowParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
        FlowParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let Self {
            workspace,
            item_spec_graph,
            output,
            workspace_params,
            workspace_params_type_reg,
            profile_params,
            profile_params_type_reg,
            flow_params: _,
            flow_params_type_reg: _,
        } = self;

        let mut flow_params_type_reg = TypeReg::<FlowParamsK, BoxDt>::new();
        flow_params_type_reg.register::<FlowParam>(k.clone());
        let mut flow_params = FlowParams::<FlowParamsK>::new();
        if let Some(flow_param) = flow_param {
            flow_params.insert(k, flow_param);
        }

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            workspace_params,
            workspace_params_type_reg,
            profile_params,
            profile_params_type_reg,
            flow_params: Some(flow_params),
            flow_params_type_reg,
        }
    }
}

impl<'ctx, E, O, WorkflowParamsKMaybe, ProfileParamsKMaybe, FlowParamsK>
    CmdContextBuilder<'ctx, E, O, WorkflowParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>
where
    E: std::error::Error + From<Error>,
    WorkflowParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Adds a flow parameter.
    ///
    /// # Parameters
    ///
    /// * `k`: Key to store the parameter with.
    /// * `flow_param`: The flow parameter to register.
    pub fn with_flow_param<FlowParam>(
        mut self,
        k: FlowParamsK,
        flow_param: Option<FlowParam>,
    ) -> CmdContextBuilder<
        'ctx,
        E,
        O,
        WorkflowParamsKMaybe,
        ProfileParamsKMaybe,
        KeyKnown<FlowParamsK>,
    >
    where
        FlowParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.flow_params_type_reg.register::<FlowParam>(k.clone());
        if let (Some(flow_params), Some(flow_param)) = (self.flow_params.as_mut(), flow_param) {
            flow_params.insert(k, flow_param);
        }

        self
    }
}
