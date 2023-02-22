#![allow(clippy::type_complexity)]

use std::{fmt::Debug, hash::Hash};

use peace_resources::Resources;
use peace_rt_model::{
    cmd_context_params::{ParamsKeysImpl, ParamsTypeRegs},
    Error, Workspace,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::ctx::cmd_ctx_builder::{
    flow_params_selection::{FlowParamsNone, FlowParamsSome},
    profile_params_selection::{ProfileParamsNone, ProfileParamsSome},
    workspace_params_selection::{WorkspaceParamsNone, WorkspaceParamsSome},
};

/// Data stored by `CmdCtxBuilder` while building a
/// `CmdCtx<SingleProfileSingleFlow>`.
#[peace_code_gen::cmd_ctx_builder_impl]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleProfileSingleFlowBuilder;
