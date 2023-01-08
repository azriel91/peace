use clap::{Parser, Subcommand};
use peace::{cfg::Profile, rt_model::output::OutputFormat};
use semver::Version;

use crate::model::{EnvType, RepoSlug};

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "Manages an application's lifecycle.",
    long_about = "Manages deployment, configuration, upgrading, and cleaning up of an application."
)]
pub struct CliArgs {
    /// Command to run.
    #[command(subcommand)]
    pub command: AppCycleCommand,
    /// The format of the command output.
    ///
    /// At this level, this needs to be specified before the subcommand.
    /// <https://github.com/clap-rs/clap/issues/3002> needs to be implemented
    /// for the argument to be passed in after the subcommand.
    #[clap(long)]
    pub format: Option<OutputFormat>,
}

#[derive(Subcommand)]
pub enum AppCycleCommand {
    /// Downloads the web application to run.
    Init {
        /// Username and repository of the application to download.
        slug: RepoSlug,
        /// Version of the application to download.
        version: Version,
    },
    /// Shows or initializes the current profile.
    Profile {
        /// Profile command to run.
        #[command(subcommand)]
        command: ProfileCommand,
    },
    /// Switches the current profile.
    ///
    /// Similar to changing the branch in git.
    Switch {
        /// Profile name to switch to.
        profile: Profile,
    },
    /// Fetches the state of the environment.
    Fetch,
    /// Shows the state of the environment.
    Status,
    /// Shows the desired state of the environment.
    Desired,
    /// Shows the diff between the current and desired states of the
    /// environment.
    Diff,
    /// Pushes changes to the environment.
    Push,
    /// Pulls changes from the environment.
    Pull,
    /// Cleans the current environment.
    Clean,
}

#[derive(Subcommand)]
pub enum ProfileCommand {
    /// Initializes a new profile.
    Init {
        /// Name to use for the profile.
        #[arg(long)]
        name: Profile,
        /// Type of the profile's deployed environment.
        #[arg(short, long)]
        r#type: EnvType,
    },
    /// Lists available profiles.
    List,
    /// Shows the current profile, if any.
    Show,
}
