use std::{fmt::Debug, hash::Hash, path::Path};

use futures::{stream, StreamExt, TryStreamExt};

use peace_resource_rt::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    type_reg::untagged::{TypeMapOpt, TypeReg},
};
use peace_rt_model_core::{
    params::{FlowParams, ProfileParams, WorkspaceParams},
    Error, NativeError,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::Storage;

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
    pub async fn dirs_create<'f, I>(dirs: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = &'f Path>,
    {
        stream::iter(dirs)
            .map(Result::<_, Error>::Ok)
            .try_for_each(|dir| async move {
                tokio::fs::create_dir_all(dir).await.map_err(|error| {
                    let path = dir.to_path_buf();
                    Error::Native(NativeError::WorkspaceDirCreate { path, error })
                })
            })
            .await
    }

    pub async fn workspace_params_serialize<K>(
        storage: &Storage,
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

    // TODO: for every variant in `K`, we need to deserialize it
    // Also do this for profile_params and flow_params
    //
    // The TypeReg doesn't know what the Value type is.
    // *that*s why we had the ws_and_profile_params augment in envman.

    pub async fn workspace_params_deserialize<K>(
        storage: &Storage,
        type_reg: &TypeReg<K>,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<Option<WorkspaceParams<K>>, Error>
    where
        K: Clone + Debug + Eq + Hash + DeserializeOwned + Send + Sync + 'static,
    {
        storage
            .serialized_typemap_read_opt(
                "workspace_params_deserialize".to_string(),
                type_reg,
                workspace_params_file,
                Error::WorkspaceParamsDeserialize,
            )
            .await
            .map(|type_map_opt| {
                type_map_opt
                    .map(TypeMapOpt::into_type_map)
                    .map(WorkspaceParams::from)
            })
    }

    pub async fn profile_params_serialize<K>(
        storage: &Storage,
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
        storage: &Storage,
        type_reg: &TypeReg<K>,
        profile_params_file: &ProfileParamsFile,
    ) -> Result<Option<ProfileParams<K>>, Error>
    where
        K: Clone + Debug + Eq + Hash + DeserializeOwned + Send + Sync + 'static,
    {
        storage
            .serialized_typemap_read_opt(
                "profile_params_deserialize".to_string(),
                type_reg,
                profile_params_file,
                Error::ProfileParamsDeserialize,
            )
            .await
            .map(|type_map_opt| {
                type_map_opt
                    .map(TypeMapOpt::into_type_map)
                    .map(ProfileParams::from)
            })
    }

    pub async fn flow_params_serialize<K>(
        storage: &Storage,
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
        storage: &Storage,
        type_reg: &TypeReg<K>,
        flow_params_file: &FlowParamsFile,
    ) -> Result<Option<FlowParams<K>>, Error>
    where
        K: Clone + Debug + Eq + Hash + DeserializeOwned + Send + Sync + 'static,
    {
        storage
            .serialized_typemap_read_opt(
                "flow_params_deserialize".to_string(),
                type_reg,
                flow_params_file,
                Error::FlowParamsDeserialize,
            )
            .await
            .map(|type_map_opt| {
                type_map_opt
                    .map(TypeMapOpt::into_type_map)
                    .map(FlowParams::from)
            })
    }
}
