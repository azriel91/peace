//! Common logic that is useful as part of a user facing command.
//!
//! Commands in this module don't write to [`OutputWrite`], but the provided
//! logic is useful to build user facing commands.
//!
//! [`OutputWrite`]: peace_rt_model_core::OutputWrite

pub use self::{
    states_current_discover_cmd::StatesCurrentDiscoverCmd,
    states_current_read_cmd::StatesCurrentReadCmd,
    states_desired_discover_cmd::StatesDesiredDiscoverCmd,
    states_desired_read_cmd::StatesDesiredReadCmd,
};

mod states_current_discover_cmd;
mod states_current_read_cmd;
mod states_desired_discover_cmd;
mod states_desired_read_cmd;