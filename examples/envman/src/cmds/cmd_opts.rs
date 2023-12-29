/// Options to configure a `Cmd`'s output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CmdOpts {
    /// Whether or not to print the active profile.
    pub profile_print: bool,
}

impl CmdOpts {
    /// Sets whether or not to print the active profile.
    pub fn with_profile_print(mut self, profile_print: bool) -> Self {
        self.profile_print = profile_print;
        self
    }
}

impl Default for CmdOpts {
    fn default() -> Self {
        Self {
            profile_print: true,
        }
    }
}
