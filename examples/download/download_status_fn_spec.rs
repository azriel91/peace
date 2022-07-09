use std::path::PathBuf;

use peace::cfg::{async_trait, nougat, FnSpec, FnSpecඞData, State};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{DownloadError, DownloadParams, FileState};

/// Status `FnSpec` for the file to download.
#[derive(Debug)]
pub struct DownloadStatusFnSpec;

#[async_trait]
#[nougat::gat]
impl FnSpec for DownloadStatusFnSpec {
    type Data<'op> = DownloadParams<'op>
        where Self: 'op;
    type Error = DownloadError;
    type Output = State<Option<FileState>, PathBuf>;

    async fn exec(download_params: DownloadParams<'_>) -> Result<Self::Output, DownloadError> {
        // Destination file doesn't exist.
        let dest = download_params.dest().ok_or(DownloadError::DestFileInit)?;
        if !dest.exists() {
            return Ok(State::new(None, dest.to_path_buf()));
        }

        // Check file length
        let mut file = File::open(dest)
            .await
            .map_err(DownloadError::DestFileOpen)?;
        let metadata = file
            .metadata()
            .await
            .map_err(DownloadError::DestMetadataRead)?;

        let file_state = if metadata.len() > crate::IN_MEMORY_CONTENTS_MAX {
            Some(FileState::Length(metadata.len()))
        } else {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .await
                .map_err(DownloadError::DestFileRead)?;
            Some(FileState::StringContents(buffer))
        };

        Ok(State::new(file_state, dest.to_path_buf()))
    }
}
