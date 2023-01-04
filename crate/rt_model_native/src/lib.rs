//! Runtime data types for the peace automation framework (native).
//!
//! Consumers should depend on the `peace_rt_model` crate, which re-exports
//! same-named types, depending on whether a native or WASM target is used.

// Re-exports
pub use tokio_util::io::SyncIoBridge;

pub use crate::{
    cli_output::CliOutput, error::Error, native_storage::NativeStorage, workspace::Workspace,
    workspace_dirs_builder::WorkspaceDirsBuilder, workspace_initializer::WorkspaceInitializer,
    workspace_spec::WorkspaceSpec,
};

#[cfg(feature = "output_progress")]
pub(crate) use crate::cli_progress_format::CliProgressFormatChosen;
#[cfg(feature = "output_progress")]
pub use crate::{
    cli_progress_format::CliProgressFormat,
    cli_progress_format_parse_error::CliProgressFormatParseError,
};

mod cli_output;
mod error;
mod native_storage;
mod workspace;
mod workspace_dirs_builder;
mod workspace_initializer;
mod workspace_spec;

#[cfg(feature = "output_progress")]
mod cli_progress_format;
#[cfg(feature = "output_progress")]
mod cli_progress_format_parse_error;
