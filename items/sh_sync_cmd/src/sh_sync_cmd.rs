use std::ffi::OsString;

use peace::params::Params;
use serde::{Deserialize, Serialize};

/// Shell command to execute.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Params)]
pub struct ShSyncCmd {
    /// Command to run.
    program: OsString,
    /// Arguments to pass to the command.
    args: Vec<OsString>,
}

impl ShSyncCmd {
    /// Constructs a new `ShSyncCmd` for launching the program at
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
    /// `PATH` environment variable on the ShSyncCmd,
    /// but this has some implementation limitations on Windows
    /// (see issue #37519).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use peace_item_sh_sync_cmd::ShSyncCmd;
    ///
    /// let sh_sync_cmd = ShSyncCmd::new("sh");
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
    /// # peace_item_sh_sync_cmd::ShSyncCmd::new("sh")
    /// .arg("-C /path/to/repo")
    /// # ;
    /// ```
    ///
    /// usage would be:
    ///
    /// ```rust,no_run
    /// # peace_item_sh_sync_cmd::ShSyncCmd::new("sh")
    /// .arg("-C")
    /// .arg("/path/to/repo")
    /// # ;
    /// ```
    ///
    /// To pass multiple arguments see [`args`].
    ///
    /// [`args`]: ShSyncCmd::args
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
    /// use peace_item_sh_sync_cmd::ShSyncCmd;
    ///
    /// let sh_sync_cmd = ShSyncCmd::new("ls").arg("-l").arg("-a");
    /// ```
    pub fn arg<S: Into<OsString>>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    /// Adds multiple arguments to pass to the program.
    ///
    /// To pass a single argument see [`arg`].
    ///
    /// [`arg`]: ShSyncCmd::arg
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
    /// use peace_item_sh_sync_cmd::ShSyncCmd;
    ///
    /// let sh_sync_cmd = ShSyncCmd::new("ls").args(["-l", "-a"]);
    /// ```
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
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
