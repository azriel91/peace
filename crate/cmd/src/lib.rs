#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! Command structure for the Peace framework.

pub use crate::cmd_independence::CmdIndependence;

pub mod ctx;
pub mod scopes;

mod cmd_independence;
