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
    #[clap(subcommand)]
    pub command: DownloadCommand,
    // #[clap(value_hint(ValueHint::Url))]
    // url: Url,
    // #[clap(value_parser)]
    // output: PathBuf,
}

#[derive(Subcommand)]
pub enum DownloadCommand {
    /// Shows the download state.
    Status,
    /// Shows the download state.
    Desired,
    /// Shows the diff between the remote file and local file state.
    ///
    /// This may not be the full content diff if the file is large.
    Diff,
    /// Executes the download if necessary.
    Ensure,
}
