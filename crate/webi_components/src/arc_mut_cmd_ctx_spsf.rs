use std::sync::{Arc, Mutex};

use peace_cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow};

/// Type alias for `CmdCtx<SingleProfileSingleFlow<_>>`.
pub type ArcMutCmdCtxSpsf<CmdCtxTypesT> =
    Arc<Mutex<CmdCtx<SingleProfileSingleFlow<'static, CmdCtxTypesT>>>>;
