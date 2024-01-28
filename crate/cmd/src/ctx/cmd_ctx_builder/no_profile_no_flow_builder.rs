use crate::ctx::CmdCtxBuilderTypes;

/// Data stored by `CmdCtxBuilder` while building a
/// `CmdCtx<NoProfileNoFlow>`.
#[peace_code_gen::cmd_ctx_builder_impl]
#[derive(Debug)]
pub struct NoProfileNoFlowBuilder<CmdCtxBuilderTypesT>
where
    CmdCtxBuilderTypesT: CmdCtxBuilderTypes;
