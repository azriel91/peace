//! Data model for the peace automation framework.

pub use fn_graph::{self, resman, DataAccess, DataAccessDyn, Resources, TypeIds, R, W};
pub use peace_data_derive::Data;

pub use crate::{data::Data, data_init::DataInit};

mod data;
mod data_init;
