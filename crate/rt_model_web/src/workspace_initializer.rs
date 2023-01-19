use std::{fmt::Debug, hash::Hash, path::Path};

use peace_resources::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceDirs, WorkspaceParamsFile},
    type_reg::untagged::TypeReg,
};
use peace_rt_model_core::cmd_context_params::{FlowParams, ProfileParams, WorkspaceParams};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, Storage};

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
    ///
    /// For web storage, this sets empty values at directory paths to emulate
    /// the native storage.
    pub async fn dirs_initialize(storage: &Storage, dirs: &WorkspaceDirs) -> Result<(), Error> {
        let dirs = [
            AsRef::<Path>::as_ref(dirs.workspace_dir()),
            AsRef::<Path>::as_ref(dirs.peace_dir()),
            AsRef::<Path>::as_ref(dirs.peace_app_dir()),
            AsRef::<Path>::as_ref(dirs.profile_dir()),
            AsRef::<Path>::as_ref(dirs.profile_history_dir()),
            AsRef::<Path>::as_ref(dirs.flow_dir()),
        ];

        storage.set_items(dirs.iter().map(|dir| (*dir, "")))?;

        Ok(())
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
                workspace_params_file,
                workspace_params,
                Error::WorkspaceParamsSerialize,
            )
            .await
    }

    pub async fn workspace_params_deserialize<K>(
        storage: &Storage,
        type_reg: &TypeReg<K>,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<Option<WorkspaceParams<K>>, Error>
    where
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(
                type_reg,
                workspace_params_file,
                Error::WorkspaceParamsDeserialize,
            )
            .await
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
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(
                type_reg,
                profile_params_file,
                Error::ProfileParamsDeserialize,
            )
            .await
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
            .serialized_write(flow_params_file, flow_params, Error::FlowParamsSerialize)
            .await
    }

    pub async fn flow_params_deserialize<K>(
        storage: &Storage,
        type_reg: &TypeReg<K>,
        flow_params_file: &FlowParamsFile,
    ) -> Result<Option<FlowParams<K>>, Error>
    where
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(type_reg, flow_params_file, Error::FlowParamsDeserialize)
            .await
    }
}
