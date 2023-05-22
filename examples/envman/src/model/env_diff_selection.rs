use peace::cfg::Profile;

/// What to run the diff on.
///
/// Of note is, this is for showing diffs between flow states, but what is
/// valuable to the user may be any combination within the following groupings.
///
/// Within one profile:
///
/// * previous vs current workspace params
/// * previous vs current profile params
/// * previous vs current flow params
/// * previous vs current item params
/// * previous vs current state
/// * current vs desired state
///
/// Between two profiles:
///
/// * profile params
/// * flow params
/// * item params
/// * current states
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnvDiffSelection {
    /// Compare the current and desired states of the active profile.
    CurrentAndDesired,
    /// Compare the current states of the specified profiles.
    DiffProfilesCurrent {
        /// First profile in the comparison, the reference point.
        profile_a: Profile,
        /// Second profile in the comparison.
        profile_b: Profile,
    },
}
