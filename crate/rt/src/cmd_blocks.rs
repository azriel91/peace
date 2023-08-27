//! Blocks of logic that run one [`Item`] function
//!
//! [`Item`]: peace_cfg::Item

pub use diff_cmd_block::DiffCmdBlock;
pub use states_discover_cmd_block::StatesDiscoverCmdBlock;

mod diff_cmd_block;
mod states_discover_cmd_block;
