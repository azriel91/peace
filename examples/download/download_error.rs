/// Error while managing a file download.
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Source URL not initialized.")]
    SrcUrlInit,
    #[error("Destination file not initialized.")]
    DestFileInit,

    #[error("Failed to open destination file.")]
    DestFileOpen(#[source] std::io::Error),
    #[error("Failed to read destination file metadata.")]
    DestMetadataRead(#[source] std::io::Error),
    #[error("Failed to read destination file contents.")]
    DestFileRead(#[source] std::io::Error),
    #[error("Failed to open destination file for writing.")]
    DestFileCreate(#[source] std::io::Error),
    #[error("Failed to delete destination file.")]
    DestFileRemove(#[source] std::io::Error),
    #[error("Failed to parse source URL.")]
    SrcUrlParse(url::ParseError),
    #[error("Failed to parse source URL.")]
    SrcGet(#[source] reqwest::Error),
    #[error("Failed to fetch source file metadata. Response status code: {status_code}")]
    SrcFileUndetermined { status_code: reqwest::StatusCode },
    #[error("Failed to read source file content.")]
    SrcFileRead(#[source] reqwest::Error),
    #[error("Failed to stream source file content.")]
    ResponseBytesStream(#[source] reqwest::Error),
    #[error("Failed to transfer source file content.")]
    ResponseFileWrite(#[source] std::io::Error),
}
