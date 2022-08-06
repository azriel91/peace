/// Error while managing a file download.
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
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

    // Framework / scaffolding errors
    #[error("Failed to initialize tokio runtime.")]
    TokioRuntimeInit(#[source] std::io::Error),
    #[error("Failed to serialize states.")]
    StatesSerialize(#[source] serde_yaml::Error),
    #[error("Failed to serialize desired states.")]
    StatesDesiredSerialize(#[source] serde_yaml::Error),
    #[error("Failed to serialize state diffs.")]
    StateDiffsSerialize(#[source] serde_yaml::Error),
    #[error("Failed to initialize tokio runtime.")]
    StdoutWrite(#[source] std::io::Error),

    // WASM errors.
    #[cfg(target_arch = "wasm32")]
    #[error("Failed to read text from contents.")]
    ResponseTextRead(#[source] reqwest::Error),
}
