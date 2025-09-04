use peace::cmd_ctx::CmdCtxSpsf;

use crate::rt_model::EnvmanCmdCtxTypes;

/// Alias to simplify naming the `CmdCtx` type.
pub type EnvManCmdCtx<'ctx, O> = CmdCtxSpsf<'_, EnvmanCmdCtxTypes<O>>;
