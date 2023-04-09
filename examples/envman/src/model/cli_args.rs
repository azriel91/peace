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
    pub command: EnvManCommand,
    /// Whether to run with multiple threads.
    #[arg(long, default_value = "false")]
    pub fast: bool,
    /// The format of the command output.
    ///
    /// At this level, this needs to be specified before the subcommand.
    /// <https://github.com/clap-rs/clap/issues/3002> needs to be implemented
    /// for the argument to be passed in after the subcommand.
    #[arg(long)]
    pub format: Option<OutputFormat>,
    /// Whether output should be colorized.
    ///
    /// * "auto" (default): Colorize when used interactively.
    /// * "always": Always colorize output.
    /// * "never": Never colorize output.
    #[cfg(feature = "output_colorized")]
    #[arg(long, default_value = "auto")]
    pub color: CliColorizeOpt,
}

#[derive(Subcommand)]
pub enum EnvManCommand {
    /// Initializes a profile to deploy a web application.
    Init {
        /// Name to use for the profile.
        profile: Profile,
        /// Type of the profile's deployed environment.
        #[arg(short, long)]
        r#type: EnvType,
        /// Username and repository of the application to download.
        slug: RepoSlug,
        /// Version of the application to download.
        version: Version,
        /// URL to override the default download URL.
        #[arg(long, value_hint(ValueHint::Url))]
        url: Option<Url>,
    },
    /// Shows or initializes the current profile.
    Profile {
        /// Profile command to run.
        #[command(subcommand)]
        command: Option<ProfileCommand>,
    },
    /// Switches the current profile.
    ///
    /// Similar to changing the branch in git.
    Switch {
        /// Profile name to switch to.
        profile: Profile,
        /// Whether or not to create the profile.
        ///
        /// * If this flag is specified, and the profile already exists, then
        ///   the switch does not happen.
        /// * If this flag is not specified, and the profile does not exist,
        ///   then the switch does not happen.
        #[arg(short, long)]
        create: bool,
        /// Type of the profile's deployed environment.
        #[arg(short, long, required_if_eq("create", "true"))]
        r#type: Option<EnvType>,
        /// Username and repository of the application to download.
        #[arg(required_if_eq("create", "true"))]
        slug: Option<RepoSlug>,
        /// Version of the application to download.
        #[arg(required_if_eq("create", "true"))]
        version: Option<Version>,
        /// URL to override the default download URL.
        #[arg(short, long, value_hint(ValueHint::Url))]
        url: Option<Url>,
    },
    /// Discovers the state of the environment.
    Discover,
    /// Shows the state of the environment.
    Status,
    /// Shows the desired state of the environment.
    Desired,
    /// Shows the diff between states of the environment.
    ///
    /// By default, this compares the current and desired states of the active
    /// profile.
    ///
    /// Users may pass in two profiles to compare the current states of both
    /// profiles.
    Diff {
        /// First profile in the comparison.
        #[arg(requires("profile_b"))]
        profile_a: Option<Profile>,
        /// Second profile in the comparison.
        profile_b: Option<Profile>,
    },
    /// Deploys / updates the environment.
    Deploy,
    /// Cleans the current environment.
    Clean,
}

#[derive(Subcommand)]
pub enum ProfileCommand {
    /// Lists available profiles.
    List,
    /// Shows the current profile, if any.
    Show,
}
