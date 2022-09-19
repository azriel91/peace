//! Core runtime traits for the peace automation framework.
//!
//! These types are in this crate so that the `rt_model_native` and
//! `rt_model_web` crates are able to reference them and either use or provide
//! default implementations.

// Re-exports
pub use async_trait::async_trait;

pub use crate::output_write::OutputWrite;

mod output_write;
