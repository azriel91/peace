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

pub mod cmd_context_params;
pub mod output;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        pub use crate::cmd_progress_tracker::CmdProgressTracker;

        mod cmd_progress_tracker;
    }
}
