use peace_core::Profile;

/// A command that works with multiple profiles, without any item specs.
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
/// * Read or write flow parameters -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
/// * Read or write flow state -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiProfileNoFlow {
    /// The profiles that are accessible by this command.
    profiles: Vec<Profile>,
}

impl MultiProfileNoFlow {
    pub fn new(profiles: Vec<Profile>) -> Self {
        Self { profiles }
    }

    /// Returns the accessible profiles.
    ///
    /// These are the profiles that are filtered by the filter function, if
    /// provided.
    pub fn profiles(&self) -> &[Profile] {
        self.profiles.as_ref()
    }
}
