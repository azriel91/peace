#![allow(clippy::type_complexity)]

use std::{fmt::Debug, hash::Hash};

use peace_resources::{
    internal::{ProfileParamsFile, WorkspaceParamsFile},
    paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    Resources,
};
use peace_rt_model::{
    cmd::CmdDirsBuilder,
    cmd_context_params::{KeyMaybe, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs},
    Error, Workspace, WorkspaceInitializer,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    ctx::{
        cmd_ctx_builder::{
            flow_id_selection::FlowIdSelected,
            flow_params_selection::{FlowParamsNone, FlowParamsSome},
            profile_params_selection::{ProfileParamsNone, ProfileParamsSome},
            profile_selection::{ProfileFromWorkspaceParam, ProfileSelected},
            workspace_params_selection::{WorkspaceParamsNone, WorkspaceParamsSome},
        },
        CmdCtx, CmdCtxBuilder,
    },
    scopes::SingleProfileSingleFlow,
};

#[cfg(not(target_arch = "wasm32"))]
use peace_rt_model::NativeError;

/// Data stored by `CmdCtxBuilder` while building a
/// `CmdCtx<SingleProfileSingleFlow>`.
#[peace_code_gen::cmd_ctx_builder_impl]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleProfileSingleFlowBuilder;

impl<'ctx, PKeys>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelected,
            FlowIdSelected,
            WorkspaceParamsNone,
            ProfileParamsNone,
            FlowParamsNone,
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
                    profile_params_selection: ProfileParamsNone,
                    flow_params_selection: FlowParamsNone,
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
            WorkspaceParamsNone,
            ProfileParamsSome<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
            FlowParamsNone,
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
        let storage = self.workspace.storage();

        let cmd_dirs = CmdDirsBuilder::build(
            workspace_dirs.peace_app_dir(),
            &self.scope_builder.profile_selection.0,
            &self.scope_builder.flow_id_selection.0,
        );

        let profile_params_file = ProfileParamsFile::from(cmd_dirs.profile_dir());
        self.profile_params_merge(&profile_params_file).await?;

        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileSelected(profile),
                    flow_id_selection: FlowIdSelected(flow_id),
                    workspace_params_selection: WorkspaceParamsNone,
                    profile_params_selection: ProfileParamsSome(profile_params),
                    flow_params_selection: FlowParamsNone,
                },
            params_type_regs_builder,
        } = self;

        let peace_app_dir = workspace.dirs().peace_app_dir();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

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
        Self::profile_params_serialize(profile_params.as_ref(), storage, &profile_params_file)
            .await?;

        // Track items in memory.
        let mut resources = Resources::new();
        Self::profile_params_insert(profile_params, &mut resources);

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
            ProfileParamsSome<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
            FlowParamsNone,
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
        let storage = self.workspace.storage();
        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());

        self.workspace_params_merge(&workspace_params_file).await?;

        let cmd_dirs = CmdDirsBuilder::build(
            workspace_dirs.peace_app_dir(),
            &self.scope_builder.profile_selection.0,
            &self.scope_builder.flow_id_selection.0,
        );

        let profile_params_file = ProfileParamsFile::from(cmd_dirs.profile_dir());
        self.profile_params_merge(&profile_params_file).await?;

        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileSelected(profile),
                    flow_id_selection: FlowIdSelected(flow_id),
                    workspace_params_selection: WorkspaceParamsSome(workspace_params),
                    profile_params_selection: ProfileParamsSome(profile_params),
                    flow_params_selection: FlowParamsNone,
                },
            params_type_regs_builder,
        } = self;

        let peace_app_dir = workspace.dirs().peace_app_dir();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

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
            storage,
            &workspace_params_file,
        )
        .await?;
        Self::profile_params_serialize(profile_params.as_ref(), storage, &profile_params_file)
            .await?;

        // Track items in memory.
        let mut resources = Resources::new();
        Self::workspace_params_insert(workspace_params, &mut resources);
        Self::profile_params_insert(profile_params, &mut resources);

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
            ProfileParamsNone,
            FlowParamsNone,
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
        let storage = self.workspace.storage();
        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());

        self.workspace_params_merge(&workspace_params_file).await?;

        let cmd_dirs = CmdDirsBuilder::build(
            workspace_dirs.peace_app_dir(),
            &self.scope_builder.profile_selection.0,
            &self.scope_builder.flow_id_selection.0,
        );

        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileSelected(profile),
                    flow_id_selection: FlowIdSelected(flow_id),
                    workspace_params_selection: WorkspaceParamsSome(workspace_params),
                    profile_params_selection: ProfileParamsNone,
                    flow_params_selection: FlowParamsNone,
                },
            params_type_regs_builder,
        } = self;

        let peace_app_dir = workspace.dirs().peace_app_dir();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

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
            storage,
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
            ProfileParamsNone,
            FlowParamsNone,
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
        let storage = self.workspace.storage();
        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());

        self.workspace_params_merge(&workspace_params_file).await?;

        let profile = self
            .scope_builder
            .workspace_params_selection
            .0
            .as_ref()
            .ok_or(Error::WorkspaceParamsNoneForProfile)?
            .get(self.scope_builder.profile_selection.0)
            .cloned()
            .ok_or(Error::WorkspaceParamsProfileNone)?;

        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileFromWorkspaceParam(_workspace_params_k),
                    flow_id_selection: FlowIdSelected(flow_id),
                    workspace_params_selection: WorkspaceParamsSome(workspace_params),
                    profile_params_selection: ProfileParamsNone,
                    flow_params_selection: FlowParamsNone,
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
            storage,
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
            ProfileParamsSome<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
            FlowParamsNone,
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
        let storage = self.workspace.storage();
        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());

        self.workspace_params_merge(&workspace_params_file).await?;

        let profile = self
            .scope_builder
            .workspace_params_selection
            .0
            .as_ref()
            .ok_or(Error::WorkspaceParamsNoneForProfile)?
            .get(self.scope_builder.profile_selection.0)
            .cloned()
            .ok_or(Error::WorkspaceParamsProfileNone)?;

        let cmd_dirs = CmdDirsBuilder::build(
            workspace_dirs.peace_app_dir(),
            &profile,
            &self.scope_builder.flow_id_selection.0,
        );

        let profile_params_file = ProfileParamsFile::from(cmd_dirs.profile_dir());
        self.profile_params_merge(&profile_params_file).await?;

        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileFromWorkspaceParam(_workspace_params_k),
                    flow_id_selection: FlowIdSelected(flow_id),
                    workspace_params_selection: WorkspaceParamsSome(workspace_params),
                    profile_params_selection: ProfileParamsSome(profile_params),
                    flow_params_selection: FlowParamsNone,
                },
            params_type_regs_builder,
        } = self;

        let peace_app_dir = workspace.dirs().peace_app_dir();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

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
            storage,
            &workspace_params_file,
        )
        .await?;
        Self::profile_params_serialize(profile_params.as_ref(), storage, &profile_params_file)
            .await?;

        // Track items in memory.
        let mut resources = Resources::new();
        Self::workspace_params_insert(workspace_params, &mut resources);
        Self::profile_params_insert(profile_params, &mut resources);

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

impl<'ctx, 'key, PKeys, ProfileSelection, ProfileParamsSelection, FlowParamsSelection>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdSelected,
            WorkspaceParamsSome<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
            ProfileParamsSelection,
            FlowParamsSelection,
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
            workspace_params_file,
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

impl<'ctx, 'key, PKeys, ProfileSelection, WorkspaceParamsSelection, FlowParamsSelection>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdSelected,
            WorkspaceParamsSelection,
            ProfileParamsSome<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
            FlowParamsSelection,
        >,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    /// Merges profile params provided by the caller with the profile params
    /// on disk.
    async fn profile_params_merge(
        &mut self,
        profile_params_file: &ProfileParamsFile,
    ) -> Result<(), Error> {
        let storage = self.workspace.storage();
        let params_deserialized = WorkspaceInitializer::profile_params_deserialize::<
            <PKeys::ProfileParamsKMaybe as KeyMaybe>::Key,
        >(
            storage,
            self.params_type_regs_builder.profile_params_type_reg(),
            profile_params_file,
        )
        .await?;
        match (
            self.scope_builder.profile_params_selection.0.as_mut(),
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
                self.scope_builder.profile_params_selection.0 = Some(params_deserialized)
            }
            (Some(_), None) => {}
            (None, None) => {}
        }

        Ok(())
    }
}
