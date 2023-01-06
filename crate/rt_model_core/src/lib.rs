//! Core runtime traits for the peace automation framework.
//!
//! These types are in this crate so that the `rt_model_native` and
//! `rt_model_web` crates are able to reference them and either use or provide
//! default implementations.

// Re-exports
pub use async_trait::async_trait;
pub use indicatif;
#[cfg(feature = "output_progress")]
pub use peace_core::progress::ProgressUpdate;
pub use rt_map;

pub use crate::{
    output_format::OutputFormat, output_format_parse_error::OutputFormatParseError,
    output_write::OutputWrite,
};

pub mod cmd_context_params;

mod output_format;
mod output_format_parse_error;
mod output_write;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        pub use crate::cmd_progress_tracker::CmdProgressTracker;

        mod cmd_progress_tracker;
    }
}
