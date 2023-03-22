//! Runtime logic for the peace automation library.

/// Maximum number of item specs to execute simultaneously.
///
/// 64 is arbitrarily chosen, as there is not enough data to inform us what a
/// suitable number is.
pub const BUFFERED_FUTURES_MAX: usize = 64;

#[cfg(feature = "output_progress")]
/// Maximum number of progress messages to buffer.
pub const PROGRESS_COUNT_MAX: usize = 256;

pub mod cmds;

#[cfg(feature = "output_progress")]
pub(crate) mod progress;
