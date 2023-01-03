//! Core runtime traits for the peace automation framework.
//!
//! These types are in this crate so that the `rt_model_native` and
//! `rt_model_web` crates are able to reference them and either use or provide
//! default implementations.

// Re-exports
pub use async_trait::async_trait;
pub use indicatif;
// Keep in sync with `peace_cfg`.
pub use tokio::sync::mpsc::{
    error::{SendError, TryRecvError, TrySendError},
    Receiver, Sender,
};

pub use crate::{
    output_format::OutputFormat, output_format_parse_error::OutputFormatParseError,
    output_write::OutputWrite, progress_output_write::ProgressOutputWrite,
};

pub mod cmd_context_params;

mod output_format;
mod output_format_parse_error;
mod output_write;
mod progress_output_write;
