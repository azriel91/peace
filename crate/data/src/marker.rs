//! Markers for `State`s inserted into `Resources`.
//!
//! For `SingleProfileSingleFlow` commands, `Current<Step::State>(None)` and
//! `Goal<Step::State>(None)` are inserted into `Resources` when the
//! command context is built, and automatically mutated to `Some` when either
//! `Step::state_current` or `Step::state_goal` is executed.

// Corresponds to variants in `crate/params/src/value_resolution_mode.rs`.
// Remember to update there when updating here.
pub use self::{apply_dry::ApplyDry, clean::Clean, current::Current, goal::Goal};

mod apply_dry;
mod clean;
mod current;
mod goal;
