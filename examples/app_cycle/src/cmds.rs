//! User facing commands.
//!
//! These should be callable from different endpoints, e.g. CLI, or WASM.
//! Each endpoint is responsible for receiving the parameters from the user.
//!
//! The `*Cmd` types map between the parameters received from users, to each
//! `ItemSpec`'s params type.

pub use self::{
    app_init_cmd::AppInitCmd, cmd_ctx_builder::params_augment, profile_list_cmd::ProfileListCmd,
    profile_show_cmd::ProfileShowCmd,
};

mod app_init_cmd;
mod cmd_ctx_builder;
mod profile_list_cmd;
mod profile_show_cmd;
