use std::{marker::PhantomData, path::Path};

use peace::cfg::{async_trait, state::Nothing, FnSpec, State};

use crate::{FileMetadata, FileMetadatas, TarXData, TarXError};

/// Status `FnSpec` for the tar to extract.
#[derive(Debug)]
pub struct TarXStateCurrentFnSpec<Id>(PhantomData<Id>);

impl<Id> TarXStateCurrentFnSpec<Id> {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn files_extracted(dest: &Path) -> Result<Vec<FileMetadata>, TarXError> {
        use std::time::UNIX_EPOCH;

        use futures::stream::TryStreamExt;

        let dest_file_metadatas = if dest.exists() {
            let read_dir = tokio::fs::read_dir(dest).await.map_err(|error| {
                let dest = dest.to_path_buf();
                TarXError::TarDestReadDir { dest, error }
            })?;

            // `ReadDir` doesn't implement `Stream`, this does that mapping.
            futures::stream::try_unfold(read_dir, move |mut read_dir| async move {
                read_dir
                    .next_entry()
                    .await
                    .map_err(|error| {
                        let dest = dest.to_path_buf();
                        TarXError::TarDestEntryRead { dest, error }
                    })
                    .map(|dir_entry| dir_entry.map(|dir_entry| (dir_entry, read_dir)))
            })
            .try_fold(
                Vec::new(),
                |mut dest_file_metadatas, dir_entry| async move {
                    let entry_path = dir_entry.path();
                    let mtime = dir_entry
                        .metadata()
                        .await
                        .map_err(|error| {
                            Self::dest_mtime_read_error(
                                dest.to_path_buf(),
                                entry_path.clone(),
                                error,
                            )
                        })?
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

                    let file_metadata = FileMetadata::new(entry_path, mtime);
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
    fn files_extracted(_storage: &Storage, _dest: &Path) -> Result<Vec<FileMetadata>, TarXError> {
        todo!()
    }
}

#[async_trait(?Send)]
impl<Id> FnSpec for TarXStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type Output = State<FileMetadatas, Nothing>;

    async fn exec(tar_x_data: TarXData<'_, Id>) -> Result<Self::Output, TarXError> {
        let tar_x_params = tar_x_data.tar_x_params();
        let tar_path = tar_x_params.tar_path();
        let dest = tar_x_params.dest();

        let tar_x_state = if tar_path.exists() {
            #[cfg(not(target_arch = "wasm32"))]
            let files_extracted = Self::files_extracted(dest).await?;
            #[cfg(target_arch = "wasm32")]
            let files_extracted = Self::files_extracted(storage, dest)?;

            FileMetadatas::from(files_extracted)
        } else {
            let tar_path = tar_path.to_path_buf();
            return Err(TarXError::TarFileNotExists { tar_path });
        };
        let state = State::new(tar_x_state, Nothing);

        Ok(state)
    }
}
