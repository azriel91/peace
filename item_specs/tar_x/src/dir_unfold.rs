use std::{collections::VecDeque, path::Path};

use tokio::fs::{DirEntry, ReadDir};

use crate::TarXError;

#[derive(Debug)]
pub struct DirUnfold;

impl DirUnfold {
    /// Provides a function that recursively produces file entries within the
    /// original directory.
    pub fn unfold(
        base_dir: &Path,
    ) -> impl futures::TryStream<Ok = DirEntry, Error = TarXError> + '_ {
        // `ReadDir` doesn't implement `Stream`, this does that mapping.
        futures::stream::try_unfold(
            (
                VecDeque::from([base_dir.to_path_buf()]),
                Option::<ReadDir>::None,
            ),
            move |(mut dirs, mut read_dir_opt)| async move {
                loop {
                    if let Some(mut read_dir) = read_dir_opt.take() {
                        let dir_entry = read_dir.next_entry().await.map_err(|error| {
                            let base_dir = base_dir.to_path_buf();
                            TarXError::TarDestEntryRead {
                                dest: base_dir,
                                error,
                            }
                        })?;

                        if let Some(dir_entry) = dir_entry {
                            let entry_path = dir_entry.path();
                            // Don't include directories as dir entries, but recursively descend
                            let file_type = dir_entry.file_type().await.map_err(|error| {
                                TarXError::TarDestEntryFileTypeRead {
                                    entry_path: entry_path.clone(),
                                    error,
                                }
                            })?;

                            if file_type.is_dir() {
                                dirs.push_back(entry_path);
                                continue;
                            } else {
                                break Result::<_, TarXError>::Ok(Some((
                                    dir_entry,
                                    (dirs, Some(read_dir)),
                                )));
                            }
                        } else {
                            read_dir_opt = None;
                            continue;
                        }
                    } else {
                        if let Some(dir) = dirs.pop_front() {
                            // Process next directory
                            read_dir_opt =
                                Some(tokio::fs::read_dir(&dir).await.map_err(|error| {
                                    let dir = dir.to_path_buf();
                                    TarXError::TarDestReadDir { dir, error }
                                })?);

                            continue;
                        } else {
                            // no more directories to process
                            break Ok(None);
                        }
                    }
                }
            },
        )
    }
}
