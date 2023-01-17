//! 🕊️ peace -- zero stress automation

// Re-exports so consumers don't need to depend on crates individually.
#[cfg(feature = "error_reporting")]
pub use miette;

pub use peace_cfg as cfg;
pub use peace_data as data;
pub use peace_diff as diff;
pub use peace_resources as resources;
pub use peace_rt as rt;
pub use peace_rt_model as rt_model;

#[cfg(all(target_arch = "wasm32", feature = "output_progress"))]
compile_error!(
    r#"The `"output_progress"` feature does not work on WASM, pending support from `indicatif`.
See <https://github.com/console-rs/indicatif/issues/513>."#
);
