//! Data structures

pub use self::{
    cmd_exec_request::CmdExecRequest,
    env_diff_selection::EnvDiffSelection,
    env_man_flow::EnvManFlow,
    env_man_flow_parse_error::EnvManFlowParseError,
    env_type::EnvType,
    env_type_parse_error::EnvTypeParseError,
    envman_error::EnvManError,
    item_ids::WebApp,
    params_keys::{ProfileParamsKey, WorkspaceParamsKey},
    profile_switch::ProfileSwitch,
    repo_slug::RepoSlug,
    repo_slug_error::RepoSlugError,
};

#[cfg(feature = "cli")]
pub mod cli_args;

mod cmd_exec_request;
mod env_diff_selection;
mod env_man_flow;
mod env_man_flow_parse_error;
mod env_type;
mod env_type_parse_error;
mod envman_error;
mod item_ids;
mod params_keys;
mod profile_switch;
mod repo_slug;
mod repo_slug_error;
