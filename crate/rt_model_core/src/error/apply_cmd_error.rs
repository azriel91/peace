/// Error applying changes to items.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum ApplyCmdError {
    /// Stored current states were not up to date with actual current states.
    #[error("Stored current states were not up to date with actual current states.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::apply_cmd_error::states_current_out_of_sync),
            help(
                "\
                Run `StatesDiscoverCmd::current` to update the stored current states,\n\
                and re-check the difference before applying changes.\
                "
            ),
        )
    )]
    StatesCurrentOutOfSync,

    /// Stored goal states were not up to date with actual goal states.
    #[error("Stored goal states were not up to date with actual goal states.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::apply_cmd_error::states_goal_out_of_sync),
            help(
                "\
                Run `StatesDiscoverCmd::goal` to update the stored goal states,\n\
                and re-check the difference before applying changes.\
                "
            ),
        )
    )]
    StatesGoalOutOfSync,
}
