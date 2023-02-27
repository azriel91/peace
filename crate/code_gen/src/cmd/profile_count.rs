/// Number of profiles accessed by this command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileCount {
    /// No profiles are accessed.
    None,
    /// One profile is accessed.
    One,
    /// Multiple profiles are accessed.
    Multiple,
}
