use peace_cfg::Profile;

use crate::cmds::diff_cmd::DiffStateSpec;

/// Indicates where to source information to diff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffInfoSpec<'diff> {
    /// Profile to read parameters and state from.
    profile: &'diff Profile,
    /// Whether to read stored state or discover state.
    state_src: DiffStateSpec,
}
