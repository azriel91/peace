use std::fmt;

/// Command variants which take in scripts in `ShCmdParams`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdVariant {
    /// The `state_clean` command.
    StateClean,
    /// The `state_current` command.
    StateCurrent,
    /// The `state_goal` command.
    StateGoal,
    /// The `state_diff` command.
    StateDiff,
    /// The `apply_check` command.
    ApplyCheck,
    /// The `apply_exec` command.
    ApplyExec,
}

impl fmt::Display for CmdVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StateClean => "state_clean".fmt(f),
            Self::StateCurrent => "state_current".fmt(f),
            Self::StateGoal => "state_goal".fmt(f),
            Self::StateDiff => "state_diff".fmt(f),
            Self::ApplyCheck => "apply_check".fmt(f),
            Self::ApplyExec => "apply_exec".fmt(f),
        }
    }
}
