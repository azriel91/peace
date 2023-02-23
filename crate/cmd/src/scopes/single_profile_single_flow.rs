use peace_core::{FlowId, Profile};
use peace_resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir};

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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleProfileSingleFlow<ProfileParamsSelection, FlowParamsSelection> {
    /// The profile this command operates on.
    profile: Profile,
    /// Profile directory that stores params and flows.
    profile_dir: ProfileDir,
    /// Directory to store profile execution history.
    profile_history_dir: ProfileHistoryDir,
    /// Profile params for the profile.
    profile_params_selection: ProfileParamsSelection,
    /// Identifier or name of the chosen process flow.
    flow_id: FlowId,
    /// Flow directory that stores params and states.
    flow_dir: FlowDir,
    /// Flow params for the selected flow.
    flow_params_selection: FlowParamsSelection,
}

impl<ProfileParamsSelection, FlowParamsSelection>
    SingleProfileSingleFlow<ProfileParamsSelection, FlowParamsSelection>
{
    /// Returns a new `SingleProfileSingleFlow` scope.
    pub fn new(
        profile: Profile,
        profile_dir: ProfileDir,
        profile_history_dir: ProfileHistoryDir,
        profile_params_selection: ProfileParamsSelection,
        flow_id: FlowId,
        flow_dir: FlowDir,
        flow_params_selection: FlowParamsSelection,
    ) -> Self {
        Self {
            profile,
            profile_dir,
            profile_history_dir,
            profile_params_selection,
            flow_id,
            flow_dir,
            flow_params_selection,
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

    /// Returns a reference to the flow ID.
    pub fn flow_id(&self) -> &FlowId {
        &self.flow_id
    }

    /// Returns a reference to the flow directory.
    pub fn flow_dir(&self) -> &FlowDir {
        &self.flow_dir
    }
}
