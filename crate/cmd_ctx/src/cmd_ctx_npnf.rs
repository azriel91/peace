use interruptible::InterruptibilityState;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_resource_rt::paths::{PeaceAppDir, PeaceDir, WorkspaceDir};
use peace_rt_model::Workspace;
use peace_rt_model_core::params::WorkspaceParams;
use type_reg::untagged::{BoxDt, TypeReg};

use crate::{CmdCtxNpnfParams, CmdCtxNpnfParamsBuilder, CmdCtxTypes};

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

impl<CmdCtxTypesT> CmdCtxNpnf<'_, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Returns a [`CmdCtxNpnfParamsBuilder`] to construct this command context.
    pub fn builder<'ctx>() -> CmdCtxNpnfParamsBuilder<'ctx, CmdCtxTypesT> {
        CmdCtxNpnfParams::<'ctx, CmdCtxTypesT>::builder()
    }

    /// Returns a reference to the output.
    pub fn output(&self) -> &CmdCtxTypesT::Output {
        &self.output
    }

    /// Returns a mutable reference to the output.
    pub fn output_mut(&mut self) -> &mut CmdCtxTypesT::Output {
        &mut self.output
    }

    /// Returns the interruptibility capability.
    pub fn interruptibility_state(&mut self) -> InterruptibilityState<'_, '_> {
        self.interruptibility_state.reborrow()
    }

    /// Returns the workspace that the `peace` tool runs in.
    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    /// Returns a reference to the workspace directory.
    pub fn workspace_dir(&self) -> &WorkspaceDir {
        self.workspace.dirs().workspace_dir()
    }

    /// Returns a reference to the `.peace` directory.
    pub fn peace_dir(&self) -> &PeaceDir {
        self.workspace.dirs().peace_dir()
    }

    /// Returns a reference to the `.peace/$app` directory.
    pub fn peace_app_dir(&self) -> &PeaceAppDir {
        self.workspace.dirs().peace_app_dir()
    }

    /// Returns a reference to the workspace params type registry.
    pub fn workspace_params_type_reg(&self) -> &TypeReg<CmdCtxTypesT::WorkspaceParamsKey, BoxDt> {
        &self.workspace_params_type_reg
    }

    /// Returns a mutable reference to the workspace params type registry.
    pub fn workspace_params_type_reg_mut(
        &mut self,
    ) -> &mut TypeReg<CmdCtxTypesT::WorkspaceParamsKey, BoxDt> {
        &mut self.workspace_params_type_reg
    }

    /// Returns the workspace params.
    pub fn workspace_params(&self) -> &WorkspaceParams<CmdCtxTypesT::WorkspaceParamsKey> {
        &self.workspace_params
    }

    /// Returns the workspace params.
    pub fn workspace_params_mut(
        &mut self,
    ) -> &mut WorkspaceParams<CmdCtxTypesT::WorkspaceParamsKey> {
        &mut self.workspace_params
    }
}
