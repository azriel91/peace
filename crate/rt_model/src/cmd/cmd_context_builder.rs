use std::{fmt::Debug, future::IntoFuture, hash::Hash, marker::PhantomData, pin::Pin};

use fn_graph::resman::Resource;
use futures::{Future, StreamExt, TryStreamExt};
use peace_cfg::{FlowId, Profile};
use peace_resources::{
    internal::{CmdDirs, FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    paths::StatesSavedFile,
    resources::ts::{Empty, SetUp},
    states::StatesSaved,
    Resources,
};
use peace_rt_model_core::cmd_context_params::{
    KeyKnown, KeyMaybe, KeyUnknown, ParamsTypeRegs, ParamsTypeRegsBuilder,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    cmd::{
        ts::{CmdContextCommon, FlowIdSelected, ProfileSelected},
        CmdContext, CmdDirsBuilder,
    },
    cmd_context_params::{FlowParams, ParamsKeys, ParamsKeysImpl, ProfileParams, WorkspaceParams},
    Error, ItemSpecGraph, StatesSerializer, StatesTypeRegs, Storage, Workspace,
    WorkspaceInitializer,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use std::collections::HashMap;

        use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget};
        use peace_cfg::progress::ProgressTracker;

        use crate::CmdProgressTracker;
    }
}

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
/// [`OutputWrite`]: peace_rt_model_core::OutputWrite
/// [`PeaceDir`]: peace_resources::paths::PeaceDir
/// [`Profile`]: peace_cfg::Profile
/// [`ProfileDir`]: peace_resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace_resources::paths::ProfileHistoryDir
/// [`States`]: peace_resources::States
/// [`WorkspaceDir`]: peace_resources::paths::WorkspaceDir
#[derive(Debug)]
pub struct CmdContextBuilder<
    'ctx,
    E,
    O,
    TS,
    PKeys = ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
> where
    PKeys: ParamsKeys + 'static,
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
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    params_type_regs_builder: ParamsTypeRegsBuilder<PKeys>,
    /// Identifier or namespace to distinguish execution environments.
    profile_selection: ProfileSelection<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Identifier or name of the chosen process flow.
    flow_id: FlowId,
    /// Workspace parameters.
    workspace_params: Option<WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>>,
    /// Profile parameters.
    profile_params: Option<ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>>,
    /// Flow parameters.
    flow_params: Option<FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>>,
    /// Marker.
    marker: PhantomData<TS>,
}

impl<'ctx, E, O>
    CmdContextBuilder<
        'ctx,
        E,
        O,
        CmdContextCommon,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    >
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
            params_type_regs_builder: ParamsTypeRegs::builder(),
            profile_selection: ProfileSelection::Selected(Profile::workspace_init()),
            flow_id: FlowId::workspace_init(),
            workspace_params: None,
            profile_params: None,
            flow_params: None,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, PKeys> CmdContextBuilder<'ctx, E, O, CmdContextCommon, PKeys>
where
    E: std::error::Error + From<Error>,
    PKeys: ParamsKeys + 'static,
{
    /// Sets the profile for the command execution.
    ///
    /// If this is not called, then the [`"workspace_init"`] profile is used.
    ///
    /// [`"workspace_init"`]: Profile::workspace_init
    pub fn with_profile(
        self,
        profile: Profile,
    ) -> CmdContextBuilder<'ctx, E, O, ProfileSelected, PKeys> {
        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection: _,
            flow_id: _,
            workspace_params,
            profile_params,
            flow_params,
            marker: _,
        } = self;

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection: ProfileSelection::Selected(profile),
            flow_id: FlowId::profile_init(),
            workspace_params,
            profile_params,
            flow_params,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<
        'ctx,
        E,
        O,
        CmdContextCommon,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
    E: std::error::Error + From<Error>,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Sets the profile for the command execution to be read from workspace
    /// params.
    ///
    /// `CmdContextBuilder::with_workspace_params` must have been called before
    /// this to set the workspace params key type.
    ///
    /// If this is not called, then the [`"workspace_init"`] profile is used.
    ///
    /// [`"workspace_init"`]: Profile::workspace_init
    pub fn with_profile_from_workspace_params(
        self,
        key: WorkspaceParamsK,
    ) -> CmdContextBuilder<
        'ctx,
        E,
        O,
        ProfileSelected,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    > {
        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection: _,
            flow_id: _,
            workspace_params,
            profile_params,
            flow_params,
            marker: _,
        } = self;

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection: ProfileSelection::WorkspaceParam(key),
            flow_id: FlowId::profile_init(),
            workspace_params,
            profile_params,
            flow_params,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, PKeys> CmdContextBuilder<'ctx, E, O, ProfileSelected, PKeys>
where
    E: std::error::Error + From<Error>,
    PKeys: ParamsKeys + 'static,
{
    /// Selects the flow ID for the workspace.
    ///
    /// If this is not called, then the [`"default"`] flow ID is used.
    ///
    /// [`"default"`]: FlowId::default
    pub fn with_flow_id(
        self,
        flow_id: FlowId,
    ) -> CmdContextBuilder<'ctx, E, O, FlowIdSelected, PKeys> {
        let CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection,
            flow_id: _,
            workspace_params,
            profile_params,
            flow_params,
            marker: _,
        } = self;

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection,
            flow_id,
            workspace_params,
            profile_params,
            flow_params,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, TS, PKeys> CmdContextBuilder<'ctx, E, O, TS, PKeys>
where
    E: std::error::Error + From<Error>,
    PKeys: ParamsKeys + 'static,
{
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_params`: Initialization parameters for the workspace.
    /// * `profile_params`: Initialization parameters for the profile.
    /// * `flow_params`: Initialization parameters for the flow.
    pub async fn build(
        mut self,
    ) -> Result<
        CmdContext<
            'ctx,
            E,
            O,
            SetUp,
            ParamsKeysImpl<
                PKeys::WorkspaceParamsKMaybe,
                PKeys::ProfileParamsKMaybe,
                PKeys::FlowParamsKMaybe,
            >,
        >,
        E,
    > {
        // 1. Load workspace params from workspace_params_file
        // 2. Determine profile from workspace params.
        // 3. Load profile params / flow params.

        let workspace_dirs = self.workspace.dirs();
        let storage = self.workspace.storage();
        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());

        macro_rules! params_deserialize_and_merge {
            ($params:ident, $params_type_reg:ident, $params_deserialize_fn:ident, $init_file:expr) => {
                let params_deserialized = WorkspaceInitializer::$params_deserialize_fn(
                    storage,
                    &self.params_type_regs_builder.$params_type_reg(),
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

        // Read existing workspace params from storage.
        params_deserialize_and_merge!(
            workspace_params,
            workspace_params_type_reg,
            workspace_params_deserialize,
            &workspace_params_file
        );

        let profile = match &self.profile_selection {
            ProfileSelection::Selected(profile) => profile.clone(),
            ProfileSelection::WorkspaceParam(workflow_params_k) => {
                if let Some(workspace_params) = self.workspace_params.as_ref() {
                    workspace_params
                        .get::<Profile, _>(workflow_params_k)
                        .ok_or(Error::WorkspaceParamsProfileNone)?
                        .clone()
                } else {
                    return Err(Error::WorkspaceParamsNoneForProfile.into());
                }
            }
        };

        let cmd_dirs =
            CmdDirsBuilder::build(workspace_dirs.peace_app_dir(), &profile, &self.flow_id);

        let profile_params_file = ProfileParamsFile::from(cmd_dirs.profile_dir());
        let flow_params_file = FlowParamsFile::from(cmd_dirs.flow_dir());
        let states_saved_file = StatesSavedFile::from(cmd_dirs.flow_dir());

        // Read existing profile and flow params from storage.
        params_deserialize_and_merge!(
            profile_params,
            profile_params_type_reg,
            profile_params_deserialize,
            &profile_params_file
        );
        params_deserialize_and_merge!(
            flow_params,
            flow_params_type_reg,
            flow_params_deserialize,
            &flow_params_file
        );

        // Create directories and write init parameters to storage.
        #[cfg(target_arch = "wasm32")]
        WorkspaceInitializer::dirs_initialize(storage, workspace_dirs, &cmd_dirs).await?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let workspace_dir = workspace_dirs.workspace_dir();
            std::env::set_current_dir(workspace_dir).map_err(|error| Error::CurrentDirSet {
                workspace_dir: workspace_dir.clone(),
                error,
            })?;

            WorkspaceInitializer::dirs_initialize(workspace_dirs, &cmd_dirs).await?;
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
            params_type_regs_builder,
            profile_selection: _,
            flow_id,
            workspace_params: _,
            profile_params: _,
            flow_params: _,
            marker: _,
        } = self;

        // Read existing states from storage.
        //
        // It is not possible to load saved states does not work when running a
        // cross-profile command, and the flows have different states.
        //
        // e.g. different item spec graphs.
        //
        // Most prominent example is workspace initialization, where the states saved
        // per item spec for workspace initialization are likely different to
        // application specific flows.
        let states_type_regs = Self::states_type_regs(item_spec_graph);
        let states_saved = StatesSerializer::deserialize_saved_opt(
            &flow_id,
            storage,
            states_type_regs.states_current_type_reg(),
            &states_saved_file,
        )
        .await?
        .map(Into::<StatesSaved>::into);
        if let Some(states_saved) = states_saved {
            resources.insert(states_saved);
        }

        Self::workspace_dirs_insert(&mut resources, workspace, profile, flow_id, cmd_dirs);
        resources.insert(workspace_params_file);
        resources.insert(profile_params_file);
        resources.insert(flow_params_file);

        // Call each `ItemSpec`'s initialization function.
        let resources = Self::item_spec_graph_setup(item_spec_graph, resources).await?;

        #[cfg(feature = "output_progress")]
        let cmd_progress_tracker = {
            let multi_progress = MultiProgress::with_draw_target(ProgressDrawTarget::hidden());
            let progress_trackers = item_spec_graph.iter_insertion().fold(
                HashMap::with_capacity(item_spec_graph.node_count()),
                |mut progress_trackers, item_spec| {
                    let progress_bar = multi_progress.add(ProgressBar::hidden());
                    let progress_tracker = ProgressTracker::new(progress_bar);
                    progress_trackers.insert(item_spec.id().clone(), progress_tracker);
                    progress_trackers
                },
            );

            CmdProgressTracker::new(multi_progress, progress_trackers)
        };

        let params_type_regs = params_type_regs_builder.build();

        Ok(CmdContext {
            workspace,
            item_spec_graph,
            output,
            params_type_regs,
            resources,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            marker: PhantomData,
        })
    }

    /// Inserts workspace directory resources into the `Resources` map.
    fn workspace_dirs_insert(
        resources: &mut Resources<Empty>,
        workspace: &Workspace,
        profile: Profile,
        flow_id: FlowId,
        cmd_dirs: CmdDirs,
    ) {
        let (app_name, workspace_dirs, storage) = workspace.clone().into_inner();
        let (workspace_dir, peace_dir, peace_app_dir) = workspace_dirs.into_inner();
        let (profile_dir, profile_history_dir, flow_dir) = cmd_dirs.into_inner();

        resources.insert(workspace_dir);
        resources.insert(peace_dir);
        resources.insert(peace_app_dir);
        resources.insert(profile_dir);
        resources.insert(profile_history_dir);
        resources.insert(flow_dir);

        resources.insert(app_name);
        resources.insert(profile);
        resources.insert(flow_id);
        resources.insert(storage);
    }

    /// Inserts init params into the `Resources` map.
    fn init_params_insert(&mut self, resources: &mut Resources<Empty>) {
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

impl<'ctx, E, O, TS, PKeys> CmdContextBuilder<'ctx, E, O, TS, PKeys>
where
    E: std::error::Error,
    PKeys: ParamsKeys + 'static,
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
pub type CmdContextFuture<'ctx, E, O, PKeys> =
    Pin<Box<dyn Future<Output = Result<CmdContext<'ctx, E, O, SetUp, PKeys>, E>> + 'ctx>>;

impl<'ctx, E, O, TS, PKeys> IntoFuture for CmdContextBuilder<'ctx, E, O, TS, PKeys>
where
    E: std::error::Error + From<Error>,
    TS: 'static,
    PKeys: ParamsKeys + 'static,
{
    type IntoFuture = CmdContextFuture<
        'ctx,
        E,
        O,
        ParamsKeysImpl<
            PKeys::WorkspaceParamsKMaybe,
            PKeys::ProfileParamsKMaybe,
            PKeys::FlowParamsKMaybe,
        >,
    >;
    type Output = <Self::IntoFuture as Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.build())
    }
}

// Crazy stuff for ergonomic API usage

impl<'ctx, E, O, TS, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<
        'ctx,
        E,
        O,
        TS,
        ParamsKeysImpl<KeyUnknown, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
    E: std::error::Error + From<Error>,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Adds a workspace parameter.
    ///
    /// Currently there is no means in code to deliberately unset any previously
    /// stored value. This can be made possibly by defining a
    /// `WorkspaceParamsBuilder` that determines a `None` value as a deliberate
    /// erasure of any previous value.
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
        TS,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
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
            params_type_regs_builder,
            profile_selection,
            flow_id,
            workspace_params: _,
            profile_params,
            flow_params,
            marker: _,
        } = self;

        let mut params_type_regs_builder =
            params_type_regs_builder.with_workspace_params_k::<WorkspaceParamsK>();
        params_type_regs_builder
            .workspace_params_type_reg_mut()
            .register::<WorkspaceParam>(k.clone());
        let mut workspace_params = WorkspaceParams::<WorkspaceParamsK>::new();
        if let Some(workspace_param) = workspace_param {
            workspace_params.insert(k, workspace_param);
        }

        let profile_selection = match profile_selection {
            ProfileSelection::Selected(profile) => ProfileSelection::Selected(profile),
            ProfileSelection::WorkspaceParam(()) => {
                unreachable!(
                    "`CmdContextBuilder::with_profile_from_workspace_params` can only be called\n\
                    after `with_workspace_param` has been called, so `profile_selection`\n\
                    will never be `WorkspaceParam` at this point."
                )
            }
        };

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection,
            flow_id,
            workspace_params: Some(workspace_params),
            profile_params,
            flow_params,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<
        'ctx,
        E,
        O,
        TS,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
    E: std::error::Error + From<Error>,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Adds a workspace parameter.
    ///
    /// Currently there is no means in code to deliberately unset any previously
    /// stored value. This can be made possibly by defining a
    /// `WorkspaceParamsBuilder` that determines a `None` value as a deliberate
    /// erasure of any previous value.
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
        TS,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
    where
        WorkspaceParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.params_type_regs_builder
            .workspace_params_type_reg_mut()
            .register::<WorkspaceParam>(k.clone());
        let Some(workspace_params) = self.workspace_params.as_mut() else {
            unreachable!("This is set to `Some` in `Self::with_params_type_regs_builder`");
        };
        if let Some(workspace_param) = workspace_param {
            workspace_params.insert(k, workspace_param);
        }

        self
    }
}

impl<'ctx, E, O, TS, WorkflowParamsKMaybe, FlowParamsKMaybe>
    CmdContextBuilder<
        'ctx,
        E,
        O,
        TS,
        ParamsKeysImpl<WorkflowParamsKMaybe, KeyUnknown, FlowParamsKMaybe>,
    >
where
    E: std::error::Error + From<Error>,
    WorkflowParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Adds a profile parameter.
    ///
    /// Currently there is no means in code to deliberately unset any previously
    /// stored value. This can be made possibly by defining a
    /// `ProfileParamsBuilder` that determines a `None` value as a deliberate
    /// erasure of any previous value.
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
        TS,
        ParamsKeysImpl<WorkflowParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
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
            params_type_regs_builder,
            profile_selection,
            flow_id,
            workspace_params,
            profile_params: _,
            flow_params,
            marker: _,
        } = self;

        let mut params_type_regs_builder =
            params_type_regs_builder.with_profile_params_k::<ProfileParamsK>();
        params_type_regs_builder
            .profile_params_type_reg_mut()
            .register::<ProfileParam>(k.clone());
        let mut profile_params = ProfileParams::<ProfileParamsK>::new();
        if let Some(profile_param) = profile_param {
            profile_params.insert(k, profile_param);
        }

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection,
            flow_id,
            workspace_params,
            profile_params: Some(profile_params),
            flow_params,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, TS, WorkflowParamsKMaybe, ProfileParamsK, FlowParamsKMaybe>
    CmdContextBuilder<
        'ctx,
        E,
        O,
        TS,
        ParamsKeysImpl<WorkflowParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
    >
where
    E: std::error::Error + From<Error>,
    WorkflowParamsKMaybe: KeyMaybe,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Adds a profile parameter.
    ///
    /// Currently there is no means in code to deliberately unset any previously
    /// stored value. This can be made possibly by defining a
    /// `ProfileParamsBuilder` that determines a `None` value as a deliberate
    /// erasure of any previous value.
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
        TS,
        ParamsKeysImpl<WorkflowParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
    >
    where
        ProfileParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.params_type_regs_builder
            .profile_params_type_reg_mut()
            .register::<ProfileParam>(k.clone());
        let Some(profile_params) = self.profile_params.as_mut() else {
            unreachable!("This is set to `Some` in `Self::with_params_type_regs_builder`");
        };
        if let Some(profile_param) = profile_param {
            profile_params.insert(k, profile_param);
        }

        self
    }
}

impl<'ctx, E, O, TS, WorkspaceParamsKMaybe, ProfileParamsKMaybe>
    CmdContextBuilder<
        'ctx,
        E,
        O,
        TS,
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyUnknown>,
    >
where
    E: std::error::Error + From<Error>,
    ProfileParamsKMaybe: KeyMaybe,
    WorkspaceParamsKMaybe: KeyMaybe,
{
    /// Adds a flow parameter.
    ///
    /// Currently there is no means in code to deliberately unset any previously
    /// stored value. This can be made possibly by defining a
    /// `FlowParamsBuilder` that determines a `None` value as a deliberate
    /// erasure of any previous value.
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
        TS,
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
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
            params_type_regs_builder,
            profile_selection,
            flow_id,
            workspace_params,
            profile_params,
            flow_params: _,
            marker: _,
        } = self;

        let mut params_type_regs_builder =
            params_type_regs_builder.with_flow_params_k::<FlowParamsK>();
        params_type_regs_builder
            .flow_params_type_reg_mut()
            .register::<FlowParam>(k.clone());
        let mut flow_params = FlowParams::<FlowParamsK>::new();
        if let Some(flow_param) = flow_param {
            flow_params.insert(k, flow_param);
        }

        CmdContextBuilder {
            workspace,
            item_spec_graph,
            output,
            params_type_regs_builder,
            profile_selection,
            flow_id,
            workspace_params,
            profile_params,
            flow_params: Some(flow_params),
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, TS, WorkflowParamsKMaybe, ProfileParamsKMaybe, FlowParamsK>
    CmdContextBuilder<
        'ctx,
        E,
        O,
        TS,
        ParamsKeysImpl<WorkflowParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
    >
where
    E: std::error::Error + From<Error>,
    WorkflowParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Adds a flow parameter.
    ///
    /// Currently there is no means in code to deliberately unset any previously
    /// stored value. This can be made possibly by defining a
    /// `FlowParamsBuilder` that determines a `None` value as a deliberate
    /// erasure of any previous value.
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
        TS,
        ParamsKeysImpl<WorkflowParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
    >
    where
        FlowParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        self.params_type_regs_builder
            .flow_params_type_reg_mut()
            .register::<FlowParam>(k.clone());
        let Some(flow_params) = self.flow_params.as_mut() else {
            unreachable!("This is set to `Some` in `Self::with_params_type_regs_builder`");
        };
        if let Some(flow_param) = flow_param {
            flow_params.insert(k, flow_param);
        }

        self
    }
}

#[derive(Debug)]
enum ProfileSelection<WorkflowParamsK> {
    /// A `Profile` is selected.
    Selected(Profile),
    /// `Profile` selection is deferred until command context build.
    WorkspaceParam(WorkflowParamsK),
}
