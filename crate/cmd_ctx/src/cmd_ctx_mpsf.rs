use std::{collections::BTreeMap, fmt::Debug};

use interruptible::InterruptibilityState;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_flow_rt::Flow;
use peace_params::{MappingFnReg, ParamsSpecs};
use peace_profile_model::Profile;
use peace_resource_rt::{
    paths::{FlowDir, PeaceAppDir, PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir},
    resources::ts::SetUp,
    states::StatesCurrentStored,
    Resources,
};
use peace_rt_model::{
    params::{FlowParams, ProfileParams, WorkspaceParams},
    ParamsSpecsTypeReg, StatesTypeReg, Workspace,
};
use type_reg::untagged::{BoxDt, TypeReg};

use crate::{CmdCtxMpsfParams, CmdCtxMpsfParamsBuilder, CmdCtxTypes};

/// A command that works with multiple profiles, and a single flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a           # ✅ can list multiple `Profile`s
/// |   |- 📝 profile_params.yaml  # ✅ can read multiple `ProfileParams`
/// |   |
/// |   |- 🌊 deploy                # ✅ can read `FlowId`
/// |   |   |- 📝 flow_params.yaml  # ✅ can read or write `FlowParams`
/// |   |   |- 📋 states_goal.yaml  # ✅ can read or write `StatesGoal`
/// |   |   |- 📋 states_current.yaml # ✅ can read or write `StatesCurrentStored`
/// |   |
/// |   |- 🌊 ..                       # ❌ cannot read or write other `Flow` information
/// |
/// |- 🌏 customer_a_dev           # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |   |
/// |   |- 🌊 deploy                # ✅
/// |       |- 📝 flow_params.yaml  # ✅
/// |       |- 📋 states_goal.yaml  # ✅
/// |       |- 📋 states_current.yaml # ✅
/// |
/// |- 🌏 customer_a_prod          # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |   |
/// |   |- 🌊 deploy                # ✅
/// |       |- 📝 flow_params.yaml  # ✅
/// |       |- 📋 states_goal.yaml  # ✅
/// |       |- 📋 states_current.yaml # ✅
/// |
/// |
/// |- 🌏 workspace_init           # ✅ can list multiple `Profile`s
///     |- 📝 profile_params.yaml  # ❌ cannot read profile params of different underlying type
/// |   |- 🌊 workspace_init       # ❌ cannot read unrelated flows
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write multiple profiles' parameters &ndash; as long as they are of
///   the same type (same `struct`).
/// * Read or write flow parameters for the same flow.
/// * Read or write flow state for the same flow.
///
/// This kind of command cannot:
///
/// * Read or write flow parameters for different flows.
/// * Read or write flow state for different flows.
#[derive(Debug)]
pub struct CmdCtxMpsf<'ctx, CmdCtxTypesT>
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
    pub fields: CmdCtxMpsfFields<'ctx, CmdCtxTypesT>,
}

/// Fields of [`CmdCtxMpsf`].
///
/// # Design
///
/// This is necessary so that the `output` can be separated from the fields
/// during execution.
#[derive(Debug)]
pub struct CmdCtxMpsfFields<'ctx, CmdCtxTypesT>
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
    /// The chosen process flow.
    pub flow: OwnedOrRef<'ctx, Flow<CmdCtxTypesT::AppError>>,
    /// Flow directory that stores params and states.
    pub flow_dirs: BTreeMap<Profile, FlowDir>,
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
    /// Type registry for [`FlowParams`] deserialization.
    ///
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    pub flow_params_type_reg: TypeReg<CmdCtxTypesT::FlowParamsKey, BoxDt>,
    /// Flow params for the selected flow.
    pub profile_to_flow_params: BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>>,
    /// Stored current states for each profile for the selected flow.
    pub profile_to_states_current_stored: BTreeMap<Profile, Option<StatesCurrentStored>>,
    /// Type registry for each item's [`Params`]`::Spec`.
    ///
    /// This is used to deserialize [`ParamsSpecsFile`].
    ///
    /// [`Params`]: peace_cfg::Item::Params
    /// [`ParamsSpecsFile`]: peace_resource_rt::paths::ParamsSpecsFile
    pub params_specs_type_reg: ParamsSpecsTypeReg,
    /// Item params specs for each profile for the selected flow.
    pub profile_to_params_specs: BTreeMap<Profile, ParamsSpecs>,
    /// Mapping function registry for each item's [`Params`]`::Spec`.
    ///
    /// This maps the [`MappingFnName`] stored in [`ParamsSpecsFile`] to the
    /// [`MappingFn`] logic.
    ///
    /// [`MappingFn`]: peace_params::MappingFn
    /// [`MappingFnName`]: peace_params::MappingFnName
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

impl<'ctx, CmdCtxTypesT> CmdCtxMpsf<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Returns a [`CmdCtxMpsfParamsBuilder`] to construct this command context.
    pub fn builder<'ctx_local>() -> CmdCtxMpsfParamsBuilder<'ctx_local, CmdCtxTypesT> {
        CmdCtxMpsfParams::<'ctx_local, CmdCtxTypesT>::builder()
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
    pub fn fields(&self) -> &CmdCtxMpsfFields<'_, CmdCtxTypesT> {
        &self.fields
    }

    /// Returns a mutable reference to the fields.
    pub fn fields_mut(&mut self) -> &mut CmdCtxMpsfFields<'ctx, CmdCtxTypesT> {
        &mut self.fields
    }
}

impl<CmdCtxTypesT> CmdCtxMpsfFields<'_, CmdCtxTypesT>
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

    /// Returns the flow.
    pub fn flow(&self) -> &Flow<CmdCtxTypesT::AppError> {
        &self.flow
    }

    /// Returns the flow directories keyed by each profile.
    pub fn flow_dirs(&self) -> &BTreeMap<Profile, FlowDir> {
        &self.flow_dirs
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

    /// Returns a reference to the flow params type registry.
    pub fn flow_params_type_reg(&self) -> &TypeReg<CmdCtxTypesT::FlowParamsKey, BoxDt> {
        &self.flow_params_type_reg
    }

    /// Returns a mutable reference to the flow params type registry.
    pub fn flow_params_type_reg_mut(&mut self) -> &mut TypeReg<CmdCtxTypesT::FlowParamsKey, BoxDt> {
        &mut self.flow_params_type_reg
    }

    /// Returns the flow params for each profile.
    pub fn profile_to_flow_params(
        &self,
    ) -> &BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>> {
        &self.profile_to_flow_params
    }

    /// Returns the flow params for each profile.
    pub fn profile_to_flow_params_mut(
        &mut self,
    ) -> &mut BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>> {
        &mut self.profile_to_flow_params
    }

    /// Returns the stored current states for each profile for the selected
    /// flow.
    pub fn profile_to_states_current_stored(
        &self,
    ) -> &BTreeMap<Profile, Option<StatesCurrentStored>> {
        &self.profile_to_states_current_stored
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

    /// Returns the item params specs for each profile for the selected
    /// flow.
    pub fn profile_to_params_specs(&self) -> &BTreeMap<Profile, ParamsSpecs> {
        &self.profile_to_params_specs
    }

    /// Returns the mapping function registry for each item's
    /// [`Params`]`::Spec`.
    ///
    /// This maps the [`MappingFnName`] stored in [`ParamsSpecsFile`] to the
    /// [`MappingFn`] logic.
    ///
    /// [`MappingFn`]: peace_params::MappingFn
    /// [`MappingFnName`]: peace_params::MappingFnName
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
