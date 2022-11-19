//! Data model for the peace automation framework.

// Re-exports
pub use fn_graph::{self, resman, DataAccess, DataAccessDyn, Resources, TypeIds, R, W};
pub use peace_data_derive::Data;

pub use crate::{data::Data, r_maybe::RMaybe, w_maybe::WMaybe};

mod data;
mod r_maybe;
mod w_maybe;
