use std::{collections::BTreeMap, fmt::Debug};

use interruptible::InterruptibilityState;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_profile_model::Profile;
use peace_resource_rt::paths::{
    PeaceAppDir, PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir,
};
use peace_rt_model::{
    params::{ProfileParams, WorkspaceParams},
    Workspace,
};
use type_reg::untagged::{BoxDt, TypeReg};

use crate::{CmdCtxMpnfParams, CmdCtxMpnfParamsBuilder, CmdCtxTypes};

/// A command that works with multiple profiles, not scoped to a flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a           # ✅ can list multiple `Profile`s
/// |   |- 📝 profile_params.yaml  # ✅ can read multiple `ProfileParams`
/// |   |
/// |   |- ..                      # ❌ cannot read or write `Flow` information
/// |
/// |- 🌏 customer_a_dev           # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |
/// |- 🌏 customer_a_prod          # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |
/// |- 🌏 workspace_init           # ✅ can list multiple `Profile`s
///     |- 📝 profile_params.yaml  # ❌ cannot read profile params of different underlying type
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write multiple profiles' parameters &ndash; as long as they are of
///   the same type (same `struct`).
///
/// This kind of command cannot:
///
/// * Read or write flow parameters -- see [`CmdCtxMpsf`].
/// * Read or write flow state -- see [`CmdCtxMpsf`].
///
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
#[derive(Debug)]
pub struct CmdCtxMpnf<'ctx, CmdCtxTypesT>
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
    pub fields: CmdCtxMpnfFields<'ctx, CmdCtxTypesT>,
}

/// Fields of [`CmdCtxMpnf`].
///
/// # Design
///
/// This is necessary so that the `output` can be separated from the fields
/// during execution.
#[derive(Debug)]
pub struct CmdCtxMpnfFields<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Whether the `CmdExecution` is interruptible.
    ///
    /// If it is, this holds the interrupt channel receiver.
    pub interruptibility_state: InterruptibilityState<'static, 'static>,
    /// Workspace that the `peace` tool runs in.
    pub workspace: OwnedOrRef<'ctx, Workspace>,
    /// The profiles that are accessible by this command.
    pub profiles: Vec<Profile>,
    /// Profile directories that store params and flows.
    pub profile_dirs: BTreeMap<Profile, ProfileDir>,
    /// Directories of each profile's execution history.
    pub profile_history_dirs: BTreeMap<Profile, ProfileHistoryDir>,
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
    pub profile_to_profile_params: BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>>,
}

impl<'ctx, CmdCtxTypesT> CmdCtxMpnf<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Returns a [`CmdCtxMpnfParamsBuilder`] to construct this command context.
    pub fn builder<'ctx_local>() -> CmdCtxMpnfParamsBuilder<'ctx_local, CmdCtxTypesT> {
        CmdCtxMpnfParams::<'ctx_local, CmdCtxTypesT>::builder()
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
    pub fn fields(&self) -> &CmdCtxMpnfFields<'_, CmdCtxTypesT> {
        &self.fields
    }

    /// Returns a mutable reference to the fields.
    pub fn fields_mut(&mut self) -> &mut CmdCtxMpnfFields<'ctx, CmdCtxTypesT> {
        &mut self.fields
    }
}

impl<CmdCtxTypesT> CmdCtxMpnfFields<'_, CmdCtxTypesT>
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

    /// Returns the accessible profiles.
    ///
    /// These are the profiles that are filtered by the filter function, if
    /// provided.
    pub fn profiles(&self) -> &[Profile] {
        self.profiles.as_ref()
    }

    /// Returns the profile directories keyed by each profile.
    pub fn profile_dirs(&self) -> &BTreeMap<Profile, ProfileDir> {
        &self.profile_dirs
    }

    /// Returns the profile history directories keyed by each profile.
    pub fn profile_history_dirs(&self) -> &BTreeMap<Profile, ProfileHistoryDir> {
        &self.profile_history_dirs
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

    /// Returns the profile params for each profile.
    pub fn profile_to_profile_params(
        &self,
    ) -> &BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>> {
        &self.profile_to_profile_params
    }

    /// Returns the profile params for each profile.
    pub fn profile_to_profile_params_mut(
        &mut self,
    ) -> &mut BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>> {
        &mut self.profile_to_profile_params
    }
}
