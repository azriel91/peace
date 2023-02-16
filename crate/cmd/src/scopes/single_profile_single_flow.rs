use peace_core::{FlowId, Profile};

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
#[derive(Clone, Debug)]
pub struct SingleProfileSingleFlow {
    /// The profile this command operates on.
    profile: Profile,
    /// Identifier or name of the chosen process flow.
    flow_id: FlowId,
}

impl SingleProfileSingleFlow {
    /// Returns a new `SingleProfileSingleFlow` scope.
    pub fn new(profile: Profile, flow_id: FlowId) -> Self {
        Self { profile, flow_id }
    }

    /// Returns a reference to the `Profile`.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Returns a reference to the flow ID.
    pub fn flow_id(&self) -> &FlowId {
        &self.flow_id
    }
}
