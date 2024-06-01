//! Accessors to fetch data from `Resources`.
//!
//! * `R<'_, T>`: Immutable access to a `T` resource.
//! * `W<'_, T>`: Mutable access to a `T` resource.
//! * `RMaybe<'_, T>`: Immutable access to a `T` resource, if it has been
//!   inserted.
//! * `WMaybe<'_, T>`: Mutable access to a `T` resource, if it has been
//!   inserted.
//! * `ROpt<'_, T>`: Immutable access to an `Option<T>` resource.
//! * `WOpt<'_, T>`: Mutable access to an `Option<T>` resource.
//!
//! Notably if you want to insert a resource during step execution, you
//! should use `WOpt` instead of `WMaybe`, and correspondingly read it using
//! `ROpt`.
pub use self::{r_maybe::RMaybe, r_opt::ROpt, w_maybe::WMaybe, w_opt::WOpt};
pub use fn_graph::{R, W};

mod r_maybe;
mod r_opt;
mod w_maybe;
mod w_opt;
