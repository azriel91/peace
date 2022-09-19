//! Commands to provide user-facing output.
//!
//! Commands directly underneath this module write to the `OutputWrite`
//! provided in the [`CmdContext`]. Commands at this level are intended to
//! provide the information requested to the user; errors should inform the user
//! of any steps that have to be run before the command is able to fulfill the
//! information request.
//!
//! [`CmdContext`]: crate::CmdContext

pub use self::{
    diff_cmd::DiffCmd, ensure_cmd::EnsureCmd, states_current_display_cmd::StatesCurrentDisplayCmd,
    states_desired_display_cmd::StatesDesiredDisplayCmd, states_discover_cmd::StatesDiscoverCmd,
};

pub mod sub;

mod diff_cmd;
mod ensure_cmd;
mod states_current_display_cmd;
mod states_desired_display_cmd;
mod states_discover_cmd;
