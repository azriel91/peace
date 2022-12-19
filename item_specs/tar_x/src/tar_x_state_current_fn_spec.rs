use std::{io::Read, marker::PhantomData, path::Path};

use peace::cfg::{async_trait, state::Nothing, FnSpec, State};
use tar::Archive;

use crate::{FileMetadata, TarXData, TarXError, TarXState};

/// Status `FnSpec` for the tar to extract.
#[derive(Debug)]
pub struct TarXStateCurrentFnSpec<Id>(PhantomData<Id>);

impl<Id> TarXStateCurrentFnSpec<Id> {
    fn tar_file_metadata<R>(
        tar_path: &Path,
        mut archive: Archive<R>,
    ) -> Result<Vec<FileMetadata>, TarXError>
    where
        R: Read,
    {
        archive
            .entries()
            .map_err(|error| {
                let tar_path = tar_path.to_path_buf();
                TarXError::TarEntryRead { tar_path, error }
            })?
            .try_fold(Vec::new(), |mut files_in_tar, entry| {
                let entry = entry.map_err(|error| {
                    let tar_path = tar_path.to_path_buf();
                    TarXError::TarEntryRead { tar_path, error }
                })?;
                let entry_path = entry.path().map_err(|error| {
                    let tar_path = tar_path.to_path_buf();
                    TarXError::TarEntryPathRead { tar_path, error }
                })?;
                let modified_time = entry.header().mtime().map_err(|error| {
                    let tar_path = tar_path.to_path_buf();
                    let entry_path = entry_path.to_path_buf();
                    TarXError::TarEntryMTimeRead {
                        tar_path,
                        entry_path,
                        error,
                    }
                })?;

                let file_metadata = FileMetadata::new(entry_path.to_path_buf(), modified_time);
                files_in_tar.push(file_metadata);

                Ok(files_in_tar)
            })
    }
}

#[async_trait(?Send)]
impl<Id> FnSpec for TarXStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type Output = State<TarXState, Nothing>;

    async fn exec(tar_x_data: TarXData<'_, Id>) -> Result<Self::Output, TarXError> {
        let tar_x_params = tar_x_data.tar_x_params();
        let storage = tar_x_data.storage();
        let tar_path = tar_x_params.tar_path();
        let dest = tar_x_params.dest();

        let files_in_tar = if tar_path.exists() {
            #[cfg(not(target_arch = "wasm32"))]
            {
                storage
                    .read_with_sync_api(
                        "TarXStateCurrentFnSpec::exec".to_string(),
                        tar_path,
                        |sync_io_bridge| {
                            Self::tar_file_metadata(tar_path, Archive::new(sync_io_bridge))
                        },
                    )
                    .await?
            }

            #[cfg(target_arch = "wasm32")]
            {
                use std::io::Cursor;

                let bytes = storage.get_item_b64(tar_path)?;
                Self::tar_file_metadata(tar_path, Archive::new(Cursor::new(bytes)))?
            }
        } else {
            let tar_path = tar_path.to_path_buf();
            return Err(TarXError::TarFileNonExistent { tar_path });
        };

        let files_extracted = if dest.exists() {
            todo!();
        } else {
            Vec::new()
        };

        let tar_x_state = TarXState::ExtractionOutOfSync {
            files_in_tar,
            files_extracted,
        };
        let state = State::new(tar_x_state, Nothing);

        Ok(state)
    }
}
