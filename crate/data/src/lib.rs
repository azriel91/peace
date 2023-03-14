//! Data model for the peace automation framework.

// Re-exports
pub use fn_graph::{self, resman, DataAccess, DataAccessDyn, Resources, TypeIds};
pub use peace_data_derive::Data;

pub use crate::data::Data;

pub mod accessors;
pub mod marker;

mod data;
