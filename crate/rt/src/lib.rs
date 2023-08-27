//! Runtime logic for the peace automation library.

/// Maximum number of items to execute simultaneously.
///
/// 64 is arbitrarily chosen, as there is not enough data to inform us what a
/// suitable number is.
pub const BUFFERED_FUTURES_MAX: usize = 64;

/// Maximum number of progress messages to buffer.
///
/// TODO: Remove this. This is duplicated with `peace_cmd_rt`.
#[cfg(feature = "output_progress")]
pub const PROGRESS_COUNT_MAX: usize = 256;

pub mod cmd_blocks;
pub mod cmds;

/// TODO: Remove this. This is duplicated with `peace_cmd_rt`.
#[cfg(feature = "output_progress")]
pub(crate) mod progress;
