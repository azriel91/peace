//! Runtime logic for the peace automation library.

pub use crate::{
    commands::{StatusCommand, StatusDesiredCommand},
    job_runner::JobRunner,
};

mod commands;
mod job_runner;
