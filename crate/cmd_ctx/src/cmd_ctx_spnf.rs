use interruptible::InterruptibilityState;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_profile_model::Profile;
use peace_resource_rt::paths::{
    PeaceAppDir, PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir,
};
use peace_rt_model::Workspace;
use peace_rt_model_core::params::{ProfileParams, WorkspaceParams};
use type_reg::untagged::{BoxDt, TypeReg};

use crate::{CmdCtxSpnfParams, CmdCtxSpnfParamsBuilder, CmdCtxTypes};

/// A command that works with a single profile, not scoped to a flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a           # ✅ can read `Profile`
/// |   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
/// |   |
/// |   |- 🌊 ..                   # ❌ cannot read or write Flow information
/// |
/// |- 🌏 ..                       # ❌ cannot read or write other `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write a single profile's parameters. For multiple profiles, see
///   [`CmdCtxMpnf`].
///
/// This kind of command cannot:
///
/// * Read or write flow parameters or state -- see [`CmdCtxSpsf`] or
///   [`CmdCtxMpsf`].
///
/// [`CmdCtxMpnf`]: crate::CmdCtxMpnf
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
/// [`CmdCtxSpsf`]: crate::CmdCtxSpsf
#[derive(Debug)]
pub struct CmdCtxSpnf<'ctx, CmdCtxTypesT>
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
    /// Inner fields without the `output`.
    ///
    /// # Design
    ///
    /// This is necessary so that the `output` can be separated from the fields
    /// during execution.
    pub fields: CmdCtxSpnfFields<'ctx, CmdCtxTypesT>,
}

/// Fields of [`CmdCtxSpnf`].
///
/// # Design
///
/// This is necessary so that the `output` can be separated from the fields
/// during execution.
#[derive(Debug)]
pub struct CmdCtxSpnfFields<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Whether the `CmdExecution` is interruptible.
    ///
    /// If it is, this holds the interrupt channel receiver.
    pub interruptibility_state: InterruptibilityState<'static, 'static>,
    /// Workspace that the `peace` tool runs in.
    pub workspace: OwnedOrRef<'ctx, Workspace>,
    /// The profile this command operates on.
    pub profile: Profile,
    /// Profile directory that stores params and flows.
    pub profile_dir: ProfileDir,
    /// Directory to store profile execution history.
    pub profile_history_dir: ProfileHistoryDir,
    /// Type registry for [`WorkspaceParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    pub workspace_params_type_reg: TypeReg<CmdCtxTypesT::WorkspaceParamsKey, BoxDt>,
    /// Workspace params.
    pub workspace_params: WorkspaceParams<CmdCtxTypesT::WorkspaceParamsKey>,
    /// Type registry for [`ProfileParams`] deserialization.
    ///
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    pub profile_params_type_reg: TypeReg<CmdCtxTypesT::ProfileParamsKey, BoxDt>,
    /// Profile params for the profile.
    pub profile_params: ProfileParams<CmdCtxTypesT::ProfileParamsKey>,
}

impl<'ctx, CmdCtxTypesT> CmdCtxSpnf<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Returns a [`CmdCtxSpnfParamsBuilder`] to construct this command context.
    pub fn builder<'ctx_local>() -> CmdCtxSpnfParamsBuilder<'ctx_local, CmdCtxTypesT> {
        CmdCtxSpnfParams::<'ctx_local, CmdCtxTypesT>::builder()
    }

    /// Returns a reference to the output.
    pub fn output(&self) -> &CmdCtxTypesT::Output {
        &self.output
    }

    /// Returns a mutable reference to the output.
    pub fn output_mut(&mut self) -> &mut CmdCtxTypesT::Output {
        &mut self.output
    }

    /// Returns a reference to the fields.
    pub fn fields(&self) -> &CmdCtxSpnfFields<'_, CmdCtxTypesT> {
        &self.fields
    }

    /// Returns a mutable reference to the fields.
    pub fn fields_mut(&mut self) -> &mut CmdCtxSpnfFields<'ctx, CmdCtxTypesT> {
        &mut self.fields
    }
}

impl<CmdCtxTypesT> CmdCtxSpnfFields<'_, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Returns the interruptibility capability.
    pub fn interruptibility_state(&mut self) -> InterruptibilityState<'_, '_> {
        self.interruptibility_state.reborrow()
    }

    /// Returns the workspace that the `peace` tool runs in.
    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    /// Returns a reference to the workspace directory.
    ///
    /// Convenience method for `cmd_ctx_Spsf.workspace.dirs().workspace_dir()`.
    pub fn workspace_dir(&self) -> &WorkspaceDir {
        self.workspace.dirs().workspace_dir()
    }

    /// Returns a reference to the `.peace` directory.
    ///
    /// Convenience method for `cmd_ctx_Spsf.workspace.dirs().peace_dir()`.
    pub fn peace_dir(&self) -> &PeaceDir {
        self.workspace.dirs().peace_dir()
    }

    /// Returns a reference to the `.peace/$app` directory.
    ///
    /// Convenience method for `cmd_ctx_Spsf.workspace.dirs().peace_app_dir()`.
    pub fn peace_app_dir(&self) -> &PeaceAppDir {
        self.workspace.dirs().peace_app_dir()
    }

    /// Returns a reference to the profile.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Returns a reference to the profile directory.
    pub fn profile_dir(&self) -> &ProfileDir {
        &self.profile_dir
    }

    /// Returns a reference to the profile history directory.
    pub fn profile_history_dir(&self) -> &ProfileHistoryDir {
        &self.profile_history_dir
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

    /// Returns a reference to the profile params type registry.
    pub fn profile_params_type_reg(&self) -> &TypeReg<CmdCtxTypesT::ProfileParamsKey, BoxDt> {
        &self.profile_params_type_reg
    }

    /// Returns a mutable reference to the profile params type registry.
    pub fn profile_params_type_reg_mut(
        &mut self,
    ) -> &mut TypeReg<CmdCtxTypesT::ProfileParamsKey, BoxDt> {
        &mut self.profile_params_type_reg
    }
}
