use std::{fmt::Debug, hash::Hash, iter, path::Path};

use futures::{stream, StreamExt, TryStreamExt};

use peace_resources::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceDirs, WorkspaceParamsFile},
    type_reg::untagged::TypeReg,
};
use peace_rt_model_core::cmd_context_params::{FlowParams, ProfileParams, WorkspaceParams};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, NativeStorage};

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
pub struct WorkspaceInitializer;

impl WorkspaceInitializer {
    /// Creates directories used by the peace framework.
    pub async fn dirs_initialize(dirs: &WorkspaceDirs) -> Result<(), Error> {
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

    pub async fn workspace_params_serialize<K>(
        storage: &NativeStorage,
        workspace_params: &WorkspaceParams<K>,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<(), Error>
    where
        K: Eq + Hash + Serialize + Send + Sync,
    {
        storage
            .serialized_write(
                "workspace_params_serialize".to_string(),
                workspace_params_file,
                workspace_params,
                Error::WorkspaceParamsSerialize,
            )
            .await
    }

    pub async fn workspace_params_deserialize<K>(
        storage: &NativeStorage,
        type_reg: &TypeReg<K>,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<Option<WorkspaceParams<K>>, Error>
    where
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(
                "workspace_params_deserialize".to_string(),
                type_reg,
                workspace_params_file,
                Error::WorkspaceParamsDeserialize,
            )
            .await
    }

    pub async fn profile_params_serialize<K>(
        storage: &NativeStorage,
        profile_params: &ProfileParams<K>,
        profile_params_file: &ProfileParamsFile,
    ) -> Result<(), Error>
    where
        K: Eq + Hash + Serialize + Send + Sync,
    {
        storage
            .serialized_write(
                "profile_params_serialize".to_string(),
                profile_params_file,
                profile_params,
                Error::ProfileParamsSerialize,
            )
            .await
    }

    pub async fn profile_params_deserialize<K>(
        storage: &NativeStorage,
        type_reg: &TypeReg<K>,
        profile_params_file: &ProfileParamsFile,
    ) -> Result<Option<ProfileParams<K>>, Error>
    where
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(
                "profile_params_deserialize".to_string(),
                type_reg,
                profile_params_file,
                Error::ProfileParamsDeserialize,
            )
            .await
    }

    pub async fn flow_params_serialize<K>(
        storage: &NativeStorage,
        flow_params: &FlowParams<K>,
        flow_params_file: &FlowParamsFile,
    ) -> Result<(), Error>
    where
        K: Eq + Hash + Serialize + Send + Sync,
    {
        storage
            .serialized_write(
                "flow_params_serialize".to_string(),
                flow_params_file,
                flow_params,
                Error::FlowParamsSerialize,
            )
            .await
    }

    pub async fn flow_params_deserialize<K>(
        storage: &NativeStorage,
        type_reg: &TypeReg<K>,
        flow_params_file: &FlowParamsFile,
    ) -> Result<Option<FlowParams<K>>, Error>
    where
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(
                "flow_params_deserialize".to_string(),
                type_reg,
                flow_params_file,
                Error::FlowParamsDeserialize,
            )
            .await
    }
}
