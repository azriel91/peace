//! User facing commands.
//!
//! These should be callable from different endpoints, e.g. CLI, or WASM.
//! Each endpoint is responsible for receiving the parameters from the user.
//!
//! The `*Cmd` types map between the parameters received from users, to each
//! `ItemSpec`'s params type.

pub use self::{app_init_cmd::AppInitCmd, cmd_ctx_builder::CmdCtxBuilder};

mod app_init_cmd;
mod cmd_ctx_builder;
