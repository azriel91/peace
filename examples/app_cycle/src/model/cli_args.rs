use clap::{Parser, Subcommand, ValueHint};
use peace::{cfg::Profile, rt_model::output::OutputFormat};
use semver::Version;
use url::Url;

use crate::model::{EnvType, RepoSlug};

#[cfg(feature = "output_colorized")]
use peace::rt_model::output::CliColorizeOpt;

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
    /// Whether output should be colorized.
    ///
    /// * "auto" (default): Colorize when used interactively.
    /// * "always": Always colorize output.
    /// * "never": Never colorize output.
    #[cfg(feature = "output_colorized")]
    #[clap(long, default_value = "auto")]
    pub color: CliColorizeOpt,
}

#[derive(Subcommand)]
pub enum AppCycleCommand {
    /// Downloads the web application to run.
    Init {
        /// Username and repository of the application to download.
        slug: RepoSlug,
        /// Version of the application to download.
        version: Version,
        /// URL to override the default download URL.
        #[clap(long, value_hint(ValueHint::Url))]
        url: Option<Url>,
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
        #[arg(long = "name")]
        profile: Profile,
        /// Type of the profile's deployed environment.
        #[arg(short, long)]
        r#type: EnvType,
    },
    /// Lists available profiles.
    List,
    /// Shows the current profile, if any.
    Show,
}
