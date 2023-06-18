use std::{fmt::Debug, hash::Hash, io::Write, path::Path, sync::Mutex};

use peace_resources::type_reg::untagged::{DataTypeWrapper, TypeMap, TypeReg};
use peace_rt_model_core::{Error, NativeError};
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    fs::File,
    io::{BufReader, BufWriter},
};
use tokio_util::io::SyncIoBridge;

/// Wrapper around file system operations.
#[derive(Clone, Debug)]
pub struct Storage;

impl Storage {
    /// Reads a serializable item from the given path.
    ///
    /// # Parameters
    ///
    /// * `thread_name`: Name of the thread to use to do the read operation.
    /// * `file_path`: Path to the file to read the serialized item.
    /// * `f_map_err`: Maps the deserialization error (if any) to an [`Error`].
    pub async fn serialized_read<T, F>(
        &self,
        thread_name: String,
        file_path: &Path,
        f_map_err: F,
    ) -> Result<T, Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        if file_path.exists() {
            let t = self
                .read_with_sync_api(thread_name, file_path, |file| {
                    serde_yaml::from_reader::<_, T>(file).map_err(f_map_err)
                })
                .await?;

            Ok(t)
        } else {
            Err(Error::ItemNotExists {
                path: file_path.to_path_buf(),
            })
        }
    }

    /// Reads a serializable item from the given path if the file exists.
    ///
    /// # Parameters
    ///
    /// * `thread_name`: Name of the thread to use to do the read operation.
    /// * `file_path`: Path to the file to read the serialized item.
    /// * `f_map_err`: Maps the deserialization error (if any) to an [`Error`].
    pub async fn serialized_read_opt<T, F>(
        &self,
        thread_name: String,
        file_path: &Path,
        f_map_err: F,
    ) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned + Send + Sync,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        if file_path.exists() {
            let t = self
                .read_with_sync_api(thread_name, file_path, |file| {
                    serde_yaml::from_reader::<_, T>(file).map_err(f_map_err)
                })
                .await?;

            Ok(Some(t))
        } else {
            Ok(None)
        }
    }

    /// Deserializes a typemap from the given path if the file exists.
    ///
    /// # Parameters
    ///
    /// * `thread_name`: Name of the thread to use to do the read operation.
    /// * `type_reg`: Type registry with the stateful deserialization mappings.
    /// * `file_path`: Path to the file to read the serialized item.
    /// * `f_map_err`: Maps the deserialization error (if any) to an [`Error`].
    pub async fn serialized_typemap_read_opt<T, K, BoxDT, F>(
        &self,
        thread_name: String,
        type_reg: &TypeReg<K, BoxDT>,
        file_path: &Path,
        f_map_err: F,
    ) -> Result<Option<T>, Error>
    where
        T: From<TypeMap<K, BoxDT>> + Send + Sync,
        K: Debug + DeserializeOwned + Eq + Hash + Sync,
        BoxDT: DataTypeWrapper + 'static,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        if file_path.exists() {
            let t = self
                .read_with_sync_api(thread_name, file_path, |file| {
                    let deserializer = serde_yaml::Deserializer::from_reader(file);
                    let type_map = type_reg.deserialize_map(deserializer).map_err(f_map_err)?;

                    Result::<_, Error>::Ok(T::from(type_map))
                })
                .await?;

            Ok(Some(t))
        } else {
            Ok(None)
        }
    }

    /// Writes a serializable item to the given path.
    ///
    /// # Parameters
    ///
    /// * `thread_name`: Name of the thread to use to do the write operation.
    /// * `file_path`: Path to the file to store the serialized item.
    /// * `t`: Item to serialize.
    /// * `f_map_err`: Maps the serialization error (if any) to an [`Error`].
    pub async fn serialized_write<T, F>(
        &self,
        thread_name: String,
        file_path: &Path,
        t: &T,
        f_map_err: F,
    ) -> Result<(), Error>
    where
        T: Serialize + Send + Sync,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        self.write_with_sync_api(thread_name, file_path, |file| {
            serde_yaml::to_writer(file, t).map_err(f_map_err)
        })
        .await?;

        Ok(())
    }

    /// Reads from a file, bridging to libraries that take a synchronous `Write`
    /// type.
    ///
    /// This method buffers the write, and calls flush on the buffer when the
    /// passed in closure returns.
    pub async fn read_with_sync_api<'f, F, T, E>(
        &self,
        thread_name: String,
        file_path: &Path,
        f: F,
    ) -> Result<T, E>
    where
        F: FnOnce(&mut SyncIoBridge<BufReader<File>>) -> Result<T, E> + Send + 'f,
        T: Send,
        E: From<Error> + Send,
    {
        let file = File::open(file_path).await.map_err(
            // Tests currently don't cover file system failure cases,
            // e.g. disk space limits.
            #[cfg_attr(coverage_nightly, no_coverage)]
            |error| {
                let path = file_path.to_path_buf();
                Error::Native(NativeError::FileOpen { path, error })
            },
        )?;
        let mut sync_io_bridge = SyncIoBridge::new(BufReader::new(file));

        // `tokio::task::spawn_blocking` doesn't work because it needs the closure's
        // environment to be `'static`
        let t = std::thread::scope(move |s| {
            std::thread::Builder::new()
                .name(thread_name)
                .spawn_scoped(s, move || f(&mut sync_io_bridge))
                .map_err(NativeError::StorageSyncThreadSpawn)
                .map_err(Error::Native)?
                .join()
                .map_err(Mutex::new)
                .map_err(NativeError::StorageSyncThreadJoin)
                .map_err(Error::Native)?
        })?;

        Ok(t)
    }

    /// Writes to a file, bridging to libraries that take a synchronous `Write`
    /// type.
    ///
    /// This method buffers the write, and calls flush on the buffer when the
    /// passed in closure returns.
    ///
    /// # Parameters
    ///
    /// * `thread_name`: Name of the thread to use to do the write operation.
    /// * `file_path`: Path to the file to store the serialized item.
    /// * `f`: Function that is given the `Write` implementation to call the
    ///   sync API with.
    pub async fn write_with_sync_api<'f, F, T>(
        &self,
        thread_name: String,
        file_path: &Path,
        f: F,
    ) -> Result<T, Error>
    where
        F: FnOnce(&mut SyncIoBridge<BufWriter<File>>) -> Result<T, Error> + Send + 'f,
        T: Send,
    {
        let file = File::create(file_path).await.map_err(
            // Tests currently don't cover file system failure cases,
            // e.g. disk space limits.
            #[cfg_attr(coverage_nightly, no_coverage)]
            |error| {
                let path = file_path.to_path_buf();
                NativeError::FileCreate { path, error }
            },
        )?;
        let mut sync_io_bridge = SyncIoBridge::new(BufWriter::new(file));

        // `tokio::task::spawn_blocking` doesn't work because it needs the closure's
        // environment to be `'static`
        let t = std::thread::scope(move |s| {
            std::thread::Builder::new()
                .name(thread_name)
                .spawn_scoped(s, move || {
                    let t = f(&mut sync_io_bridge)?;

                    sync_io_bridge.flush().map_err(
                        // Tests currently don't cover file system failure cases,
                        // e.g. disk space limits.
                        #[cfg_attr(coverage_nightly, no_coverage)]
                        |error| {
                            let path = file_path.to_path_buf();
                            NativeError::FileWrite { path, error }
                        },
                    )?;

                    Result::<_, Error>::Ok(t)
                })
                .map_err(NativeError::StorageSyncThreadSpawn)
                .map_err(Error::Native)?
                .join()
                .map_err(Mutex::new)
                .map_err(NativeError::StorageSyncThreadJoin)
                .map_err(Error::Native)?
        })?;

        Ok(t)
    }
}
