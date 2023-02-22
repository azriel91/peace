use peace_core::Profile;
use peace_resources::paths::{ProfileDir, ProfileHistoryDir};

/// A command that works with a single profile, without any item specs.
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
///   `MultiProfileNoFlow`.
///
/// This kind of command cannot:
///
/// * Read or write flow parameters -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
/// * Read or write flow state -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleProfileNoFlow {
    /// The profile this command operates on.
    profile: Profile,
    /// Profile directory that stores params and flows.
    profile_dir: ProfileDir,
    /// Directory to store profile executions' summaries.
    profile_history_dir: ProfileHistoryDir,
}

impl SingleProfileNoFlow {
    /// Returns a new `SingleProfileNoFlow` scope.
    pub fn new(
        profile: Profile,
        profile_dir: ProfileDir,
        profile_history_dir: ProfileHistoryDir,
    ) -> Self {
        Self {
            profile,
            profile_dir,
            profile_history_dir,
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
}
