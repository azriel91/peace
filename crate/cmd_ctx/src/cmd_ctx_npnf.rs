use interruptible::InterruptibilityState;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_rt_model::Workspace;
use peace_rt_model_core::params::WorkspaceParams;
use type_reg::untagged::{BoxDt, TypeReg};

use crate::CmdCtxTypes;

/// Context for a command that only works with workspace parameters -- no
/// profile / no flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 ..                       # ❌ cannot read or write `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
///
/// This kind of command cannot:
///
/// * Read or write profile parameters -- see [`CmdCtxSpnf`] or [`CmdCtxMpnf`].
/// * Read or write flow parameters or state -- see [`CmdCtxSpsf`] or
///   [`CmdCtxMpsf`].
///
/// [`CmdCtxMpnf`]: crate::CmdCtxMpnf
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
/// [`CmdCtxSpnf`]: crate::CmdCtxSpnf
/// [`CmdCtxSpsf`]: crate::CmdCtxSpsf
#[derive(Debug)]
pub struct CmdCtxNpnf<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,
    /// Whether the `CmdExecution` is interruptible.
    ///
    /// If it is, this holds the interrupt channel receiver.
    pub interruptibility_state: InterruptibilityState<'static, 'static>,
    /// Workspace that the `peace` tool runs in.
    pub workspace: OwnedOrRef<'ctx, Workspace>,
    /// Type registry for [`WorkspaceParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    pub workspace_params_type_reg: TypeReg<CmdCtxTypesT::WorkspaceParamsKey, BoxDt>,
    /// Workspace params.
    pub workspace_params: WorkspaceParams<CmdCtxTypesT::WorkspaceParamsKey>,
}
