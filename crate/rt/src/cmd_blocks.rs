//! Blocks of logic that run one [`Item`] function
//!
//! [`Item`]: peace_cfg::Item

pub use self::{
    diff_cmd_block::DiffCmdBlock, states_current_read_cmd_block::StatesCurrentReadCmdBlock,
    states_discover_cmd_block::StatesDiscoverCmdBlock,
    states_goal_read_cmd_block::StatesGoalReadCmdBlock,
};

mod diff_cmd_block;
mod states_current_read_cmd_block;
mod states_discover_cmd_block;
mod states_goal_read_cmd_block;
