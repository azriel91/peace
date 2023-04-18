use std::{collections::BTreeMap, fmt::Debug, hash::Hash};

use peace_core::Profile;
use peace_resources::{
    paths::{FlowDir, PeaceAppDir, PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir},
    resources::ts::SetUp,
    states::StatesSaved,
    Resources,
};
use peace_rt_model::{
    params::{
        FlowParams, KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs, ProfileParams,
        WorkspaceParams,
    },
    Flow, ItemSpecParams, ItemSpecParamsTypeReg, StatesTypeReg, Workspace,
};
use serde::{de::DeserializeOwned, Serialize};

/// A command that works with multiple profiles, and a single flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a           # ✅ can list multiple `Profile`s
/// |   |- 📝 profile_params.yaml  # ✅ can read multiple `ProfileParams`
/// |   |
/// |   |- 🌊 deploy                   # ✅ can read `FlowId`
/// |   |   |- 📝 flow_params.yaml     # ✅ can read or write `FlowParams`
/// |   |   |- 📋 states_desired.yaml  # ✅ can read or write `StatesDesired`
/// |   |   |- 📋 states_saved.yaml    # ✅ can read or write `StatesSaved`
/// |   |
/// |   |- 🌊 ..                       # ❌ cannot read or write other `Flow` information
/// |
/// |- 🌏 customer_a_dev           # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |   |
/// |   |- 🌊 deploy                   # ✅
/// |       |- 📝 flow_params.yaml     # ✅
/// |       |- 📋 states_desired.yaml  # ✅
/// |       |- 📋 states_saved.yaml    # ✅
/// |
/// |- 🌏 customer_a_prod          # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |   |
/// |   |- 🌊 deploy                   # ✅
/// |       |- 📝 flow_params.yaml     # ✅
/// |       |- 📋 states_desired.yaml  # ✅
/// |       |- 📋 states_saved.yaml    # ✅
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
pub struct MultiProfileSingleFlow<'ctx, E, O, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    output: &'ctx mut O,
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// The profiles that are accessible by this command.
    profiles: Vec<Profile>,
    /// Profile directories that store params and flows.
    profile_dirs: BTreeMap<Profile, ProfileDir>,
    /// Directories of each profile's execution history.
    profile_history_dirs: BTreeMap<Profile, ProfileHistoryDir>,
    /// The chosen process flow.
    flow: &'ctx Flow<E>,
    /// Flow directory that stores params and states.
    flow_dirs: BTreeMap<Profile, FlowDir>,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    params_type_regs: ParamsTypeRegs<PKeys>,
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    profile_to_profile_params:
        BTreeMap<Profile, ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>>,
    /// Flow params for the selected flow.
    profile_to_flow_params:
        BTreeMap<Profile, FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>>,
    /// Saved states for each profile for the selected flow.
    profile_to_states_saved: BTreeMap<Profile, Option<StatesSaved>>,
    /// Type registry for each item spec's [`Params`].
    ///
    /// This is used to deserialize [`ItemSpecParamsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ItemSpecParamsFile`]: peace_resources::paths::ItemSpecParamsFile
    item_spec_params_type_reg: ItemSpecParamsTypeReg,
    /// Item spec params for each profile for the selected flow.
    profile_to_item_spec_params: BTreeMap<Profile, Option<ItemSpecParams>>,
    /// Type registry for each item spec's `State`.
    ///
    /// This is used to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    states_type_reg: StatesTypeReg,
    /// `Resources` for flow execution.
    resources: Resources<TS>,
}

/// Access to fields in `MultiProfileSingleFlow` so that multiple borrows can
/// happen simultaneously.
#[derive(Debug)]
pub struct MultiProfileSingleFlowView<'view, E, O, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: &'view mut O,
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'view Workspace,
    /// The profiles that are accessible by this command.
    pub profiles: &'view [Profile],
    /// Profile directories that store params and flows.
    pub profile_dirs: &'view BTreeMap<Profile, ProfileDir>,
    /// Directories of each profile's execution history.
    pub profile_history_dirs: &'view BTreeMap<Profile, ProfileHistoryDir>,
    /// The chosen process flow.
    pub flow: &'view Flow<E>,
    /// Flow directory that stores params and states.
    pub flow_dirs: &'view BTreeMap<Profile, FlowDir>,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    pub params_type_regs: &'view ParamsTypeRegs<PKeys>,
    /// Workspace params.
    pub workspace_params: &'view WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    pub profile_to_profile_params:
        &'view BTreeMap<Profile, ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>>,
    /// Flow params for the selected flow.
    pub profile_to_flow_params:
        &'view BTreeMap<Profile, FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>>,
    /// Saved states for each profile for the selected flow.
    pub profile_to_states_saved: &'view BTreeMap<Profile, Option<StatesSaved>>,
    /// Type registry for each item spec's [`Params`].
    ///
    /// This is used to deserialize [`ItemSpecParamsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ItemSpecParamsFile`]: peace_resources::paths::ItemSpecParamsFile
    pub item_spec_params_type_reg: &'view ItemSpecParamsTypeReg,
    /// Item spec params for each profile for the selected flow.
    pub profile_to_item_spec_params: &'view BTreeMap<Profile, Option<ItemSpecParams>>,
    /// Type registry for each item spec's `State`.
    ///
    /// This is used to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    pub states_type_reg: &'view StatesTypeReg,
    /// `Resources` for flow execution.
    pub resources: &'view mut Resources<TS>,
}

impl<'ctx, E, O, PKeys> MultiProfileSingleFlow<'ctx, E, O, PKeys, SetUp>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `MultiProfileSingleFlow` scope.
    #[allow(clippy::too_many_arguments)] // Constructed by proc macro
    pub(crate) fn new(
        output: &'ctx mut O,
        workspace: &'ctx Workspace,
        profiles: Vec<Profile>,
        profile_dirs: BTreeMap<Profile, ProfileDir>,
        profile_history_dirs: BTreeMap<Profile, ProfileHistoryDir>,
        flow: &'ctx Flow<E>,
        flow_dirs: BTreeMap<Profile, FlowDir>,
        params_type_regs: ParamsTypeRegs<PKeys>,
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        profile_to_profile_params: BTreeMap<
            Profile,
            ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
        >,
        profile_to_flow_params: BTreeMap<
            Profile,
            FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
        >,
        profile_to_states_saved: BTreeMap<Profile, Option<StatesSaved>>,
        item_spec_params_type_reg: ItemSpecParamsTypeReg,
        profile_to_item_spec_params: BTreeMap<Profile, Option<ItemSpecParams>>,
        states_type_reg: StatesTypeReg,
        resources: Resources<SetUp>,
    ) -> Self {
        Self {
            output,
            workspace,
            profiles,
            profile_dirs,
            profile_history_dirs,
            flow,
            flow_dirs,
            params_type_regs,
            workspace_params,
            profile_to_profile_params,
            profile_to_flow_params,
            profile_to_states_saved,
            item_spec_params_type_reg,
            profile_to_item_spec_params,
            states_type_reg,
            resources,
        }
    }
}

impl<'ctx, E, O, PKeys, TS> MultiProfileSingleFlow<'ctx, E, O, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a view struct of this scope.
    ///
    /// This allows the flow and resources to be borrowed concurrently.
    pub fn view(&mut self) -> MultiProfileSingleFlowView<'_, E, O, PKeys, TS> {
        let Self {
            output,
            workspace,
            profiles,
            profile_dirs,
            profile_history_dirs,
            flow,
            flow_dirs,
            params_type_regs,
            workspace_params,
            profile_to_profile_params,
            profile_to_flow_params,
            profile_to_states_saved,
            item_spec_params_type_reg,
            profile_to_item_spec_params,
            states_type_reg,
            resources,
        } = self;

        MultiProfileSingleFlowView {
            output,
            workspace,
            profiles,
            profile_dirs,
            profile_history_dirs,
            flow,
            flow_dirs,
            params_type_regs,
            workspace_params,
            profile_to_profile_params,
            profile_to_flow_params,
            profile_to_states_saved,
            item_spec_params_type_reg,
            profile_to_item_spec_params,
            states_type_reg,
            resources,
        }
    }

    /// Returns a reference to the output.
    pub fn output(&self) -> &O {
        self.output
    }

    /// Returns a mutable reference to the output.
    pub fn output_mut(&mut self) -> &mut O {
        self.output
    }

    /// Returns the workspace that the `peace` tool runs in.
    pub fn workspace(&self) -> &Workspace {
        self.workspace
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
    pub fn flow(&self) -> &Flow<E> {
        self.flow
    }

    /// Returns the flow directories keyed by each profile.
    pub fn flow_dirs(&self) -> &BTreeMap<Profile, FlowDir> {
        &self.flow_dirs
    }

    /// Returns the type registries for [`WorkspaceParams`], [`ProfileParams`],
    /// and [`FlowParams`] deserialization.
    ///
    /// Not to be confused with [`item_spec_params_type_reg`], which is used to
    /// deserialize [`ItemSpecParams`]
    ///
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    /// [`ItemSpecParams`]: peace_rt_model::ItemSpecParams
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    /// [`item_spec_params_type_reg`]: Self::item_spec_params_type_reg
    pub fn params_type_regs(&self) -> &ParamsTypeRegs<PKeys> {
        &self.params_type_regs
    }

    /// Returns the saved states for each profile for the selected flow.
    pub fn profile_to_states_saved(&self) -> &BTreeMap<Profile, Option<StatesSaved>> {
        &self.profile_to_states_saved
    }

    /// Returns the type registry for each item spec's [`Params`].
    ///
    /// This is used to deserialize [`ItemSpecParamsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ItemSpecParamsFile`]: peace_resources::paths::ItemSpecParamsFile
    pub fn item_spec_params_type_reg(&self) -> &ItemSpecParamsTypeReg {
        &self.item_spec_params_type_reg
    }

    /// Returns the item spec params for each profile for the selected flow.
    pub fn profile_to_item_spec_params(&self) -> &BTreeMap<Profile, Option<ItemSpecParams>> {
        &self.profile_to_item_spec_params
    }

    /// Returns the type registry for each item spec's `State`.
    ///
    /// This is used to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    pub fn states_type_reg(&self) -> &StatesTypeReg {
        &self.states_type_reg
    }

    /// Returns a reference to the `Resources` for flow execution.
    pub fn resources(&self) -> &Resources<TS> {
        &self.resources
    }

    /// Returns a reference to the `Resources` for flow execution.
    pub fn resources_mut(&mut self) -> &mut Resources<TS> {
        &mut self.resources
    }
}

impl<'ctx, E, O, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe, TS>
    MultiProfileSingleFlow<
        'ctx,
        E,
        O,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
        TS,
    >
where
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns the workspace params.
    pub fn workspace_params(&self) -> &WorkspaceParams<WorkspaceParamsK> {
        &self.workspace_params
    }
}

impl<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsK, FlowParamsKMaybe, TS>
    MultiProfileSingleFlow<
        'ctx,
        E,
        O,
        ParamsKeysImpl<WorkspaceParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
        TS,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns the profile params for each profile.
    pub fn profile_to_profile_params(&self) -> &BTreeMap<Profile, ProfileParams<ProfileParamsK>> {
        &self.profile_to_profile_params
    }
}

impl<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsK, TS>
    MultiProfileSingleFlow<
        'ctx,
        E,
        O,
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
        TS,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Returns the flow params for the selected flow for each profile.
    pub fn profile_to_flow_params(&self) -> &BTreeMap<Profile, FlowParams<FlowParamsK>> {
        &self.profile_to_flow_params
    }
}
