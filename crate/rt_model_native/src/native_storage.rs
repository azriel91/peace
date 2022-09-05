use std::{io::Write, path::Path, sync::Mutex};

use tokio::{
    fs::File,
    io::{BufReader, BufWriter},
};
use tokio_util::io::SyncIoBridge;

use crate::Error;

/// Wrapper to retrieve `web_sys::Storage` on demand.
#[derive(Clone, Debug)]
pub struct NativeStorage;

impl NativeStorage {
    /// Reads from a file, bridging to libraries that take a synchronous `Write`
    /// type.
    ///
    /// This method buffers the write, and calls flush on the buffer when the
    /// passed in closure returns.
    pub async fn read_with_sync_api<'f, F, T>(
        &self,
        thread_name: String,
        file_path: &Path,
        f: F,
    ) -> Result<T, Error>
    where
        F: FnOnce(&mut SyncIoBridge<BufReader<File>>) -> Result<T, Error> + Send + 'f,
        T: Send,
    {
        let file = File::open(file_path).await.map_err(|error| {
            let path = file_path.to_path_buf();
            Error::FileOpen { path, error }
        })?;
        let mut sync_io_bridge = SyncIoBridge::new(BufReader::new(file));

        // `tokio::task::spawn_blocking` doesn't work because it needs the closure's
        // environment to be `'static`
        let t = std::thread::scope(move |s| {
            std::thread::Builder::new()
                .name(thread_name)
                .spawn_scoped(s, move || {
                    let t = f(&mut sync_io_bridge)?;

                    Ok(t)
                })
                .map_err(Error::StorageSyncThreadSpawn)?
                .join()
                .map_err(Mutex::new)
                .map_err(Error::StorageSyncThreadJoin)?
        })?;

        Ok(t)
    }

    /// Writes to a file, bridging to libraries that take a synchronous `Write`
    /// type.
    ///
    /// This method buffers the write, and calls flush on the buffer when the
    /// passed in closure returns.
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
        let file = File::create(file_path).await.map_err(|error| {
            let path = file_path.to_path_buf();
            Error::FileCreate { path, error }
        })?;
        let mut sync_io_bridge = SyncIoBridge::new(BufWriter::new(file));

        // `tokio::task::spawn_blocking` doesn't work because it needs the closure's
        // environment to be `'static`
        let t = std::thread::scope(move |s| {
            std::thread::Builder::new()
                .name(thread_name)
                .spawn_scoped(s, move || {
                    let t = f(&mut sync_io_bridge)?;

                    sync_io_bridge.flush().map_err(|error| {
                        let path = file_path.to_path_buf();
                        Error::FileWrite { path, error }
                    })?;

                    Ok(t)
                })
                .map_err(Error::StorageSyncThreadSpawn)?
                .join()
                .map_err(Mutex::new)
                .map_err(Error::StorageSyncThreadJoin)?
        })?;

        Ok(t)
    }
}
