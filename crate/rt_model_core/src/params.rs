//! Serializable data to initialize resources in a `CmdContext`.
//!
//! Each of these are `TypeMap<T>` newtypes, and are:
//!
//! * automatically serialized when a `CmdContext` is created with params.
//! * automatically deserialized and inserted as resources when subsequent
//!   `CmdContext`s are created.
//!
//! # Intended Use
//!
//! [`WorkspaceParams`] are information that is shared across all profiles and
//! flows in a workspace, such as:
//!
//! * User ID
//! * Customer ID
//!
//! [`ProfilesParams`] are information that are shared across flows in within a
//! profile, but specific to a profile -- `dev`, `prod` -- such as:
//!
//! * Profile name
//! * Server hostnames
//!
//! [`FlowParams`] are information that are applicable to a flow -- `deploy`,
//! `config_fetch`, `clean` -- such as:
//!
//! * Server count: applicable to `deploy`
//! * Force remove: applicable to `clean`

pub use self::{
    flow_params::FlowParams,
    flow_params_opt::FlowParamsOpt,
    params_keys::{KeyKnown, KeyMaybe, KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsKeysUnknown},
    params_type_regs::ParamsTypeRegs,
    params_type_regs_builder::ParamsTypeRegsBuilder,
    profile_params::ProfileParams,
    profile_params_opt::ProfileParamsOpt,
    workspace_params::WorkspaceParams,
    workspace_params_opt::WorkspaceParamsOpt,
};

mod flow_params;
mod flow_params_opt;
mod params_keys;
mod params_type_regs;
mod params_type_regs_builder;
mod profile_params;
mod profile_params_opt;
mod workspace_params;
mod workspace_params_opt;
