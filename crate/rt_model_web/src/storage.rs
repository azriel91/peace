use std::{
    fmt::Debug,
    hash::Hash,
    path::{Path, PathBuf},
};

use base64::Engine;
use peace_resource_rt::type_reg::{
    common::UnknownEntriesSome,
    untagged::{DataTypeWrapper, TypeMapOpt, TypeReg},
};
use peace_rt_model_core::{Error, WebError};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;

use crate::WorkspaceSpec;

/// Wrapper to retrieve `web_sys::Storage` on demand.
#[derive(Clone, Debug)]
pub struct Storage {
    /// Describes how to store peace automation data.
    workspace_spec: WorkspaceSpec,
}

#[wasm_bindgen(module = "/js/workspace.js")]
extern "C" {
    /// Returns whether local storage is available.
    fn localStorageAvailable() -> bool;
    /// Returns whether session storage is available.
    fn sessionStorageAvailable() -> bool;
}

impl Storage {
    /// Returns a new `Storage`.
    pub fn new(workspace_spec: WorkspaceSpec) -> Self {
        Self { workspace_spec }
    }

    /// Returns the browser storage used for the workspace.
    ///
    /// This is the local or session storage depending on the `WorkspaceSpec`
    /// passed into `Workspace::new`.
    ///
    /// `web_sys::Storage` is `!Send`, so cannot be inserted into `Resources`.
    /// As a compromise, we provide this function to fetch the storage when it
    /// needs to be accessed.
    pub fn get(&self) -> Result<web_sys::Storage, Error> {
        let window = web_sys::window().ok_or(WebError::WindowNone)?;
        let storage = match self.workspace_spec {
            WorkspaceSpec::LocalStorage => {
                if !localStorageAvailable() {
                    return Err(Error::Web(WebError::LocalStorageUnavailable));
                }
                window
                    .local_storage()
                    .map_err(crate::stringify_js_value)
                    .map_err(WebError::LocalStorageGet)
                    .map_err(Error::Web)?
                    .ok_or(Error::Web(WebError::LocalStorageNone))?
            }
            WorkspaceSpec::SessionStorage => {
                if !sessionStorageAvailable() {
                    return Err(Error::Web(WebError::SessionStorageUnavailable));
                }
                window
                    .session_storage()
                    .map_err(crate::stringify_js_value)
                    .map_err(WebError::SessionStorageGet)
                    .map_err(Error::Web)?
                    .ok_or(Error::Web(WebError::SessionStorageNone))?
            }
        };

        Ok(storage)
    }

    /// Returns whether an item exists in the web storage.
    pub fn contains_item(&self, path: &Path) -> Result<bool, Error> {
        self.get_item_opt(path).map(|item| item.is_some())
    }

    /// Gets an optional item in the web storage.
    ///
    /// * Use [`get_item_opt`] if you would like to fetch an item that may not
    ///   exist.
    /// * Use [`get_items_opt`] if you would like to fetch multiple optional
    ///   items.
    /// * Use [`get_item`] if you would like to fetch an item that must exist.
    /// * Use [`get_items`] if you would like to fetch multiple items that must
    ///   exist.
    ///
    /// [`get_items_opt`]: Self::get_items
    pub fn get_item_opt(&self, path: &Path) -> Result<Option<String>, Error> {
        let storage = self.get()?;
        let key = path.to_string_lossy();
        storage.get_item(key.as_ref()).map_err(|js_value| {
            Error::Web(WebError::StorageGetItem {
                path: path.to_path_buf(),
                error: crate::stringify_js_value(js_value),
            })
        })
    }

    /// Gets multiple items in the web storage.
    ///
    /// * Use [`get_item_opt`] if you would like to fetch an item that may not
    ///   exist.
    /// * Use [`get_items_opt`] if you would like to fetch multiple optional
    ///   items.
    /// * Use [`get_item`] if you would like to fetch an item that must exist.
    /// * Use [`get_items`] if you would like to fetch multiple items that must
    ///   exist.
    ///
    /// [`get_item`]: Self::get_item
    pub fn get_items_opt<'f, I>(
        &self,
        iter: I,
    ) -> Result<impl Iterator<Item = Result<(&'f Path, Option<String>), Error>>, Error>
    where
        I: Iterator<Item = &'f Path>,
    {
        let storage = self.get()?;

        let iter = iter.map(move |path| {
            let key = path.to_string_lossy();
            storage
                .get_item(key.as_ref())
                .map(|value| (path, value))
                .map_err(|js_value| {
                    Error::Web(WebError::StorageGetItem {
                        path: path.to_path_buf(),
                        error: crate::stringify_js_value(js_value),
                    })
                })
        });

        Ok(iter)
    }

    /// Gets an item in the web storage.
    ///
    /// * Use [`get_item_opt`] if you would like to fetch an item that may not
    ///   exist.
    /// * Use [`get_items_opt`] if you would like to fetch multiple optional
    ///   items.
    /// * Use [`get_item`] if you would like to fetch an item that must exist.
    /// * Use [`get_items`] if you would like to fetch multiple items that must
    ///   exist.
    ///
    /// [`get_items`]: Self::get_items
    pub fn get_item(&self, path: &Path) -> Result<String, Error> {
        let storage = self.get()?;
        let key = path.to_string_lossy();
        storage
            .get_item(key.as_ref())
            .map_err(|js_value| {
                Error::Web(WebError::StorageGetItem {
                    path: path.to_path_buf(),
                    error: crate::stringify_js_value(js_value),
                })
            })
            .and_then(|value| {
                value.ok_or_else(|| Error::ItemNotExists {
                    path: path.to_path_buf(),
                })
            })
    }

    /// Gets a base64 encoded item in the web storage.
    ///
    /// * Use [`get_item_b64_opt`] if you would like to fetch an item that may
    ///   not exist.
    ///
    /// [`get_items`]: Self::get_items
    pub fn get_item_b64_opt(&self, path: &Path) -> Result<Option<Vec<u8>>, Error> {
        self.get_item_opt(path).and_then(|value| {
            value
                .map(|value| {
                    base64::engine::general_purpose::STANDARD
                        .decode(&value)
                        .map_err(|error| {
                            Error::Web(WebError::StorageB64Decode {
                                path: path.to_path_buf(),
                                value,
                                error,
                            })
                        })
                })
                .transpose()
        })
    }

    /// Gets a base64 encoded item in the web storage.
    ///
    /// * Use [`get_item_b64_opt`] if you would like to fetch an item that may
    ///   not exist.
    ///
    /// [`get_items`]: Self::get_items
    pub fn get_item_b64(&self, path: &Path) -> Result<Vec<u8>, Error> {
        self.get_item(path).and_then(|value| {
            base64::engine::general_purpose::STANDARD
                .decode(&value)
                .map_err(|error| {
                    Error::Web(WebError::StorageB64Decode {
                        path: path.to_path_buf(),
                        value,
                        error,
                    })
                })
        })
    }

    /// Gets multiple items in the web storage.
    ///
    /// * Use [`get_item_opt`] if you would like to fetch an item that may not
    ///   exist.
    /// * Use [`get_items_opt`] if you would like to fetch multiple optional
    ///   items.
    /// * Use [`get_item`] if you would like to fetch an item that must exist.
    /// * Use [`get_items`] if you would like to fetch multiple items that must
    ///   exist.
    ///
    /// [`get_item`]: Self::get_item
    pub fn get_items<'f, I>(
        &self,
        iter: I,
    ) -> Result<impl Iterator<Item = Result<(&'f Path, String), Error>>, Error>
    where
        I: Iterator<Item = &'f Path>,
    {
        let storage = self.get()?;

        let iter = iter.map(move |path| {
            let key = path.to_string_lossy();
            storage
                .get_item(key.as_ref())
                .map_err(|js_value| {
                    Error::Web(WebError::StorageGetItem {
                        path: path.to_path_buf(),
                        error: crate::stringify_js_value(js_value),
                    })
                })
                .and_then(|value| {
                    value.ok_or_else(|| Error::ItemNotExists {
                        path: path.to_path_buf(),
                    })
                })
                .map(|value| (path, value))
        });

        Ok(iter)
    }

    /// Sets an item in the web storage.
    ///
    /// See [`set_items`] if you would like to set multiple items.
    ///
    /// [`set_items`]: Self::set_items
    pub fn set_item(&self, path: &Path, value: &str) -> Result<(), Error> {
        let storage = self.get()?;
        let key = path.to_string_lossy();
        storage.set_item(key.as_ref(), value).map_err(|js_value| {
            Error::Web(WebError::StorageSetItem {
                path: path.to_path_buf(),
                value: value.to_string(),
                error: crate::stringify_js_value(js_value),
            })
        })
    }

    /// Base64 encodes and sets a value in the web storage.
    pub fn set_item_b64<B>(&self, path: &Path, bytes: &B) -> Result<(), Error>
    where
        B: AsRef<[u8]>,
    {
        let value = base64::engine::general_purpose::STANDARD.encode(bytes);
        self.set_item(path, &value)
    }

    /// Sets multiple items in the web storage.
    ///
    /// See [`set_item`] if you would like to set a single item.
    ///
    /// [`set_item`]: Self::set_item
    pub fn set_items<'f, I>(&self, mut iter: I) -> Result<(), Error>
    where
        I: Iterator<Item = (&'f Path, &'f str)>,
    {
        let storage = self.get()?;

        iter.try_for_each(|(path, value)| {
            let key = path.to_string_lossy();
            storage.set_item(key.as_ref(), value).map_err(|js_value| {
                Error::Web(WebError::StorageSetItem {
                    path: path.to_path_buf(),
                    value: value.to_string(),
                    error: crate::stringify_js_value(js_value),
                })
            })
        })
    }

    /// Runs the provided closure for each item produced by the iterator,
    /// augmented with the web storage.
    ///
    /// Note that the `storage` passed to the closure is the browser storage, so
    /// `set_item` takes a `&str` for the key.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use std::path::PathBuf;
    /// # use peace_rt_model_web::{Storage, WorkspaceSpec, Error};
    /// #
    /// # fn main() -> Result<(), Error> {
    /// let storage = Storage::new(WorkspaceSpec::SessionStorage);
    /// let keys = ["abc", "def"];
    ///
    /// storage.iter_with_storage(keys.into_iter(), |storage, key| {
    ///     let value = "something";
    ///     storage
    ///         .set_item(key, value)
    ///         .map_err(|js_value| (PathBuf::from(key), value.to_string(), js_value))
    /// })?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn iter_with_storage<F, I, T>(&self, mut iter: I, mut f: F) -> Result<(), Error>
    where
        F: for<'f> FnMut(&'f web_sys::Storage, T) -> Result<(), (PathBuf, String, JsValue)>,
        I: Iterator<Item = T>,
    {
        let storage = self.get()?;

        iter.try_for_each(|t| {
            f(&storage, t).map_err(|(path, value, js_value)| {
                Error::Web(WebError::StorageSetItem {
                    path,
                    value,
                    error: crate::stringify_js_value(js_value),
                })
            })
        })
    }

    /// Reads a serializable item from the given key.
    ///
    /// # Parameters
    ///
    /// * `path`: Path to read the serialized item.
    /// * `f_map_err`: Maps the deserialization error (if any) to an [`Error`].
    pub async fn serialized_read_opt<T, F>(
        &self,
        path: &Path,
        f_map_err: F,
    ) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned + Send + Sync,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        self.get_item_opt(path)?
            .map(|s| serde_yaml::from_str::<T>(&s).map_err(f_map_err))
            .transpose()
    }

    /// Deserializes a typemap from the given path if the file exists.
    ///
    /// # Parameters
    ///
    /// * `thread_name`: Name of the thread to use to do the read operation.
    /// * `type_reg`: Type registry with the stateful deserialization mappings.
    /// * `file_path`: Path to the file to read the serialized item.
    /// * `f_map_err`: Maps the deserialization error (if any) to an [`Error`].
    pub async fn serialized_typemap_read_opt<K, BoxDT, F>(
        &self,
        type_reg: &TypeReg<K, BoxDT>,
        path: &Path,
        f_map_err: F,
    ) -> Result<Option<TypeMapOpt<K, BoxDT, UnknownEntriesSome<serde_yaml::Value>>>, Error>
    where
        K: Clone + Debug + DeserializeOwned + Eq + Hash + Sync + 'static,
        BoxDT: DataTypeWrapper + 'static,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        self.get_item_opt(path)?
            .map(|s| {
                let deserializer = serde_yaml::Deserializer::from_str(&s);
                let type_map_opt = type_reg
                    .deserialize_map_opt_with_unknowns::<'_, serde_yaml::Value, _, _>(deserializer)
                    .map_err(f_map_err)?;

                Ok(type_map_opt)
            })
            .transpose()
    }

    /// Writes a serializable item to the given path.
    ///
    /// # Parameters
    ///
    /// * `path`: Path to store the serialized item.
    /// * `t`: Item to serialize.
    /// * `f_map_err`: Maps the serialization error (if any) to an [`Error`].
    pub async fn serialized_write<T, F>(
        &self,
        path: &Path,
        t: &T,
        f_map_err: F,
    ) -> Result<(), Error>
    where
        T: Serialize + Send + Sync,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        self.set_item(path, &serde_yaml::to_string(t).map_err(f_map_err)?)?;

        Ok(())
    }

    /// Serializes an item to a string.
    ///
    /// # Parameters
    ///
    /// * `t`: Item to serialize.
    /// * `f_map_err`: Maps the serialization error (if any) to an [`Error`].
    pub fn serialized_write_string<T, F>(&self, t: &T, f_map_err: F) -> Result<String, Error>
    where
        T: Serialize + Send + Sync,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        serde_yaml::to_string(t).map_err(f_map_err)
    }

    /// Deletes an item from the web storage.
    pub fn remove_item(&self, path: &Path) -> Result<(), Error> {
        let storage = self.get()?;
        let key = path.to_string_lossy();
        storage.remove_item(key.as_ref()).map_err(|js_value| {
            Error::Web(WebError::StorageRemoveItem {
                path: path.to_path_buf(),
                error: crate::stringify_js_value(js_value),
            })
        })
    }
}
