//! Information about how this command's logic should be run.
//!
//! These are information used when implementing commands, not necessarily
//! things that developers who *use* commands need to know.

pub use self::cmd_independence::CmdIndependence;

mod cmd_independence;
