use std::path::PathBuf;

#[cfg(feature = "error_reporting")]
use peace::miette;

use peace::{
    cfg::{Profile, ProfileInvalidFmt},
    resources::internal::ProfileParamsFile,
    rt_model::fn_graph::{Edge, WouldCycle},
};

/// Error while managing a web application.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum AppCycleError {
    /// Failed to construct web application download URL.
    #[error("Failed to construct web application download URL.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(app_cycle::app_cycle_url_build),
            help("If the URL is valid, this may be a bug in the example, or the `url` library.")
        )
    )]
    AppCycleUrlBuild {
        /// Computed URL that is attempted to be parsed.
        #[cfg_attr(feature = "error_reporting", source_code)]
        url_candidate: String,
        /// URL parse error.
        #[source]
        error: url::ParseError,
    },
    /// Failed to parse environment type.
    #[error("Failed to parse environment type.")]
    EnvTypeParseError(
        #[source]
        #[from]
        crate::model::EnvTypeParseError,
    ),

    /// Profile directory name is not a valid profile name.
    #[error("Profile directory name is not a valid profile name: {}, path: {}", dir_name, path.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(app_cycle::profile_dir_invalid_name))
    )]
    ProfileDirInvalidName {
        /// Name of the directory attempted to be parsed as a `Profile`.
        dir_name: String,
        /// Path to the profile directory.
        path: PathBuf,
        /// Underlying error,
        error: ProfileInvalidFmt<'static>,
    },

    /// Profile params does not exist.
    #[error("Profile params does not exist for profile: {}, path: {}", profile, profile_params_file.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(app_cycle::profile_params_none),
            help(
                "Run `app_cycle profile init --name {} --type <env_type>`\n\
                to set the environment type.",
                profile
            )
        )
    )]
    ProfileParamsNone {
        /// Profile that has no params file.
        profile: Profile,
        /// Path to the profile params file.
        profile_params_file: ProfileParamsFile,
    },

    /// Environment type does not exist for profile.
    #[error("Environment type does not exist for profile: {}, path: {}", profile, profile_params_file.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(app_cycle::profile_env_type_none),
            help(
                "Run `app_cycle profile init --name {} --type <env_type>`\n\
                to set the environment type.",
                profile
            )
        )
    )]
    ProfileEnvTypeNone {
        /// Profile that has no params file.
        profile: Profile,
        /// Path to the profile params file.
        profile_params_file: ProfileParamsFile,
    },

    // === Native errors === //
    /// Failed to list entries in `PeaceAppDir`.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to list entries in `PeaceAppDir`: {}", peace_app_dir.display())]
    PeaceAppDirRead {
        /// Path to the `PeaceAppDir`.
        peace_app_dir: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    /// Failed to read entry in `PeaceAppDir`.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to read entry in `PeaceAppDir`: {}", peace_app_dir.display())]
    PeaceAppDirEntryRead {
        /// Path to the `PeaceAppDir`.
        peace_app_dir: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    /// Failed to read entry file type in `PeaceAppDir`.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to read entry file type in `PeaceAppDir`: {}", path.display())]
    PeaceAppDirEntryFileTypeRead {
        /// Path to the entry within `PeaceAppDir`.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    // === Item Spec errors === //
    /// A `FileDownload` item spec error occurred.
    #[error("A `FileDownload` item spec error occurred.")]
    PeaceItemSpecFileDownload(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace_item_specs::file_download::FileDownloadError,
    ),
    /// A `TarX` item spec error occurred.
    #[error("A `TarX` item spec error occurred.")]
    PeaceItemSpecTarX(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace_item_specs::tar_x::TarXError,
    ),

    // === Framework errors === //
    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),

    /// A graph `WouldCycle` error occurred.
    #[error("A `peace` runtime error occurred.")]
    WouldCycleError(
        #[source]
        #[from]
        WouldCycle<Edge>,
    ),

    // === Scaffolding errors === //
    #[error("Failed to initialize tokio runtime.")]
    TokioRuntimeInit(#[source] std::io::Error),
}
