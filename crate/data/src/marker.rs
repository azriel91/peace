//! Markers for `State`s inserted into `Resources`.
//!
//! For `SingleProfileSingleFlow` commands, `Current<Item::State>(None)` and
//! `Desired<Item::State>(None)` are inserted into `Resources` when the
//! command context is built, and automatically mutated to `Some` when either
//! `Item::state_current` or `Item::state_desired` is executed.

// Corresponds to variants in `crate/params/src/value_resolution_mode.rs`.
// Remember to update there when updating here.
pub use self::{apply_dry::ApplyDry, clean::Clean, current::Current, desired::Desired};

mod apply_dry;
mod clean;
mod current;
mod desired;
