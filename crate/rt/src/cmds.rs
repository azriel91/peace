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
    clean_cmd::CleanCmd,
    cmd_base::CmdBase,
    diff_cmd::{DiffCmd, DiffInfoSpec, DiffStateSpec},
    ensure_cmd::EnsureCmd,
    states_current_read_cmd::StatesCurrentReadCmd,
    states_current_stored_display_cmd::StatesCurrentStoredDisplayCmd,
    states_discover_cmd::StatesDiscoverCmd,
    states_goal_display_cmd::StatesGoalDisplayCmd,
    states_goal_read_cmd::StatesGoalReadCmd,
    sub::ApplyStoredStateSync,
};

pub mod sub;

mod clean_cmd;
mod cmd_base;
mod diff_cmd;
mod ensure_cmd;
mod states_current_read_cmd;
mod states_current_stored_display_cmd;
mod states_discover_cmd;
mod states_goal_display_cmd;
mod states_goal_read_cmd;
