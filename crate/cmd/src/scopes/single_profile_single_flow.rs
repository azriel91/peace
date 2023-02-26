use std::{fmt::Debug, hash::Hash};

use peace_core::Profile;
use peace_resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir};
use peace_rt_model::{
    cmd_context_params::{
        FlowParams, KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, ProfileParams, WorkspaceParams,
    },
    Flow,
};
use serde::{de::DeserializeOwned, Serialize};

/// A command that works with one profile and one flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a
/// |   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
/// |   |
/// |   |- 🌊 deploy                   # ✅ can read `FlowId`
/// |   |   |- 📝 flow_params.yaml     # ✅ can read or write `FlowParams`
/// |   |   |- 📋 states_desired.yaml  # ✅ can read or write `StatesDesired`
/// |   |   |- 📋 states_saved.yaml    # ✅ can read or write `StatesSaved`
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
///   `MultiProfileNoFlow`.
///
/// This kind of command cannot:
///
/// * Read or write flow parameters -- see `MultiProfileNoFlow`.
/// * Read or write flow state -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Debug)]
pub struct SingleProfileSingleFlow<E, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// The profile this command operates on.
    profile: Profile,
    /// Profile directory that stores params and flows.
    profile_dir: ProfileDir,
    /// Directory to store profile execution history.
    profile_history_dir: ProfileHistoryDir,
    /// The chosen process flow.
    flow: Flow<E>,
    /// Flow directory that stores params and states.
    flow_dir: FlowDir,
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    profile_params: ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
    /// Flow params for the selected flow.
    flow_params: FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
}

impl<E, PKeys> SingleProfileSingleFlow<E, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `SingleProfileSingleFlow` scope.
    #[allow(clippy::too_many_arguments)] // Constructed by proc macro
    pub(crate) fn new(
        profile: Profile,
        profile_dir: ProfileDir,
        profile_history_dir: ProfileHistoryDir,
        flow: Flow<E>,
        flow_dir: FlowDir,
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        profile_params: ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
        flow_params: FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
    ) -> Self {
        Self {
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            workspace_params,
            profile_params,
            flow_params,
        }
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
    pub fn flow(&self) -> &Flow<E> {
        &self.flow
    }

    /// Returns a reference to the flow directory.
    pub fn flow_dir(&self) -> &FlowDir {
        &self.flow_dir
    }
}

impl<E, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    SingleProfileSingleFlow<
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
    SingleProfileSingleFlow<
        E,
        ParamsKeysImpl<WorkspaceParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns the profile params.
    pub fn profile_params(&self) -> &ProfileParams<ProfileParamsK> {
        &self.profile_params
    }
}

impl<E, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsK>
    SingleProfileSingleFlow<
        E,
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Returns the flow params for the selected flow.
    pub fn flow_params(&self) -> &FlowParams<FlowParamsK> {
        &self.flow_params
    }
}
