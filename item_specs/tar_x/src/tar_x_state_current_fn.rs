use std::{marker::PhantomData, path::Path};

use peace::cfg::OpCtx;

use crate::{FileMetadata, FileMetadatas, TarXData, TarXError};

/// Reads the current state of the tar to extract.
#[derive(Debug)]
pub struct TarXStateCurrentFn<Id>(PhantomData<Id>);

impl<Id> TarXStateCurrentFn<Id>
where
    Id: Send + Sync,
{
    pub async fn state_current(
        op_ctx: OpCtx<'_>,
        data: TarXData<'_, Id>,
    ) -> Result<FileMetadatas, TarXError> {
        let tar_x_params = data.tar_x_params();
        let dest = tar_x_params.dest();

        #[cfg(not(target_arch = "wasm32"))]
        let files_extracted = Self::files_extracted(op_ctx, dest).await?;
        #[cfg(target_arch = "wasm32")]
        let files_extracted = Self::files_extracted(op_ctx, data.storage(), dest)?;

        let dest_files = FileMetadatas::from(files_extracted);

        let state = dest_files;

        Ok(state)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn files_extracted(
        _op_ctx: OpCtx<'_>,
        dest: &Path,
    ) -> Result<Vec<FileMetadata>, TarXError> {
        use std::time::UNIX_EPOCH;

        use futures::stream::TryStreamExt;

        use crate::native::{DestDirEntry, DirUnfold};

        let dest_file_metadatas = if dest.exists() {
            DirUnfold::unfold(dest)
                .try_fold(
                    Vec::new(),
                    |mut dest_file_metadatas, dest_dir_entry| async move {
                        let DestDirEntry {
                            dest_dir_relative_path,
                            dir_entry,
                        } = dest_dir_entry;
                        let entry_path = dir_entry.path();
                        let metadata = dir_entry.metadata().await.map_err(|error| {
                            Self::dest_metadata_read_error(
                                dest.to_path_buf(),
                                entry_path.clone(),
                                error,
                            )
                        })?;

                        let mtime = metadata
                            .modified()
                            .map_err(|error| {
                                Self::dest_mtime_read_error(
                                    dest.to_path_buf(),
                                    entry_path.clone(),
                                    error,
                                )
                            })
                            .and_then(|system_time| {
                                let mtime_secs = system_time
                                    .duration_since(UNIX_EPOCH)
                                    .map_err(|error| TarXError::TarDestFileMTimeSystemTimeRead {
                                        dest: dest.to_path_buf(),
                                        entry_path: entry_path.clone(),
                                        error,
                                    })?
                                    .as_secs();
                                Ok(mtime_secs)
                            })?;

                        let file_metadata = FileMetadata::new(dest_dir_relative_path, mtime);
                        dest_file_metadatas.push(file_metadata);

                        Ok(dest_file_metadatas)
                    },
                )
                .await?
        } else {
            Vec::new()
        };

        Ok(dest_file_metadatas)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn dest_metadata_read_error(
        dest: std::path::PathBuf,
        entry_path: std::path::PathBuf,
        error: std::io::Error,
    ) -> TarXError {
        TarXError::TarDestFileMetadataRead {
            dest,
            entry_path,
            error,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn dest_mtime_read_error(
        dest: std::path::PathBuf,
        entry_path: std::path::PathBuf,
        error: std::io::Error,
    ) -> TarXError {
        TarXError::TarDestFileMTimeRead {
            dest,
            entry_path,
            error,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn files_extracted(
        _op_ctx: OpCtx<'_>,
        _storage: &peace::rt_model::Storage,
        _dest: &Path,
    ) -> Result<Vec<FileMetadata>, TarXError> {
        todo!()
    }
}