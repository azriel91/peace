#![allow(clippy::type_complexity)]

/// Data stored by `CmdCtxBuilder` while building a
/// `CmdCtx<SingleProfileSingleFlow>`.
#[peace_code_gen::cmd_ctx_builder_impl]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleProfileSingleFlowBuilder;
