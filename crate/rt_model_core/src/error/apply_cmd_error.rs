use std::{fmt, fmt::Write};

use crate::{ItemsStateStoredStale, StateStoredAndDiscovered};

/// Error applying changes to items.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum ApplyCmdError {
    /// Stored current states were not up to date with actual current states.
    #[error(
        "Stored current states were not up to date with actual current states.\n\n{stale_states}",
        stale_states = stale_states_fmt(items_state_stored_stale)?,
    )]
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
    StatesCurrentOutOfSync {
        /// Items whose stored current state is out of sync with the discovered
        /// state.
        items_state_stored_stale: ItemsStateStoredStale,
    },

    /// Stored goal states were not up to date with actual goal states.
    #[error(
        "Stored goal states were not up to date with actual goal states.\n\n{stale_states}",
        stale_states = stale_states_fmt(items_state_stored_stale)?,
    )]
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
    StatesGoalOutOfSync {
        /// Items whose stored goal state is out of sync with the discovered
        /// state.
        items_state_stored_stale: ItemsStateStoredStale,
    },
}

fn stale_states_fmt(
    items_state_stored_stale: &ItemsStateStoredStale,
) -> Result<String, fmt::Error> {
    let mut buffer = String::with_capacity(items_state_stored_stale.len() * 256);
    items_state_stored_stale
        .iter()
        .try_for_each(|(item_id, state_stored_and_discovered)| {
            writeln!(&mut buffer, "* {item_id}:\n")?;

            match state_stored_and_discovered {
                StateStoredAndDiscovered::OnlyStoredExists { state_stored } => {
                    writeln!(&mut buffer, "    - stored: {state_stored}")?;
                    writeln!(&mut buffer, "    - discovered: <none>\n")?;
                }
                StateStoredAndDiscovered::OnlyDiscoveredExists { state_discovered } => {
                    writeln!(&mut buffer, "    - stored: <none>")?;
                    writeln!(&mut buffer, "    - discovered: {state_discovered}\n")?;
                }
                StateStoredAndDiscovered::ValuesDiffer {
                    state_stored,
                    state_discovered,
                } => {
                    writeln!(&mut buffer, "    - stored: {state_stored}")?;
                    writeln!(&mut buffer, "    - discovered: {state_discovered}\n")?;
                }
            }

            Ok(())
        })?;

    Ok(buffer)
}
