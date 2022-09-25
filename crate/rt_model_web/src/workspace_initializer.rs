use std::{iter, marker::PhantomData, path::Path};

use peace_resources::internal::{FlowInitFile, ProfileInitFile, WorkspaceDirs, WorkspaceInitFile};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, WebStorage};

/// Logic to create peace directories and reads/writes initialization params.
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
#[derive(Debug)]
pub struct WorkspaceInitializer<WorkspaceInit, ProfileInit, FlowInit>(
    PhantomData<(WorkspaceInit, ProfileInit, FlowInit)>,
);

impl<WorkspaceInit, ProfileInit, FlowInit>
    WorkspaceInitializer<WorkspaceInit, ProfileInit, FlowInit>
where
    WorkspaceInit: Serialize + DeserializeOwned + Send + Sync + 'static,
    ProfileInit: Serialize + DeserializeOwned + Send + Sync + 'static,
    FlowInit: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Creates directories used by the peace framework.
    ///
    /// For web storage, this sets empty values at directory paths to emulate
    /// the native storage.
    pub async fn dirs_initialize(storage: &WebStorage, dirs: &WorkspaceDirs) -> Result<(), Error> {
        let dirs = iter::once(AsRef::<Path>::as_ref(dirs.workspace_dir()))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.peace_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.profile_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(
                dirs.profile_history_dir(),
            )))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.flow_dir())));

        storage.iter_with_storage(dirs, |storage, dir| {
            let dir_str = dir.to_string_lossy();
            let value = "";
            storage
                .set_item(dir_str.as_ref(), value)
                .map_err(|js_value| (dir_str.to_string(), value.to_string(), js_value))
        })?;

        Ok(())
    }

    pub async fn workspace_init_params_serialize(
        storage: &WebStorage,
        workspace_init_params: &WorkspaceInit,
        workspace_init_file: &WorkspaceInitFile,
    ) -> Result<(), Error> {
        storage
            .serialized_write(
                &workspace_init_file.to_string_lossy(),
                workspace_init_params,
                Error::WorkspaceInitParamsSerialize,
            )
            .await
    }

    pub async fn workspace_init_params_deserialize(
        storage: &WebStorage,
        workspace_init_file: &WorkspaceInitFile,
    ) -> Result<Option<WorkspaceInit>, Error> {
        storage
            .serialized_read_opt(
                &workspace_init_file.to_string_lossy(),
                Error::FlowInitParamsDeserialize,
            )
            .await
    }

    pub async fn profile_init_params_serialize(
        storage: &WebStorage,
        profile_init_params: &ProfileInit,
        profile_init_file: &ProfileInitFile,
    ) -> Result<(), Error> {
        storage
            .serialized_write(
                &profile_init_file.to_string_lossy(),
                profile_init_params,
                Error::ProfileInitParamsSerialize,
            )
            .await
    }

    pub async fn profile_init_params_deserialize(
        storage: &WebStorage,
        profile_init_file: &ProfileInitFile,
    ) -> Result<Option<ProfileInit>, Error> {
        storage
            .serialized_read_opt(
                &profile_init_file.to_string_lossy(),
                Error::FlowInitParamsDeserialize,
            )
            .await
    }

    pub async fn flow_init_params_serialize(
        storage: &WebStorage,
        flow_init_params: &FlowInit,
        flow_init_file: &FlowInitFile,
    ) -> Result<(), Error> {
        storage
            .serialized_write(
                &flow_init_file.to_string_lossy(),
                flow_init_params,
                Error::FlowInitParamsSerialize,
            )
            .await
    }

    pub async fn flow_init_params_deserialize(
        storage: &WebStorage,
        flow_init_file: &FlowInitFile,
    ) -> Result<Option<FlowInit>, Error> {
        storage
            .serialized_read_opt(
                &flow_init_file.to_string_lossy(),
                Error::FlowInitParamsDeserialize,
            )
            .await
    }
}
