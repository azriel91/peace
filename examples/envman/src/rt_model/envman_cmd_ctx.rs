use peace::cmd::scopes::SingleProfileSingleFlow;

use crate::rt_model::EnvmanCmdCtxTypes;

/// Alias to simplify naming the `CmdCtx` type.
pub type EnvManCmdCtx<'ctx, Output, TS> =
    peace::cmd::ctx::CmdCtx<SingleProfileSingleFlow<'ctx, EnvmanCmdCtxTypes<Output>, TS>>;
