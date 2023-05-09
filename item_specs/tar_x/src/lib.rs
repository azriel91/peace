//! Manages extracting a tar file for the peace framework

pub use crate::{
    file_metadata::FileMetadata,
    file_metadatas::FileMetadatas,
    tar_x_apply_fns::TarXApplyFns,
    tar_x_data::TarXData,
    tar_x_error::TarXError,
    tar_x_item_spec::TarXItemSpec,
    tar_x_params::{TarXParams, TarXParamsFieldWise, TarXParamsPartial},
    tar_x_state_current_fn::TarXStateCurrentFn,
    tar_x_state_desired_fn::TarXStateDesiredFn,
    tar_x_state_diff::TarXStateDiff,
    tar_x_state_diff_fn::TarXStateDiffFn,
};

mod file_metadata;
mod file_metadatas;
mod tar_x_apply_fns;
mod tar_x_data;
mod tar_x_error;
mod tar_x_item_spec;
mod tar_x_params;
mod tar_x_state_current_fn;
mod tar_x_state_desired_fn;
mod tar_x_state_diff;
mod tar_x_state_diff_fn;

#[cfg(not(target_arch = "wasm32"))]
mod native;
