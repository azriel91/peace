use std::marker::PhantomData;

use derivative::Derivative;
use peace::params::Params;
use serde::{Deserialize, Serialize};

use crate::ShCmd;

/// Grouping of commands to run a shell command idempotently.
///
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Derivative, Params, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Clone, Debug)]
#[serde(bound = "")]
pub struct ShCmdParams<Id> {
    /// Shell command to run to discover the example state.
    ///
    /// The command's stdout is used as the example state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    #[cfg(feature = "item_state_example")]
    state_example_sh_cmd: ShCmd,
    /// Shell command to run to discover the clean state.
    ///
    /// The command's stdout is used as the clean state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    state_clean_sh_cmd: ShCmd,
    /// Shell command to run to discover the current state.
    ///
    /// The command's stdout is used as the current state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    state_current_sh_cmd: ShCmd,
    /// Shell command to run to discover the goal state.
    ///
    /// The command's stdout is used as the goal state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    state_goal_sh_cmd: ShCmd,
    /// Shell command to run to show the state difference.
    ///
    /// The command will be passed the following as two separate arguments:
    ///
    /// * Current state string
    /// * Goal state string
    ///
    /// The command's stdout is used as the state difference.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state difference. This must be output as a single line.
    state_diff_sh_cmd: ShCmd,
    /// Shell command to run in `ApplyFns::check`.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Goal state string
    /// * State diff string
    ///
    /// If the shell command returns the string `true` as its final line, then
    /// it is taken to mean `ApplyFns::exec` needs to be run.
    ///
    /// If the shell command returns the string `false` as its final line, then
    /// it is taken to mean `ApplyFns::exec` does not need to be run.
    apply_check_sh_cmd: ShCmd,
    /// Shell command to run in `ApplyFns::exec`.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Goal state string
    /// * State diff string
    apply_exec_sh_cmd: ShCmd,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> ShCmdParams<Id> {
    /// Returns new `ShCmdParams`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        #[cfg(feature = "item_state_example")] state_example_sh_cmd: ShCmd,
        state_clean_sh_cmd: ShCmd,
        state_current_sh_cmd: ShCmd,
        state_goal_sh_cmd: ShCmd,
        state_diff_sh_cmd: ShCmd,
        apply_check_sh_cmd: ShCmd,
        apply_exec_sh_cmd: ShCmd,
    ) -> Self {
        Self {
            #[cfg(feature = "item_state_example")]
            state_example_sh_cmd,
            state_clean_sh_cmd,
            state_current_sh_cmd,
            state_goal_sh_cmd,
            state_diff_sh_cmd,
            apply_check_sh_cmd,
            apply_exec_sh_cmd,
            marker: PhantomData,
        }
    }

    /// Returns the shell command to run to discover the example state.
    ///
    /// The command's stdout is used as the example state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    #[cfg(feature = "item_state_example")]
    pub fn state_example_sh_cmd(&self) -> &ShCmd {
        &self.state_example_sh_cmd
    }

    /// Returns the shell command to run to discover the clean state.
    ///
    /// The command's stdout is used as the clean state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    pub fn state_clean_sh_cmd(&self) -> &ShCmd {
        &self.state_clean_sh_cmd
    }

    /// Returns the shell command to run to discover the current state.
    ///
    /// The command's stdout is used as the current state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    pub fn state_current_sh_cmd(&self) -> &ShCmd {
        &self.state_current_sh_cmd
    }

    /// Returns the shell command to run to discover the goal state.
    ///
    /// The command's stdout is used as the goal state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    pub fn state_goal_sh_cmd(&self) -> &ShCmd {
        &self.state_goal_sh_cmd
    }

    /// Returns the shell command to run to show the state difference.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Goal state string
    ///
    /// The command's stdout is used as the state difference.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state difference. This must be output as a single line.
    pub fn state_diff_sh_cmd(&self) -> &ShCmd {
        &self.state_diff_sh_cmd
    }

    /// Returns the shell command to run in `ApplyFns::check`.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Goal state string
    /// * State diff string
    ///
    /// If the shell command returns the string `true` as its final line, then
    /// it is taken to mean `ApplyFns::exec` needs to be run.
    ///
    /// If the shell command returns the string `false` as its final line, then
    /// it is taken to mean `ApplyFns::exec` does not need to be run.
    pub fn apply_check_sh_cmd(&self) -> &ShCmd {
        &self.apply_check_sh_cmd
    }

    /// Returns the shell command to run in `ApplyFns::exec`.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Goal state string
    /// * State diff string
    pub fn apply_exec_sh_cmd(&self) -> &ShCmd {
        &self.apply_exec_sh_cmd
    }
}
