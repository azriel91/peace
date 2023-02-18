use std::{fmt::Debug, hash::Hash};

use peace_core::{FlowId, Profile};
use peace_resources::{
    internal::WorkspaceParamsFile,
    paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    resources::ts::Empty,
    Resources,
};
use peace_rt_model::{
    cmd::CmdDirsBuilder,
    cmd_context_params::{
        KeyKnown, KeyMaybe, KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs,
        ParamsTypeRegsBuilder, WorkspaceParams,
    },
    fn_graph::resman::Resource,
    Error, Storage, Workspace, WorkspaceInitializer,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    ctx::CmdCtx,
    scopes::{NoProfileNoFlow, SingleProfileSingleFlow},
};

#[cfg(not(target_arch = "wasm32"))]
use peace_rt_model::NativeError;

pub(crate) use self::{
    flow_id_selection::{FlowIdNotSelected, FlowIdSelected},
    profile_selection::{ProfileFromWorkspaceParam, ProfileNotSelected, ProfileSelected},
    single_profile_single_flow_builder::SingleProfileSingleFlowBuilder,
    workspace_params_selection::{WorkspaceParamsNone, WorkspaceParamsSome},
};

mod flow_id_selection;
mod profile_selection;
mod single_profile_single_flow_builder;
mod workspace_params_selection;

/// Collects parameters and initializes values relevant to the built [`CmdCtx`].
#[derive(Debug)]
pub struct CmdCtxBuilder<'ctx, ScopeBuilder, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Data held while building `CmdCtx`.
    scope_builder: ScopeBuilder,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    params_type_regs_builder: ParamsTypeRegsBuilder<PKeys>,
}

impl<'ctx, ScopeBuilder, PKeys> CmdCtxBuilder<'ctx, ScopeBuilder, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Serializes workspace params to storage.
    async fn workspace_params_serialize(
        workspace_params: Option<&WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>>,
        storage: &Storage,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<(), Error> {
        if let Some(workspace_params) = workspace_params {
            WorkspaceInitializer::workspace_params_serialize(
                storage,
                workspace_params,
                workspace_params_file,
            )
            .await?;
        }

        Ok(())
    }

    /// Inserts workspace params into the `Resources` map.
    fn workspace_params_insert(
        mut workspace_params: Option<
            WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        >,
        resources: &mut Resources<Empty>,
    ) {
        if let Some(workspace_params) = workspace_params.as_mut() {
            workspace_params
                .drain(..)
                .for_each(|(_key, workspace_param)| {
                    let workspace_param = workspace_param.into_inner().upcast();
                    let type_id = Resource::type_id(&*workspace_param);
                    resources.insert_raw(type_id, workspace_param);
                });
        }
    }
}

impl<'ctx>
    CmdCtxBuilder<'ctx, NoProfileNoFlow, ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>
{
    /// Returns a `CmdCtxBuilder` for no profile.
    pub fn no_profile_no_flow(workspace: &'ctx Workspace) -> Self {
        Self {
            workspace,
            scope_builder: NoProfileNoFlow,
            params_type_regs_builder: ParamsTypeRegs::builder(),
        }
    }
}

impl<'ctx, PKeys> CmdCtxBuilder<'ctx, NoProfileNoFlow, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters.
    pub fn build(
        self,
    ) -> CmdCtx<
        'ctx,
        NoProfileNoFlow,
        ParamsKeysImpl<
            PKeys::WorkspaceParamsKMaybe,
            PKeys::ProfileParamsKMaybe,
            PKeys::FlowParamsKMaybe,
        >,
    > {
        let CmdCtxBuilder {
            workspace,
            scope_builder: scope,
            params_type_regs_builder,
        } = self;

        let params_type_regs = params_type_regs_builder.build();

        CmdCtx {
            workspace,
            scope,
            params_type_regs,
        }
    }
}

impl<'ctx>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<ProfileNotSelected, FlowIdNotSelected, WorkspaceParamsNone>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    >
{
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn single_profile_single_flow(workspace: &'ctx Workspace) -> Self {
        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection: ProfileNotSelected,
            flow_id_selection: FlowIdNotSelected,
            workspace_params_selection: WorkspaceParamsNone,
        };

        Self {
            workspace,
            scope_builder,
            params_type_regs_builder: ParamsTypeRegs::builder(),
        }
    }
}

impl<'ctx, ProfileSelection, FlowIdSelection, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<ProfileSelection, FlowIdSelection, WorkspaceParamsNone>,
        ParamsKeysImpl<KeyUnknown, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
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
    ) -> CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdSelection,
            WorkspaceParamsSome<WorkspaceParamsK>,
        >,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
    where
        WorkspaceParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
        WorkspaceParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let Self {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection,
                    flow_id_selection,
                    workspace_params_selection: WorkspaceParamsNone,
                },
            params_type_regs_builder,
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

        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection,
            flow_id_selection,
            workspace_params_selection: WorkspaceParamsSome(Some(workspace_params)),
        };

        CmdCtxBuilder {
            workspace,
            scope_builder,
            params_type_regs_builder,
        }
    }
}

impl<
    'ctx,
    ProfileSelection,
    FlowIdSelection,
    WorkspaceParamsK,
    ProfileParamsKMaybe,
    FlowParamsKMaybe,
>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdSelection,
            WorkspaceParamsSome<WorkspaceParamsK>,
        >,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
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
        self,
        k: WorkspaceParamsK,
        workspace_param: Option<WorkspaceParam>,
    ) -> CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdSelection,
            WorkspaceParamsSome<WorkspaceParamsK>,
        >,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
    where
        WorkspaceParam: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let Self {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection,
                    flow_id_selection,
                    mut workspace_params_selection,
                },
            mut params_type_regs_builder,
        } = self;

        params_type_regs_builder
            .workspace_params_type_reg_mut()
            .register::<WorkspaceParam>(k.clone());
        let Some(workspace_params) = workspace_params_selection.0.as_mut() else {
            unreachable!("This is set to `Some` in `Self::with_workspace_param`");
        };
        if let Some(workspace_param) = workspace_param {
            workspace_params.insert(k, workspace_param);
        }

        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection,
            flow_id_selection,
            workspace_params_selection,
        };

        CmdCtxBuilder {
            workspace,
            scope_builder,
            params_type_regs_builder,
        }
    }
}

impl<'ctx, FlowIdSelection, WorkspaceParamsSelection, PKeys>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileNotSelected,
            FlowIdSelection,
            WorkspaceParamsSelection,
        >,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    pub fn with_profile(
        self,
        profile: Profile,
    ) -> CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<ProfileSelected, FlowIdSelection, WorkspaceParamsSelection>,
        PKeys,
    > {
        let Self {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: _,
                    flow_id_selection,
                    workspace_params_selection: workspace_params,
                },
            params_type_regs_builder,
        } = self;

        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection: ProfileSelected(profile),
            flow_id_selection,
            workspace_params_selection: workspace_params,
        };

        CmdCtxBuilder {
            workspace,
            scope_builder,
            params_type_regs_builder,
        }
    }
}

impl<'ctx, FlowIdSelection, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileNotSelected,
            FlowIdSelection,
            WorkspaceParamsSome<WorkspaceParamsK>,
        >,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    pub fn with_profile_from_workspace_param<'key>(
        self,
        workspace_param_k: &'key WorkspaceParamsK,
    ) -> CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileFromWorkspaceParam<'key, WorkspaceParamsK>,
            FlowIdSelection,
            WorkspaceParamsSome<WorkspaceParamsK>,
        >,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    > {
        let Self {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: _,
                    flow_id_selection,
                    workspace_params_selection: workspace_params,
                },
            params_type_regs_builder,
        } = self;

        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection: ProfileFromWorkspaceParam(workspace_param_k),
            flow_id_selection,
            workspace_params_selection: workspace_params,
        };

        CmdCtxBuilder {
            workspace,
            scope_builder,
            params_type_regs_builder,
        }
    }
}

impl<'ctx, PKeys, ProfileSelection, WorkspaceParamsSelection>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdNotSelected,
            WorkspaceParamsSelection,
        >,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    pub fn with_flow_id(
        self,
        flow_id: FlowId,
    ) -> CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<ProfileSelection, FlowIdSelected, WorkspaceParamsSelection>,
        PKeys,
    > {
        let Self {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection,
                    flow_id_selection: _,
                    workspace_params_selection: workspace_params,
                },
            params_type_regs_builder,
        } = self;

        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection,
            flow_id_selection: FlowIdSelected(flow_id),
            workspace_params_selection: workspace_params,
        };

        CmdCtxBuilder {
            workspace,
            scope_builder,
            params_type_regs_builder,
        }
    }
}

impl<'ctx, PKeys>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<ProfileSelected, FlowIdSelected, WorkspaceParamsNone>,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters
    pub async fn build(
        self,
    ) -> Result<
        CmdCtx<
            'ctx,
            SingleProfileSingleFlow,
            ParamsKeysImpl<
                PKeys::WorkspaceParamsKMaybe,
                PKeys::ProfileParamsKMaybe,
                PKeys::FlowParamsKMaybe,
            >,
        >,
        Error,
    > {
        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileSelected(profile),
                    flow_id_selection: FlowIdSelected(flow_id),
                    workspace_params_selection: WorkspaceParamsNone,
                },
            params_type_regs_builder,
        } = self;

        let peace_app_dir = workspace.dirs().peace_app_dir();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let scope = SingleProfileSingleFlow::new(
            profile,
            profile_dir,
            profile_history_dir,
            flow_id,
            flow_dir,
        );

        let params_type_regs = params_type_regs_builder.build();

        Ok(CmdCtx {
            workspace,
            scope,
            params_type_regs,
        })
    }
}

impl<'ctx, PKeys>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelected,
            FlowIdSelected,
            WorkspaceParamsSome<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        >,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters
    pub async fn build(
        mut self,
    ) -> Result<
        CmdCtx<
            'ctx,
            SingleProfileSingleFlow,
            ParamsKeysImpl<
                PKeys::WorkspaceParamsKMaybe,
                PKeys::ProfileParamsKMaybe,
                PKeys::FlowParamsKMaybe,
            >,
        >,
        Error,
    > {
        // Values shared by subsequent functions.
        let workspace_dirs = self.workspace.dirs();
        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());

        self.workspace_params_merge(&workspace_params_file).await?;

        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileSelected(profile),
                    flow_id_selection: FlowIdSelected(flow_id),
                    workspace_params_selection: WorkspaceParamsSome(workspace_params),
                },
            params_type_regs_builder,
        } = self;

        let peace_app_dir = workspace.dirs().peace_app_dir();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let cmd_dirs = CmdDirsBuilder::build(workspace_dirs.peace_app_dir(), &profile, &flow_id);

        // Create directories and write init parameters to storage.
        #[cfg(target_arch = "wasm32")]
        WorkspaceInitializer::dirs_initialize(storage, workspace_dirs, &cmd_dirs).await?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let workspace_dir = workspace_dirs.workspace_dir();
            WorkspaceInitializer::dirs_initialize(workspace_dirs, &cmd_dirs).await?;

            std::env::set_current_dir(workspace_dir).map_err(|error| {
                Error::Native(NativeError::CurrentDirSet {
                    workspace_dir: workspace_dir.clone(),
                    error,
                })
            })?;
        }

        // Serialize params to `PeaceAppDir`.
        Self::workspace_params_serialize(
            workspace_params.as_ref(),
            workspace.storage(),
            &workspace_params_file,
        )
        .await?;

        // Track items in memory.
        let mut resources = Resources::new();
        Self::workspace_params_insert(workspace_params, &mut resources);

        let scope = SingleProfileSingleFlow::new(
            profile,
            profile_dir,
            profile_history_dir,
            flow_id,
            flow_dir,
        );

        let params_type_regs = params_type_regs_builder.build();

        Ok(CmdCtx {
            workspace,
            scope,
            params_type_regs,
        })
    }
}

impl<'ctx, 'key, PKeys>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileFromWorkspaceParam<'key, <PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
            FlowIdSelected,
            WorkspaceParamsSome<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        >,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters
    pub async fn build(
        mut self,
    ) -> Result<
        CmdCtx<
            'ctx,
            SingleProfileSingleFlow,
            ParamsKeysImpl<
                PKeys::WorkspaceParamsKMaybe,
                PKeys::ProfileParamsKMaybe,
                PKeys::FlowParamsKMaybe,
            >,
        >,
        Error,
    > {
        // Values shared by subsequent functions.
        let workspace_dirs = self.workspace.dirs();
        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());

        self.workspace_params_merge(&workspace_params_file).await?;

        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileFromWorkspaceParam(workspace_params_k),
                    flow_id_selection: FlowIdSelected(flow_id),
                    workspace_params_selection: WorkspaceParamsSome(workspace_params),
                },
            params_type_regs_builder,
        } = self;

        let peace_app_dir = workspace.dirs().peace_app_dir();

        let profile = workspace_params
            .as_ref()
            .ok_or(Error::WorkspaceParamsNoneForProfile)?
            .get(workspace_params_k)
            .cloned()
            .ok_or(Error::WorkspaceParamsProfileNone)?;
        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let cmd_dirs = CmdDirsBuilder::build(workspace_dirs.peace_app_dir(), &profile, &flow_id);

        // Create directories and write init parameters to storage.
        #[cfg(target_arch = "wasm32")]
        WorkspaceInitializer::dirs_initialize(storage, workspace_dirs, &cmd_dirs).await?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let workspace_dir = workspace_dirs.workspace_dir();
            WorkspaceInitializer::dirs_initialize(workspace_dirs, &cmd_dirs).await?;

            std::env::set_current_dir(workspace_dir).map_err(|error| {
                Error::Native(NativeError::CurrentDirSet {
                    workspace_dir: workspace_dir.clone(),
                    error,
                })
            })?;
        }

        // Serialize params to `PeaceAppDir`.
        Self::workspace_params_serialize(
            workspace_params.as_ref(),
            workspace.storage(),
            &workspace_params_file,
        )
        .await?;

        // Track items in memory.
        let mut resources = Resources::new();
        Self::workspace_params_insert(workspace_params, &mut resources);

        let scope = SingleProfileSingleFlow::new(
            profile,
            profile_dir,
            profile_history_dir,
            flow_id,
            flow_dir,
        );

        let params_type_regs = params_type_regs_builder.build();

        Ok(CmdCtx {
            workspace,
            scope,
            params_type_regs,
        })
    }
}

impl<'ctx, 'key, PKeys, ProfileSelection>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdSelected,
            WorkspaceParamsSome<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        >,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    /// Merges workspace params provided by the caller with the workspace params
    /// on disk.
    async fn workspace_params_merge(
        &mut self,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<(), Error> {
        let storage = self.workspace.storage();
        let params_deserialized = WorkspaceInitializer::workspace_params_deserialize::<
            <PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key,
        >(
            storage,
            self.params_type_regs_builder.workspace_params_type_reg(),
            &workspace_params_file,
        )
        .await?;
        match (
            self.scope_builder.workspace_params_selection.0.as_mut(),
            params_deserialized,
        ) {
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
            (None, Some(params_deserialized)) => {
                self.scope_builder.workspace_params_selection.0 = Some(params_deserialized)
            }
            (Some(_), None) => {}
            (None, None) => {}
        }

        Ok(())
    }
}
