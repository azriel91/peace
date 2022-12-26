use std::{fmt::Debug, hash::Hash, iter, path::Path};

use peace_resources::{
    internal::{FlowInitFile, ProfileInitFile, WorkspaceDirs, WorkspaceInitFile},
    type_reg::untagged::TypeReg,
};
use peace_rt_model_core::cmd_context_params::{FlowParams, ProfileParams, WorkspaceParams};
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
pub struct WorkspaceInitializer;

impl WorkspaceInitializer {
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

        storage.set_items(dirs.map(|dir| (dir, "")))?;

        Ok(())
    }

    pub async fn workspace_params_serialize<K>(
        storage: &WebStorage,
        workspace_params: &WorkspaceParams<K>,
        workspace_init_file: &WorkspaceInitFile,
    ) -> Result<(), Error>
    where
        K: Eq + Hash + Serialize + Send + Sync,
    {
        storage
            .serialized_write(
                workspace_init_file,
                workspace_params,
                Error::WorkspaceInitParamsSerialize,
            )
            .await
    }

    pub async fn workspace_params_deserialize<K>(
        storage: &WebStorage,
        type_reg: &TypeReg<K>,
        workspace_init_file: &WorkspaceInitFile,
    ) -> Result<Option<WorkspaceParams<K>>, Error>
    where
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(
                type_reg,
                workspace_init_file,
                Error::WorkspaceInitParamsDeserialize,
            )
            .await
    }

    pub async fn profile_params_serialize<K>(
        storage: &WebStorage,
        profile_params: &ProfileParams<K>,
        profile_init_file: &ProfileInitFile,
    ) -> Result<(), Error>
    where
        K: Eq + Hash + Serialize + Send + Sync,
    {
        storage
            .serialized_write(
                profile_init_file,
                profile_params,
                Error::ProfileInitParamsSerialize,
            )
            .await
    }

    pub async fn profile_params_deserialize<K>(
        storage: &WebStorage,
        type_reg: &TypeReg<K>,
        profile_init_file: &ProfileInitFile,
    ) -> Result<Option<ProfileParams<K>>, Error>
    where
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(
                type_reg,
                profile_init_file,
                Error::ProfileInitParamsDeserialize,
            )
            .await
    }

    pub async fn flow_params_serialize<K>(
        storage: &WebStorage,
        flow_params: &FlowParams<K>,
        flow_init_file: &FlowInitFile,
    ) -> Result<(), Error>
    where
        K: Eq + Hash + Serialize + Send + Sync,
    {
        storage
            .serialized_write(flow_init_file, flow_params, Error::FlowInitParamsSerialize)
            .await
    }

    pub async fn flow_params_deserialize<K>(
        storage: &WebStorage,
        type_reg: &TypeReg<K>,
        flow_init_file: &FlowInitFile,
    ) -> Result<Option<FlowParams<K>>, Error>
    where
        K: Debug + Eq + Hash + DeserializeOwned + Send + Sync,
    {
        storage
            .serialized_typemap_read_opt(type_reg, flow_init_file, Error::FlowInitParamsDeserialize)
            .await
    }
}
