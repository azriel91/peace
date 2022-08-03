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
    /// Shows the download state.
    Status {
        /// URL to download from.
        #[clap(value_hint(ValueHint::Url))]
        url: Url,
        /// File path to write to.
        #[clap(value_parser)]
        dest: PathBuf,
    },
    /// Shows the download state.
    Desired {
        /// URL to download from.
        #[clap(value_hint(ValueHint::Url))]
        url: Url,
        /// File path to write to.
        #[clap(value_parser)]
        dest: PathBuf,
    },
    /// Shows the diff between the remote file and local file state.
    ///
    /// This may not be the full content diff if the file is large.
    Diff {
        /// URL to download from.
        #[clap(value_hint(ValueHint::Url))]
        url: Url,
        /// File path to write to.
        #[clap(value_parser)]
        dest: PathBuf,
    },
    /// Executes the download if necessary.
    Ensure {
        /// URL to download from.
        #[clap(value_hint(ValueHint::Url))]
        url: Url,
        /// File path to write to.
        #[clap(value_parser)]
        dest: PathBuf,
    },
}
