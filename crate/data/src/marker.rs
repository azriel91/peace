//! Markers for `State`s inserted into `Resources`.
//!
//! For `SingleProfileSingleFlow` commands, `Current<ItemSpec::State>(None)` and
//! `Desired<ItemSpec::State>(None)` are inserted into `Resources` when the
//! command context is built, and automatically mutated to `Some` when either
//! `StateCurrentFnSpec` or `StateDesiredFnSpec` is executed.

pub use self::{current::Current, desired::Desired};

mod current;
mod desired;
