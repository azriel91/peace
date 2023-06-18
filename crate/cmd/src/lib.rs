#![cfg_attr(coverage_nightly, feature(no_coverage))]

//! Command structure for the Peace framework.

pub use crate::cmd_independence::CmdIndependence;

pub mod ctx;
pub mod scopes;

mod cmd_independence;
