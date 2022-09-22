use std::path::PathBuf;

use clap::{AppSettings, Parser, Subcommand, ValueHint};
use url::Url;

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "Downloads a file",
    long_about = "Downloads a file from a URL only if the local copy is out of sync with the remote copy."
)] // Read from `Cargo.toml`
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct DownloadArgs {
    /// Command to run.
    #[clap(subcommand)]
    pub command: DownloadCommand,
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
}
