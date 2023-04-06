//! Markers for `State`s inserted into `Resources`.
//!
//! For `SingleProfileSingleFlow` commands, `Current<ItemSpec::State>(None)` and
//! `Desired<ItemSpec::State>(None)` are inserted into `Resources` when the
//! command context is built, and automatically mutated to `Some` when either
//! `StateCurrentFn` or `StateDesiredFn` is executed.

pub use self::{apply_dry::ApplyDry, clean::Clean, current::Current, desired::Desired};

mod apply_dry;
mod clean;
mod current;
mod desired;
