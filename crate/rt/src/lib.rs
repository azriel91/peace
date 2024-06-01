//! Runtime logic for the peace automation library.

/// Maximum number of steps to execute simultaneously.
///
/// 64 is arbitrarily chosen, as there is not enough data to inform us what a
/// suitable number is.
pub const BUFFERED_FUTURES_MAX: usize = 64;

pub mod cmd_blocks;
pub mod cmds;
