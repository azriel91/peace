use crate::ctx::CmdCtxBuilderTypeParams;

/// Data stored by `CmdCtxBuilder` while building a
/// `CmdCtx<MultiProfileSingleFlow>`.
#[peace_code_gen::cmd_ctx_builder_impl]
#[derive(Debug)]
pub struct MultiProfileSingleFlowBuilder<CmdCtxBuilderTypeParamsT>
where
    CmdCtxBuilderTypeParamsT: CmdCtxBuilderTypeParams;
