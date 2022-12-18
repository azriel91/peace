//! User facing commands.
//!
//! These should be callable from different endpoints, e.g. CLI, or WASM.
//! Each endpoint is responsible for receiving the parameters from the user.

pub use self::app_init_cmd::AppInitCmd;

mod app_init_cmd;
