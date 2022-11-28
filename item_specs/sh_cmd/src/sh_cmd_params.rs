use std::{fmt, marker::PhantomData};

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
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShCmdParams<Id> {
    /// Shell command to run to discover the current state.
    ///
    /// The command's stdout is used as the current state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    state_current_sh_cmd: ShCmd,
    /// Shell command to run to discover the desired state.
    ///
    /// The command's stdout is used as the desired state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    state_desired_sh_cmd: ShCmd,
    /// Shell command to run to show the state difference.
    ///
    /// The command will be passed the following as two separate arguments:
    ///
    /// * Current state string
    /// * Desired state string
    ///
    /// The command's stdout is used as the state difference.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state difference. This must be output as a single line.
    state_diff_sh_cmd: ShCmd,
    /// Shell command to run in `EnsureOpSpec::check`.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Desired state string
    /// * State diff string
    ///
    /// If the shell command returns the string `true` as its final line, then
    /// it is taken to mean `EnsureOpSpec::exec` needs to be run.
    ///
    /// If the shell command returns the string `false` as its final line, then
    /// it is taken to mean `EnsureOpSpec::exec` does not need to be run.
    ensure_check_sh_cmd: ShCmd,
    /// Shell command to run in `EnsureOpSpec::exec`.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Desired state string
    /// * State diff string
    ensure_exec_sh_cmd: ShCmd,
    /// Shell command to run in `CleanOpSpec::check`.
    ///
    /// The command will be passed the following as an argument:
    ///
    /// * Current state string
    ///
    /// If the shell command returns the string `true` as its final line, then
    /// it is taken to mean `CleanOpSpec::exec` needs to be run.
    ///
    /// If the shell command returns the string `false` as its final line, then
    /// it is taken to mean `CleanOpSpec::exec` does not need to be run.
    clean_check_sh_cmd: ShCmd,
    /// Shell command to run in `CleanOpSpec::exec`.
    ///
    /// The command will be passed the following as an argument:
    ///
    /// * Current state string
    clean_exec_sh_cmd: ShCmd,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> ShCmdParams<Id> {
    /// Returns new `ShCmdParams`.
    pub fn new(
        state_current_sh_cmd: ShCmd,
        state_desired_sh_cmd: ShCmd,
        state_diff_sh_cmd: ShCmd,
        ensure_check_sh_cmd: ShCmd,
        ensure_exec_sh_cmd: ShCmd,
        clean_check_sh_cmd: ShCmd,
        clean_exec_sh_cmd: ShCmd,
    ) -> Self {
        Self {
            state_current_sh_cmd,
            state_desired_sh_cmd,
            state_diff_sh_cmd,
            ensure_check_sh_cmd,
            ensure_exec_sh_cmd,
            clean_check_sh_cmd,
            clean_exec_sh_cmd,
            marker: PhantomData,
        }
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

    /// Returns the shell command to run to discover the desired state.
    ///
    /// The command's stdout is used as the desired state.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state. This must be output as a single line.
    pub fn state_desired_sh_cmd(&self) -> &ShCmd {
        &self.state_desired_sh_cmd
    }

    /// Returns the shell command to run to show the state difference.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Desired state string
    ///
    /// The command's stdout is used as the state difference.
    ///
    /// The command's stderr is used as the human readable description of the
    /// state difference. This must be output as a single line.
    pub fn state_diff_sh_cmd(&self) -> &ShCmd {
        &self.state_diff_sh_cmd
    }

    /// Returns the shell command to run in `EnsureOpSpec::check`.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Desired state string
    /// * State diff string
    ///
    /// If the shell command returns the string `true` as its final line, then
    /// it is taken to mean `EnsureOpSpec::exec` needs to be run.
    ///
    /// If the shell command returns the string `false` as its final line, then
    /// it is taken to mean `EnsureOpSpec::exec` does not need to be run.
    pub fn ensure_check_sh_cmd(&self) -> &ShCmd {
        &self.ensure_check_sh_cmd
    }

    /// Returns the shell command to run in `EnsureOpSpec::exec`.
    ///
    /// The command will be passed the following as three separate arguments:
    ///
    /// * Current state string
    /// * Desired state string
    /// * State diff string
    pub fn ensure_exec_sh_cmd(&self) -> &ShCmd {
        &self.ensure_exec_sh_cmd
    }

    /// Returns the shell command to run in `CleanOpSpec::check`.
    ///
    /// The command will be passed the following as an argument:
    ///
    /// * Current state string
    ///
    /// If the shell command returns the string `true` as its final line, then
    /// it is taken to mean `CleanOpSpec::exec` needs to be run.
    ///
    /// If the shell command returns the string `false` as its final line, then
    /// it is taken to mean `CleanOpSpec::exec` does not need to be run.
    pub fn clean_check_sh_cmd(&self) -> &ShCmd {
        &self.clean_check_sh_cmd
    }

    /// Returns the shell command to run in `CleanOpSpec::exec`.
    ///
    /// The command will be passed the following as an argument:
    ///
    /// * Current state string
    pub fn clean_exec_sh_cmd(&self) -> &ShCmd {
        &self.clean_exec_sh_cmd
    }
}

impl<Id> fmt::Debug for ShCmdParams<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShCmdParams")
            .field("state_current_sh_cmd", &self.state_current_sh_cmd)
            .field("state_desired_sh_cmd", &self.state_desired_sh_cmd)
            .field("state_diff_sh_cmd", &self.state_diff_sh_cmd)
            .field("ensure_check_sh_cmd", &self.ensure_check_sh_cmd)
            .field("ensure_exec_sh_cmd", &self.ensure_exec_sh_cmd)
            .field("clean_check_sh_cmd", &self.clean_check_sh_cmd)
            .field("clean_exec_sh_cmd", &self.clean_exec_sh_cmd)
            .field("marker", &self.marker)
            .finish()
    }
}
