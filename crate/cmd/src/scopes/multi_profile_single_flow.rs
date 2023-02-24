use std::{fmt::Debug, hash::Hash};

use indexmap::IndexMap;
use peace_core::{FlowId, Profile};
use peace_resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir};
use peace_rt_model::cmd_context_params::{
    FlowParams, KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, ProfileParams, WorkspaceParams,
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
pub struct MultiProfileSingleFlow<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// The profiles that are accessible by this command.
    profiles: Vec<Profile>,
    /// Profile directories that store params and flows.
    profile_dirs: IndexMap<Profile, ProfileDir>,
    /// Directories of each profile's execution history.
    profile_history_dirs: IndexMap<Profile, ProfileHistoryDir>,
    /// Identifier or name of the chosen process flow.
    flow_id: FlowId,
    /// Flow directory that stores params and states.
    flow_dirs: IndexMap<Profile, FlowDir>,
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    profile_to_profile_params:
        IndexMap<Profile, ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>>,
    /// Flow params for the selected flow.
    profile_to_flow_params:
        IndexMap<Profile, FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>>,
}

impl<PKeys> MultiProfileSingleFlow<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `MultiProfileSingleFlow` scope.
    #[allow(clippy::too_many_arguments)] // Constructed by proc macro
    pub(crate) fn new(
        profiles: Vec<Profile>,
        profile_dirs: IndexMap<Profile, ProfileDir>,
        profile_history_dirs: IndexMap<Profile, ProfileHistoryDir>,
        flow_id: FlowId,
        flow_dirs: IndexMap<Profile, FlowDir>,
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        profile_to_profile_params: IndexMap<
            Profile,
            ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
        >,
        profile_to_flow_params: IndexMap<
            Profile,
            FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
        >,
    ) -> Self {
        Self {
            profiles,
            profile_dirs,
            profile_history_dirs,
            flow_id,
            flow_dirs,
            workspace_params,
            profile_to_profile_params,
            profile_to_flow_params,
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
    pub fn profile_dirs(&self) -> &IndexMap<Profile, ProfileDir> {
        &self.profile_dirs
    }

    /// Returns the profile history directories keyed by each profile.
    pub fn profile_history_dirs(&self) -> &IndexMap<Profile, ProfileHistoryDir> {
        &self.profile_history_dirs
    }

    /// Returns the flow ID.
    pub fn flow_id(&self) -> &FlowId {
        &self.flow_id
    }

    /// Returns the flow directories keyed by each profile.
    pub fn flow_dirs(&self) -> &IndexMap<Profile, FlowDir> {
        &self.flow_dirs
    }
}

impl<WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    MultiProfileSingleFlow<
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

impl<WorkspaceParamsKMaybe, ProfileParamsK, FlowParamsKMaybe>
    MultiProfileSingleFlow<
        ParamsKeysImpl<WorkspaceParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns the profile params for each profile.
    pub fn profile_to_profile_params(&self) -> &IndexMap<Profile, ProfileParams<ProfileParamsK>> {
        &self.profile_to_profile_params
    }
}

impl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsK>
    MultiProfileSingleFlow<
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Returns the flow params for the selected flow for each profile.
    pub fn profile_to_flow_params(&self) -> &IndexMap<Profile, FlowParams<FlowParamsK>> {
        &self.profile_to_flow_params
    }
}
