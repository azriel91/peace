use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use peace::rt_model::output::OutputFormat;
use url::Url;

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "Downloads a file",
    long_about = "Downloads a file from a URL only if the local copy is out of sync with the remote copy."
)]
pub struct DownloadArgs {
    /// Command to run.
    #[clap(subcommand)]
    pub command: DownloadCommand,
    /// Whether to output errors verbosely.
    #[clap(short, long)]
    pub verbose: bool,
    /// The format of the command output.
    ///
    /// At this level, this needs to be specified before the subcommand.
    /// <https://github.com/clap-rs/clap/issues/3002> needs to be implemented
    /// for the argument to be passed in after the subcommand.
    #[clap(long)]
    pub format: Option<OutputFormat>,
}

#[derive(Subcommand)]
pub enum DownloadCommand {
    Init {
        /// URL to download from.
        #[clap(value_hint(ValueHint::Url))]
        url: Url,
        /// File path to write to.
        #[clap(value_parser)]
        dest: PathBuf,
    },
    /// Fetches the current state and desired state.
    Fetch,
    /// Shows the download state.
    Status,
    /// Shows the download state.
    Desired,
    /// Shows the diff between the remote file and local file state.
    ///
    /// This may not be the full content diff if the file is large.
    Diff,
    /// Dry-run to execute the download if necessary.
    EnsureDry,
    /// Executes the download if necessary.
    Ensure,
    /// Dry-run to clean the downloaded file.
    CleanDry,
    /// Cleans the downloaded file.
    Clean,
}
