use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use tokio::fs::ReadDir;

use crate::{native::DestDirEntry, TarXError};

#[derive(Debug)]
pub(crate) struct DirUnfold;

impl DirUnfold {
    /// Provides a function that recursively produces file entries within the
    /// original directory.
    pub(crate) fn unfold(
        base_dir: &Path,
    ) -> impl futures::TryStream<Ok = DestDirEntry, Error = TarXError> + '_ {
        // `ReadDir` doesn't implement `Stream`, this does that mapping.

        let dir_context = DirContext {
            base_dir,
            dir_and_read_dir_opt: None,
            dir_to_reads: VecDeque::from([DirToRead {
                dir_path: base_dir.to_path_buf(),
                dir_path_base_rel: PathBuf::new(),
            }]),
        };
        futures::stream::try_unfold(dir_context, move |dir_context| async move {
            let DirContext {
                base_dir,
                mut dir_and_read_dir_opt,
                mut dir_to_reads,
            } = dir_context;
            loop {
                if let Some(dir_and_read_dir) = dir_and_read_dir_opt.take() {
                    let DirAndReadDir {
                        dir_path_base_rel,
                        mut read_dir,
                    } = dir_and_read_dir;
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

                        let dest_dir_relative_path = dir_path_base_rel.join(dir_entry.file_name());
                        if file_type.is_dir() {
                            dir_to_reads.push_back(DirToRead {
                                dir_path: entry_path,
                                dir_path_base_rel: dest_dir_relative_path,
                            });
                            dir_and_read_dir_opt = Some(DirAndReadDir {
                                dir_path_base_rel,
                                read_dir,
                            });
                            continue;
                        } else {
                            break Result::<_, TarXError>::Ok(Some((
                                DestDirEntry {
                                    dest_dir_relative_path,
                                    dir_entry,
                                },
                                DirContext {
                                    base_dir,
                                    dir_and_read_dir_opt: Some(DirAndReadDir {
                                        dir_path_base_rel,
                                        read_dir,
                                    }),
                                    dir_to_reads,
                                },
                            )));
                        }
                    } else {
                        dir_and_read_dir_opt = None;
                        continue;
                    }
                } else {
                    if let Some(dir_to_read) = dir_to_reads.pop_front() {
                        let DirToRead {
                            dir_path,
                            dir_path_base_rel,
                        } = dir_to_read;
                        // Process next directory
                        dir_and_read_dir_opt = Some(
                            tokio::fs::read_dir(&dir_path)
                                .await
                                .map_err(|error| TarXError::TarDestReadDir {
                                    dir: dir_path,
                                    error,
                                })
                                .map(|read_dir| DirAndReadDir {
                                    dir_path_base_rel,
                                    read_dir,
                                })?,
                        );

                        continue;
                    } else {
                        // no more directories to process
                        break Ok(None);
                    }
                }
            }
        })
    }
}

#[derive(Debug)]
struct DirContext<'base> {
    /// Base directory to recurse through.
    base_dir: &'base Path,
    /// Current `ReadDir` being iterated through.
    dir_and_read_dir_opt: Option<DirAndReadDir>,
    /// Remaining directories to process,
    dir_to_reads: VecDeque<DirToRead>,
}

/// Tracks a directory's path, and its relative path to the base directory.
///
/// Example values:
///
/// ```yaml
/// base_dir:          'extraction/dir'
/// dir_path:          'extraction/dir/sub/dir'
/// dir_path_base_rel: 'sub/dir'
/// ```
#[derive(Debug)]
struct DirToRead {
    /// Path to the directory to process,
    dir_path: PathBuf,
    /// Path to the directory to process, relative to the base directory.
    dir_path_base_rel: PathBuf,
}

#[derive(Debug)]
struct DirAndReadDir {
    /// Path to the directory to process, relative to the base directory.
    dir_path_base_rel: PathBuf,
    /// `ReadDir` for the directory's entries
    read_dir: ReadDir,
}
