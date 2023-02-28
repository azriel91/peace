use std::{collections::BTreeMap, fmt::Debug, hash::Hash};

use peace_core::Profile;
use peace_resources::{
    paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    states::StatesSaved,
};
use peace_rt_model::{
    cmd_context_params::{
        FlowParams, KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, ProfileParams, WorkspaceParams,
    },
    Flow, StatesTypeRegs,
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
pub struct MultiProfileSingleFlow<E, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// The profiles that are accessible by this command.
    profiles: Vec<Profile>,
    /// Profile directories that store params and flows.
    profile_dirs: BTreeMap<Profile, ProfileDir>,
    /// Directories of each profile's execution history.
    profile_history_dirs: BTreeMap<Profile, ProfileHistoryDir>,
    /// The chosen process flow.
    flow: Flow<E>,
    /// Flow directory that stores params and states.
    flow_dirs: BTreeMap<Profile, FlowDir>,
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    profile_to_profile_params:
        BTreeMap<Profile, ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>>,
    /// Flow params for the selected flow.
    profile_to_flow_params:
        BTreeMap<Profile, FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>>,
    /// Type registries to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    states_type_regs: StatesTypeRegs,
    /// Saved states for each profile for the selected flow.
    profile_to_states_saved: BTreeMap<Profile, Option<StatesSaved>>,
}

impl<E, PKeys> MultiProfileSingleFlow<E, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `MultiProfileSingleFlow` scope.
    #[allow(clippy::too_many_arguments)] // Constructed by proc macro
    pub(crate) fn new(
        profiles: Vec<Profile>,
        profile_dirs: BTreeMap<Profile, ProfileDir>,
        profile_history_dirs: BTreeMap<Profile, ProfileHistoryDir>,
        flow: Flow<E>,
        flow_dirs: BTreeMap<Profile, FlowDir>,
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        profile_to_profile_params: BTreeMap<
            Profile,
            ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
        >,
        profile_to_flow_params: BTreeMap<
            Profile,
            FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
        >,
        states_type_regs: StatesTypeRegs,
        profile_to_states_saved: BTreeMap<Profile, Option<StatesSaved>>,
    ) -> Self {
        Self {
            profiles,
            profile_dirs,
            profile_history_dirs,
            flow,
            flow_dirs,
            workspace_params,
            profile_to_profile_params,
            profile_to_flow_params,
            states_type_regs,
            profile_to_states_saved,
        }
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
        &self.flow
    }

    /// Returns the flow directories keyed by each profile.
    pub fn flow_dirs(&self) -> &BTreeMap<Profile, FlowDir> {
        &self.flow_dirs
    }

    /// Returns the type registries to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    pub fn states_type_regs(&self) -> &StatesTypeRegs {
        &self.states_type_regs
    }

    /// Returns the saved states for each profile for the selected flow.
    pub fn profile_to_states_saved(&self) -> &BTreeMap<Profile, Option<StatesSaved>> {
        &self.profile_to_states_saved
    }
}

impl<E, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    MultiProfileSingleFlow<
        E,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
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

impl<E, WorkspaceParamsKMaybe, ProfileParamsK, FlowParamsKMaybe>
    MultiProfileSingleFlow<
        E,
        ParamsKeysImpl<WorkspaceParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
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

impl<E, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsK>
    MultiProfileSingleFlow<
        E,
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
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
