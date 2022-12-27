use std::{ffi::OsString, path::PathBuf, sync::Mutex};

use peace_core::ItemSpecId;
use peace_resources::paths::WorkspaceDir;

// Remember to add common variants to `rt_model_web/src/error.rs`.

/// Peace runtime errors.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to serialize error.
    #[error("Failed to serialize error.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::error_serialize))
    )]
    ErrorSerialize(#[source] serde_yaml::Error),

    /// Failed to deserialize states.
    #[error("Failed to deserialize states.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::states_deserialize))
    )]
    StatesDeserialize {
        /// Source text to be deserialized.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        states_file_source: miette::NamedSource,
        /// Offset within the source text that the error occurred.
        #[cfg(feature = "error_reporting")]
        #[label("{}", error_message)]
        error_span: Option<miette::SourceOffset>,
        /// Message explaining the error.
        #[cfg(feature = "error_reporting")]
        error_message: String,
        /// Offset within the source text surrounding the error.
        #[cfg(feature = "error_reporting")]
        #[label]
        context_span: Option<miette::SourceOffset>,
        /// Underlying error.
        #[source]
        error: serde_yaml::Error,
    },

    /// Failed to serialize states.
    #[error("Failed to serialize states.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::states_serialize))
    )]
    StatesSerialize(#[source] serde_yaml::Error),

    /// Failed to discover current state for a particular item spec.
    ///
    /// This happens when current state is discovered during an `EnsureCmd`
    /// execution -- as the current state is expected to be discovered for all
    /// item specs as their predecessors (dependencies) are meant to exist.
    ///
    /// This does *not* happen during a `StateCurrentDiscoverCmd` execution --
    /// i.e. it is okay for a `StateCurrentFnSpec` to return `Ok(None)` for
    /// inspecting the state of a file on a remote server, if the server doesn't
    /// exist.
    #[error("Failed to discover current state for item spec: `{item_spec_id}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::state_current_discover_none),
            help(
                "This is a bug in the automation for `{item_spec_id}`.\n\
                It should return an error explaining why the current state could not be discovered,\n\
                instead of `None`."
            )
        )
    )]
    StateCurrentDiscoverNone {
        /// ID of the item spec whose state failed to be discovered.
        item_spec_id: ItemSpecId,
    },

    /// Current states have not been discovered.
    ///
    /// This is returned when `StatesSavedFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Current states have not been discovered.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::states_current_discover_required),
            help("Ensure that `StatesDiscoverCmd` or `StatesCurrentDiscoverCmd` has been called.")
        )
    )]
    StatesCurrentDiscoverRequired,

    /// Failed to discover desired state for a particular item spec.
    ///
    /// This happens when desired state is discovered during an `EnsureCmd`
    /// execution -- as the desired state is expected to be discovered for all
    /// item specs as their predecessors (dependencies) are meant to exist.
    ///
    /// This does *not* happen during a `StateDesiredDiscoverCmd` execution --
    /// i.e. it is okay for a `StateDesiredFnSpec` to return `Ok(None)` for
    /// inspecting the state of a file to transfer, if the file needs to be
    /// created.
    #[error("Failed to discover desired state for item spec: `{item_spec_id}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::state_current_discover_none),
            help(
                "This is a bug in the automation for `{item_spec_id}`.\n\
                It should return an error explaining why the desired state could not be discovered,\n\
                instead of `None`."
            )
        )
    )]
    StateDesiredDiscoverNone {
        /// ID of the item spec whose state failed to be discovered.
        item_spec_id: ItemSpecId,
    },

    /// Desired states have not been written to disk.
    ///
    /// This is returned when `StatesDesiredFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Desired states have not been written to disk.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::states_desired_discover_required),
            help("Ensure that `StatesDiscoverCmd` or `StatesDesiredDiscoverCmd` has been called.")
        )
    )]
    StatesDesiredDiscoverRequired,

    /// Failed to serialize state diffs.
    #[error("Failed to serialize state diffs.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::state_diffs_serialize))
    )]
    StateDiffsSerialize(#[source] serde_yaml::Error),

    /// Failed to serialize error as JSON.
    #[cfg(feature = "output_json")]
    #[error("Failed to serialize error as JSON.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::error_serialize_json))
    )]
    ErrorSerializeJson(#[source] serde_json::Error),

    /// Failed to serialize states as JSON.
    #[cfg(feature = "output_json")]
    #[error("Failed to serialize states as JSON.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::states_current_serialize_json))
    )]
    StatesSerializeJson(#[source] serde_json::Error),

    /// Failed to serialize state diffs as JSON.
    #[cfg(feature = "output_json")]
    #[error("Failed to serialize state diffs as JSON.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::state_diffs_serialize_json))
    )]
    StateDiffsSerializeJson(#[source] serde_json::Error),

    /// Failed to serialize workspace init params.
    #[error("Failed to serialize workspace init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::workspace_init_params_serialize))
    )]
    WorkspaceParamsSerialize(#[source] serde_yaml::Error),

    /// Failed to deserialize workspace init params.
    #[error("Failed to serialize workspace init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::workspace_init_params_deserialize))
    )]
    WorkspaceParamsDeserialize(#[source] serde_yaml::Error),

    /// Failed to serialize profile init params.
    #[error("Failed to serialize profile init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::profile_init_params_serialize))
    )]
    ProfileParamsSerialize(#[source] serde_yaml::Error),

    /// Failed to deserialize profile init params.
    #[error("Failed to serialize profile init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::profile_init_params_deserialize))
    )]
    ProfileParamsDeserialize(#[source] serde_yaml::Error),

    /// Failed to serialize flow init params.
    #[error("Failed to serialize flow init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::flow_init_params_serialize))
    )]
    FlowParamsSerialize(#[source] serde_yaml::Error),

    /// Failed to deserialize flow init params.
    #[error("Failed to serialize flow init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::flow_init_params_deserialize))
    )]
    FlowParamsDeserialize(#[source] serde_yaml::Error),

    /// Item does not exist in storage.
    #[error("Item does not exist in storage: `{}`.", path.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::item_not_exists))
    )]
    ItemNotExists {
        /// Path to the file.
        path: PathBuf,
    },

    // Native errors.
    #[error("Failed to set current dir to workspace directory: `{}`", workspace_dir.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::current_dir_set))
    )]
    CurrentDirSet {
        /// The workspace directory.
        workspace_dir: WorkspaceDir,
        /// Underlying IO error
        #[source]
        error: std::io::Error,
    },

    /// Failed to create file for writing.
    #[error("Failed to create file for writing: `{path}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::file_create))
    )]
    FileCreate {
        /// Path to the file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    /// Failed to open file for reading.
    #[error("Failed to open file for reading: `{path}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::file_open))
    )]
    FileOpen {
        /// Path to the file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    /// Failed to read from file.
    #[error("Failed to read from file: `{path}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::file_read))
    )]
    FileRead {
        /// Path to the file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    /// Failed to write to file.
    #[error("Failed to write to file: `{path}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::file_write))
    )]
    FileWrite {
        /// Path to the file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    /// Failed to write to stdout.
    #[error("Failed to write to stdout.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::stdout_write))
    )]
    StdoutWrite(#[source] std::io::Error),

    /// Storage synchronous thread failed to be joined.
    ///
    /// This variant is used for thread spawning errors for both reads and
    /// writes.
    #[error("Storage synchronous thread failed to be joined.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::storage_sync_thread_spawn))
    )]
    StorageSyncThreadSpawn(#[source] std::io::Error),

    /// Storage synchronous thread failed to be joined.
    ///
    /// This variant is used for thread spawning errors for both reads and
    /// writes.
    ///
    /// Note: The underlying thread join error does not implement
    /// `std::error::Error`. See
    /// <https://doc.rust-lang.org/std/thread/type.Result.html>.
    ///
    /// The `Mutex` is needed to allow `Error` to be `Sync`.
    #[error("Storage synchronous thread failed to be joined.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::storage_sync_thread_join))
    )]
    StorageSyncThreadJoin(Mutex<Box<dyn std::any::Any + Send + 'static>>),

    /// Failed to read current directory to discover workspace directory.
    #[error("Failed to read current directory to discover workspace directory.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::working_dir_read))
    )]
    WorkingDirRead(#[source] std::io::Error),

    /// Failed to create a workspace directory.
    #[error("Failed to create workspace directory: `{path}`.", path = path.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::workspace_dir_create))
    )]
    WorkspaceDirCreate {
        /// The directory that was attempted to be created.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    /// Failed to determine workspace directory.
    #[error(
        "Failed to determine workspace directory as could not find `{file_name}` \
            in `{working_dir}` or any parent directories.",
        file_name = file_name.to_string_lossy(),
        working_dir = working_dir.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_native::workspace_file_not_found))
    )]
    WorkspaceFileNotFound {
        /// Beginning directory of traversal.
        working_dir: PathBuf,
        /// File or directory name searched for.
        file_name: OsString,
    },
}
