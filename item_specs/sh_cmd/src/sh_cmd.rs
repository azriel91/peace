use std::{ffi::OsString, fmt};

use peace::params::ParamsSpec;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

/// Shell command to execute.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, ParamsSpec)]
pub struct ShCmd {
    /// Command to run.
    program: OsString,
    /// Arguments to pass to the command.
    args: Vec<OsString>,
}

impl ShCmd {
    /// Constructs a new `ShCmd` for launching the program at
    /// path `program`, with the following default configuration:
    ///
    /// * No arguments to the program
    /// * Inherit the current process's environment
    /// * Inherit the current process's working directory
    /// * Inherit stdin/stdout/stderr for [`spawn`] or [`status`], but create
    ///   pipes for [`output`]
    ///
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    ///
    /// Builder methods are provided to change these defaults and
    /// otherwise configure the process.
    ///
    /// If `program` is not an absolute path, the `PATH` will be searched in
    /// an OS-defined way.
    ///
    /// The search path to be used may be controlled by setting the
    /// `PATH` environment variable on the ShCmd,
    /// but this has some implementation limitations on Windows
    /// (see issue #37519).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use peace_item_spec_sh_cmd::ShCmd;
    ///
    /// let sh_cmd = ShCmd::new("sh");
    /// ```
    pub fn new<S: Into<OsString>>(program: S) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
        }
    }

    /// Adds an argument to pass to the program.
    ///
    /// Only one argument can be passed per use. So instead of:
    ///
    /// ```rust,no_run
    /// # peace_item_spec_sh_cmd::ShCmd::new("sh")
    /// .arg("-C /path/to/repo")
    /// # ;
    /// ```
    ///
    /// usage would be:
    ///
    /// ```rust,no_run
    /// # peace_item_spec_sh_cmd::ShCmd::new("sh")
    /// .arg("-C")
    /// .arg("/path/to/repo")
    /// # ;
    /// ```
    ///
    /// To pass multiple arguments see [`args`].
    ///
    /// [`args`]: ShCmd::args
    ///
    /// Note that the argument is not passed through a shell, but given
    /// literally to the program. This means that shell syntax like quotes,
    /// escaped characters, word splitting, glob patterns, substitution, etc.
    /// have no effect.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust,no_run
    /// use peace_item_spec_sh_cmd::ShCmd;
    ///
    /// let sh_cmd = ShCmd::new("ls").arg("-l").arg("-a");
    /// ```
    pub fn arg<S: Into<OsString>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Adds multiple arguments to pass to the program.
    ///
    /// To pass a single argument see [`arg`].
    ///
    /// [`arg`]: ShCmd::arg
    ///
    /// Note that the arguments are not passed through a shell, but given
    /// literally to the program. This means that shell syntax like quotes,
    /// escaped characters, word splitting, glob patterns, substitution, etc.
    /// have no effect.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust,no_run
    /// use peace_item_spec_sh_cmd::ShCmd;
    ///
    /// let sh_cmd = ShCmd::new("ls").args(["-l", "-a"]);
    /// ```
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        args.into_iter().for_each(|arg| {
            self.args.push(arg.into());
        });
        self
    }
}

impl From<&ShCmd> for Command {
    fn from(sh_cmd: &ShCmd) -> Command {
        let mut command = Command::new(&sh_cmd.program);
        command.args(&sh_cmd.args);

        command
    }
}

impl fmt::Display for ShCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.program.to_string_lossy().fmt(f)?;
        self.args
            .iter()
            .map(|arg| arg.to_string_lossy())
            .try_for_each(|arg| write!(f, " {arg}"))?;
        Ok(())
    }
}
