use interruptible::InterruptibilityState;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_flow_rt::Flow;
use peace_params::{MappingFnReg, ParamsSpecs};
use peace_profile_model::Profile;
use peace_resource_rt::{
    paths::{FlowDir, PeaceAppDir, PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir},
    resources::ts::SetUp,
    Resources,
};
use peace_rt_model::{ParamsSpecsTypeReg, StatesTypeReg, Workspace};
use peace_rt_model_core::params::{FlowParams, ProfileParams, WorkspaceParams};
use type_reg::untagged::{BoxDt, TypeReg};

use crate::{CmdCtxSpsfParams, CmdCtxSpsfParamsBuilder, CmdCtxTypes};

/// Context for a command that works with one profile and one flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a
/// |   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
/// |   |
/// |   |- 🌊 deploy                  # ✅ can read `FlowId`
/// |   |   |- 📝 flow_params.yaml    # ✅ can read or write `FlowParams`
/// |   |   |- 📋 states_goal.yaml    # ✅ can read or write `StatesGoal`
/// |   |   |- 📋 states_current.yaml # ✅ can read or write `StatesCurrentStored`
/// |   |
/// |   |- 🌊 ..                   # ❌ cannot read or write other `Flow` information
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
///   [`CmdCtxMpsf`].
///
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
#[derive(Debug)]
pub struct CmdCtxSpsf<'ctx, CmdCtxTypesT>
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
    /// Tracks progress of each function execution.
    #[cfg(feature = "output_progress")]
    pub cmd_progress_tracker: peace_rt_model::CmdProgressTracker,
    /// Inner fields without the `output` and `cmd_progress_tracker`.
    ///
    /// # Design
    ///
    /// This is necessary so that the `output` and `cmd_progress_tracker` can be
    /// used in `CmdExecution` while passing the fields to the `CmdBlock`.
    pub fields: CmdCtxSpsfFields<'ctx, CmdCtxTypesT>,
}

/// Fields of [`CmdCtxSpsf`].
///
/// # Design
///
/// This is necessary so that the `output` and `cmd_progress_tracker` can be
/// used in `CmdExecution` while passing the fields to the `CmdBlock`.
#[derive(Debug)]
pub struct CmdCtxSpsfFields<'ctx, CmdCtxTypesT>
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
    /// The chosen process flow.
    pub flow: OwnedOrRef<'ctx, Flow<CmdCtxTypesT::AppError>>,
    /// Flow directory that stores params and states.
    pub flow_dir: FlowDir,
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
    /// Type registry for [`FlowParams`] deserialization.
    ///
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    pub flow_params_type_reg: TypeReg<CmdCtxTypesT::FlowParamsKey, BoxDt>,
    /// Flow params for the selected flow.
    pub flow_params: FlowParams<CmdCtxTypesT::FlowParamsKey>,
    /// Type registry for each item's [`Params`]`::Spec`.
    ///
    /// This is used to deserialize [`ParamsSpecsFile`].
    ///
    /// [`Params`]: peace_cfg::Item::Params
    /// [`ParamsSpecsFile`]: peace_resource_rt::paths::ParamsSpecsFile
    pub params_specs_type_reg: ParamsSpecsTypeReg,
    /// Item params specs for the selected flow.
    pub params_specs: ParamsSpecs,
    /// Mapping function registry for each item's [`Params`]`::Spec`.
    ///
    /// This maps the [`MappingFnId`] stored in [`ParamsSpecsFile`] to the
    /// [`MappingFn`] logic.
    ///
    /// [`MappingFn`]: peace_params::MappingFn
    /// [`MappingFnId`]: peace_params::MappingFnId
    /// [`Params`]: peace_cfg::Item::Params
    /// [`ParamsSpecsFile`]: peace_resource_rt::paths::ParamsSpecsFile
    pub mapping_fn_reg: MappingFnReg,
    /// Type registry for each item's `State`.
    ///
    /// This is used to deserialize [`StatesCurrentFile`] and
    /// [`StatesGoalFile`].
    ///
    /// [`StatesCurrentFile`]: peace_resource_rt::paths::StatesCurrentFile
    /// [`StatesGoalFile`]: peace_resource_rt::paths::StatesGoalFile
    pub states_type_reg: StatesTypeReg,
    /// `Resources` for flow execution.
    pub resources: Resources<SetUp>,
}

impl<'ctx, CmdCtxTypesT> CmdCtxSpsf<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Returns a [`CmdCtxSpsfParamsBuilder`] to construct this command context.
    pub fn builder<'ctx_local>() -> CmdCtxSpsfParamsBuilder<'ctx_local, CmdCtxTypesT> {
        CmdCtxSpsfParams::<'ctx_local, CmdCtxTypesT>::builder()
    }

    /// Returns a reference to the output.
    pub fn output(&self) -> &CmdCtxTypesT::Output {
        &self.output
    }

    /// Returns a mutable reference to the output.
    pub fn output_mut(&mut self) -> &mut CmdCtxTypesT::Output {
        &mut self.output
    }

    /// Returns the progress tracker for all functions' executions.
    #[cfg(feature = "output_progress")]
    pub fn cmd_progress_tracker(&self) -> &peace_rt_model::CmdProgressTracker {
        &self.cmd_progress_tracker
    }

    /// Returns a mutable reference to the progress tracker for all functions'
    /// executions.
    #[cfg(feature = "output_progress")]
    pub fn cmd_progress_tracker_mut(&mut self) -> &mut peace_rt_model::CmdProgressTracker {
        &mut self.cmd_progress_tracker
    }

    /// Returns a reference to the fields.
    pub fn fields(&self) -> &CmdCtxSpsfFields<'_, CmdCtxTypesT> {
        &self.fields
    }

    /// Returns a mutable reference to the fields.
    pub fn fields_mut(&mut self) -> &mut CmdCtxSpsfFields<'ctx, CmdCtxTypesT> {
        &mut self.fields
    }
}

impl<CmdCtxTypesT> CmdCtxSpsfFields<'_, CmdCtxTypesT>
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

    /// Returns a reference to the flow.
    pub fn flow(&self) -> &Flow<CmdCtxTypesT::AppError> {
        &self.flow
    }

    /// Returns a reference to the flow directory.
    pub fn flow_dir(&self) -> &FlowDir {
        &self.flow_dir
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

    /// Returns the profile params.
    pub fn profile_params(&self) -> &ProfileParams<CmdCtxTypesT::ProfileParamsKey> {
        &self.profile_params
    }

    /// Returns the profile params.
    pub fn profile_params_mut(&mut self) -> &mut ProfileParams<CmdCtxTypesT::ProfileParamsKey> {
        &mut self.profile_params
    }

    /// Returns a reference to the flow params type registry.
    pub fn flow_params_type_reg(&self) -> &TypeReg<CmdCtxTypesT::FlowParamsKey, BoxDt> {
        &self.flow_params_type_reg
    }

    /// Returns a mutable reference to the flow params type registry.
    pub fn flow_params_type_reg_mut(&mut self) -> &mut TypeReg<CmdCtxTypesT::FlowParamsKey, BoxDt> {
        &mut self.flow_params_type_reg
    }

    /// Returns the flow params.
    pub fn flow_params(&self) -> &FlowParams<CmdCtxTypesT::FlowParamsKey> {
        &self.flow_params
    }

    /// Returns the flow params.
    pub fn flow_params_mut(&mut self) -> &mut FlowParams<CmdCtxTypesT::FlowParamsKey> {
        &mut self.flow_params
    }

    /// Returns the type registry for each item's [`Params`]`::Spec`.
    ///
    /// This is used to deserialize [`ParamsSpecsFile`].
    ///
    /// [`Params`]: peace_cfg::Item::Params
    /// [`ParamsSpecsFile`]: peace_resource_rt::paths::ParamsSpecsFile
    pub fn params_specs_type_reg(&self) -> &ParamsSpecsTypeReg {
        &self.params_specs_type_reg
    }

    /// Returns the item params specs for the selected flow.
    pub fn params_specs(&self) -> &ParamsSpecs {
        &self.params_specs
    }

    /// Returns the mapping function registry for each item's
    /// [`Params`]`::Spec`.
    ///
    /// This maps the [`MappingFnId`] stored in [`ParamsSpecsFile`] to the
    /// [`MappingFn`] logic.
    ///
    /// [`MappingFn`]: peace_params::MappingFn
    /// [`MappingFnId`]: peace_params::MappingFnId
    /// [`Params`]: peace_cfg::Item::Params
    /// [`ParamsSpecsFile`]: peace_resource_rt::paths::ParamsSpecsFile
    pub fn mapping_fn_reg(&self) -> &MappingFnReg {
        &self.mapping_fn_reg
    }

    /// Returns the type registry for each item's `State`.
    ///
    /// This is used to deserialize [`StatesCurrentFile`] and
    /// [`StatesGoalFile`].
    ///
    /// [`StatesCurrentFile`]: peace_resource_rt::paths::StatesCurrentFile
    /// [`StatesGoalFile`]: peace_resource_rt::paths::StatesGoalFile
    pub fn states_type_reg(&self) -> &StatesTypeReg {
        &self.states_type_reg
    }

    /// Returns a reference to the `Resources` for flow execution.
    pub fn resources(&self) -> &Resources<SetUp> {
        &self.resources
    }

    /// Returns a reference to the `Resources` for flow execution.
    pub fn resources_mut(&mut self) -> &mut Resources<SetUp> {
        &mut self.resources
    }
}
