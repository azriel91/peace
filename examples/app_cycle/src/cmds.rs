//! User facing commands.
//!
//! These should be callable from different endpoints, e.g. CLI, or WASM.
//! Each endpoint is responsible for receiving the parameters from the user.
//!
//! The `*Cmd` types map between the parameters received from users, to each
//! `ItemSpec`'s params type.

pub use self::{
    cmd_ctx_builder::{
        ws_and_profile_params_augment, ws_params_augment, ws_profile_and_flow_params_augment,
    },
    env_status_cmd::EnvStatusCmd,
    profile_init_cmd::ProfileInitCmd,
    profile_list_cmd::ProfileListCmd,
    profile_show_cmd::ProfileShowCmd,
    profile_switch_cmd::ProfileSwitchCmd,
};

mod cmd_ctx_builder;
mod env_status_cmd;
mod profile_init_cmd;
mod profile_list_cmd;
mod profile_show_cmd;
mod profile_switch_cmd;
