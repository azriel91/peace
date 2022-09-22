use std::{iter, path::Path};

use futures::{stream, StreamExt, TryStreamExt};
use peace_core::{FlowId, Profile};
use peace_resources::{
    internal::WorkspaceDirs,
    paths::{FlowDir, PeaceDir, ProfileDir},
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, NativeStorage, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the `peace` tool runs in.
///
/// # Type Parameters
///
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
#[derive(Clone, Debug)]
pub struct Workspace<WorkspaceInit, ProfileInit, FlowInit> {
    /// Convention-based directories in this workspace.
    dirs: WorkspaceDirs,
    /// Identifier or namespace to distinguish execution environments.
    profile: Profile,
    /// Identifier or name of the chosen process flow.
    flow_id: FlowId,
    /// File system storage access.
    storage: NativeStorage,
    /// Workspace initialization parameters.
    workspace_init_params: WorkspaceInit,
    /// Profile initialization parameters.
    profile_init_params: ProfileInit,
    /// Flow initialization parameters.
    flow_init_params: FlowInit,
}

impl Workspace<(), (), ()> {
    /// Prepares a workspace to run commands in.
    ///
    /// This is a convenience constructor if your workspace, profile, and flow
    /// do not need any initialization parameters.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    /// * `profile`: The profile / namespace that the execution is flow.
    /// * `flow_id`: ID of the flow that is being executed.
    pub async fn init(
        workspace_spec: WorkspaceSpec,
        profile: Profile,
        flow_id: FlowId,
    ) -> Result<Self, Error> {
        Self::init_with_params(workspace_spec, profile, flow_id, (), (), ()).await
    }
}

impl<WorkspaceInit, ProfileInit, FlowInit> Workspace<WorkspaceInit, ProfileInit, FlowInit>
where
    WorkspaceInit: Serialize + DeserializeOwned + Send + Sync,
    ProfileInit: Serialize + DeserializeOwned + Send + Sync,
    FlowInit: Serialize + DeserializeOwned + Send + Sync,
{
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    /// * `profile`: The profile / namespace that the execution is flow.
    /// * `flow_id`: ID of the flow that is being executed.
    /// * `workspace_init_params`: Initialization parameters for the workspace.
    /// * `profile_init_params`: Initialization parameters for the profile.
    /// * `flow_init_params`: Initialization parameters for the flow.
    pub async fn init_with_params(
        workspace_spec: WorkspaceSpec,
        profile: Profile,
        flow_id: FlowId,
        workspace_init_params: WorkspaceInit,
        profile_init_params: ProfileInit,
        flow_init_params: FlowInit,
    ) -> Result<Self, Error> {
        let dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile, &flow_id)?;
        let storage = NativeStorage;

        Self::dirs_initialize(&dirs).await?;
        Self::workspace_init_params_serialize(&storage, &workspace_init_params, dirs.peace_dir())
            .await?;
        Self::profile_init_params_serialize(&storage, &profile_init_params, dirs.profile_dir())
            .await?;
        Self::flow_init_params_serialize(&storage, &flow_init_params, dirs.flow_dir()).await?;

        Ok(Self {
            dirs,
            profile,
            flow_id,
            storage,
            workspace_init_params,
            profile_init_params,
            flow_init_params,
        })
    }

    /// Prepares a workspace to run commands in.
    ///
    /// Initialization parameters must already exist in the `PeaceDir`.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    /// * `profile`: The profile / namespace that the execution is flow.
    /// * `flow_id`: ID of the flow that is being executed.
    pub async fn init_from_storage(
        workspace_spec: WorkspaceSpec,
        profile: Profile,
        flow_id: FlowId,
    ) -> Result<Self, Error> {
        let dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile, &flow_id)?;
        let storage = NativeStorage;

        Self::dirs_initialize(&dirs).await?;
        let workspace_init_params =
            Self::workspace_init_params_deserialize(&storage, dirs.peace_dir()).await?;
        let profile_init_params =
            Self::profile_init_params_deserialize(&storage, dirs.profile_dir()).await?;
        let flow_init_params =
            Self::flow_init_params_deserialize(&storage, dirs.flow_dir()).await?;

        Ok(Self {
            dirs,
            profile,
            flow_id,
            storage,
            workspace_init_params,
            profile_init_params,
            flow_init_params,
        })
    }

    async fn dirs_initialize(dirs: &WorkspaceDirs) -> Result<(), Error> {
        let dirs = iter::once(AsRef::<Path>::as_ref(dirs.workspace_dir()))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.peace_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.profile_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(
                dirs.profile_history_dir(),
            )))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.flow_dir())));

        stream::iter(dirs)
            .map(Result::<_, Error>::Ok)
            .try_for_each(|dir| async move {
                tokio::fs::create_dir_all(dir).await.map_err(|error| {
                    let path = dir.to_path_buf();
                    Error::WorkspaceDirCreate { path, error }
                })
            })
            .await
    }

    async fn workspace_init_params_serialize(
        storage: &NativeStorage,
        workspace_init_params: &WorkspaceInit,
        peace_dir: &PeaceDir,
    ) -> Result<(), Error> {
        storage
            .serialized_write(
                "workspace_init_params_serialize".to_string(),
                &peace_dir,
                workspace_init_params,
                Error::WorkspaceInitParamsSerialize,
            )
            .await
    }

    async fn workspace_init_params_deserialize(
        storage: &NativeStorage,
        peace_dir: &PeaceDir,
    ) -> Result<WorkspaceInit, Error> {
        storage
            .serialized_read(
                "workspace_init_params_deserialize".to_string(),
                &peace_dir,
                Error::FlowInitParamsDeserialize,
            )
            .await
    }

    async fn profile_init_params_serialize(
        storage: &NativeStorage,
        profile_init_params: &ProfileInit,
        profile_dir: &ProfileDir,
    ) -> Result<(), Error> {
        storage
            .serialized_write(
                "profile_init_params_serialize".to_string(),
                &profile_dir,
                profile_init_params,
                Error::ProfileInitParamsSerialize,
            )
            .await
    }

    async fn profile_init_params_deserialize(
        storage: &NativeStorage,
        profile_dir: &ProfileDir,
    ) -> Result<ProfileInit, Error> {
        storage
            .serialized_read(
                "profile_init_params_deserialize".to_string(),
                &profile_dir,
                Error::FlowInitParamsDeserialize,
            )
            .await
    }

    async fn flow_init_params_deserialize(
        storage: &NativeStorage,
        flow_dir: &FlowDir,
    ) -> Result<FlowInit, Error> {
        storage
            .serialized_read(
                "flow_init_params_deserialize".to_string(),
                &flow_dir,
                Error::FlowInitParamsDeserialize,
            )
            .await
    }

    async fn flow_init_params_serialize(
        storage: &NativeStorage,
        flow_init_params: &FlowInit,
        flow_dir: &FlowDir,
    ) -> Result<(), Error> {
        storage
            .serialized_write(
                "flow_init_params_serialize".to_string(),
                &flow_dir,
                flow_init_params,
                Error::FlowInitParamsSerialize,
            )
            .await
    }
}

impl<WorkspaceInit, ProfileInit, FlowInit> Workspace<WorkspaceInit, ProfileInit, FlowInit>
where
    WorkspaceInit: Send + Sync,
    ProfileInit: Send + Sync,
    FlowInit: Send + Sync,
{
    /// Returns the underlying data.
    pub fn into_inner(
        self,
    ) -> (
        WorkspaceDirs,
        Profile,
        FlowId,
        NativeStorage,
        WorkspaceInit,
        ProfileInit,
        FlowInit,
    ) {
        let Self {
            dirs,
            profile,
            flow_id,
            storage,
            workspace_init_params,
            profile_init_params,
            flow_init_params,
        } = self;

        (
            dirs,
            profile,
            flow_id,
            storage,
            workspace_init_params,
            profile_init_params,
            flow_init_params,
        )
    }

    /// Returns a reference to the workspace's directories.
    pub fn dirs(&self) -> &WorkspaceDirs {
        &self.dirs
    }

    /// Returns a reference to the workspace's profile.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Returns a reference to the workspace's flow_id.
    pub fn flow_id(&self) -> &FlowId {
        &self.flow_id
    }

    /// Returns a reference to the workspace's storage.
    pub fn storage(&self) -> &NativeStorage {
        &self.storage
    }

    /// Returns a reference to the workspace init params.
    pub fn workspace_init_params(&self) -> &WorkspaceInit {
        &self.workspace_init_params
    }

    /// Returns a reference to the profile init params.
    pub fn profile_init_params(&self) -> &ProfileInit {
        &self.profile_init_params
    }

    /// Returns a reference to the flow init params.
    pub fn flow_init_params(&self) -> &FlowInit {
        &self.flow_init_params
    }
}
