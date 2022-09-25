use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;

use crate::{Error, WorkspaceSpec};

/// Wrapper to retrieve `web_sys::Storage` on demand.
#[derive(Clone, Debug)]
pub struct WebStorage {
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

impl WebStorage {
    /// Returns a new `WebStorage`.
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
        let window = web_sys::window().ok_or(Error::WindowNone)?;
        let storage = match self.workspace_spec {
            WorkspaceSpec::LocalStorage => {
                if !localStorageAvailable() {
                    return Err(Error::LocalStorageUnavailable);
                }
                window
                    .local_storage()
                    .map_err(crate::stringify_js_value)
                    .map_err(Error::LocalStorageGet)?
                    .ok_or(Error::LocalStorageNone)?
            }
            WorkspaceSpec::SessionStorage => {
                if !sessionStorageAvailable() {
                    return Err(Error::SessionStorageUnavailable);
                }
                window
                    .session_storage()
                    .map_err(crate::stringify_js_value)
                    .map_err(Error::SessionStorageGet)?
                    .ok_or(Error::SessionStorageNone)?
            }
        };

        Ok(storage)
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
    pub fn get_item_opt(&self, key: &str) -> Result<Option<String>, Error> {
        let storage = self.get()?;
        storage
            .get_item(key)
            .map_err(|js_value| Error::StorageGetItem {
                key: key.to_string(),
                error: crate::stringify_js_value(js_value),
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
    ) -> Result<impl Iterator<Item = Result<(&'f str, Option<String>), Error>>, Error>
    where
        I: Iterator<Item = &'f str>,
    {
        let storage = self.get()?;

        let iter = iter.map(move |key| {
            storage
                .get_item(key)
                .map(|value| (key, value))
                .map_err(|js_value| Error::StorageGetItem {
                    key: key.to_string(),
                    error: crate::stringify_js_value(js_value),
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
    pub fn get_item(&self, key: &str) -> Result<String, Error> {
        let storage = self.get()?;
        storage
            .get_item(key)
            .map_err(|js_value| Error::StorageGetItem {
                key: key.to_string(),
                error: crate::stringify_js_value(js_value),
            })
            .and_then(|value| {
                value.ok_or_else(|| Error::ItemNotExistent {
                    key: key.to_string(),
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
    ) -> Result<impl Iterator<Item = Result<(&'f str, String), Error>>, Error>
    where
        I: Iterator<Item = &'f str>,
    {
        let storage = self.get()?;

        let iter = iter.map(move |key| {
            storage
                .get_item(key)
                .map_err(|js_value| Error::StorageGetItem {
                    key: key.to_string(),
                    error: crate::stringify_js_value(js_value),
                })
                .and_then(|value| {
                    value.ok_or_else(|| Error::ItemNotExistent {
                        key: key.to_string(),
                    })
                })
                .map(|value| (key, value))
        });

        Ok(iter)
    }

    /// Sets an item in the web storage.
    ///
    /// See [`set_items`] if you would like to set multiple items.
    ///
    /// [`set_items`]: Self::set_items
    pub fn set_item(&self, key: &str, value: &str) -> Result<(), Error> {
        let storage = self.get()?;
        storage
            .set_item(key, value)
            .map_err(|js_value| Error::StorageSetItem {
                key: key.to_string(),
                value: value.to_string(),
                error: crate::stringify_js_value(js_value),
            })
    }

    /// Sets multiple items in the web storage.
    ///
    /// See [`set_item`] if you would like to set a single item.
    ///
    /// [`set_item`]: Self::set_item
    pub fn set_items<'f, I>(&self, mut iter: I) -> Result<(), Error>
    where
        I: Iterator<Item = (&'f str, &'f str)>,
    {
        let storage = self.get()?;

        iter.try_for_each(|(key, value)| {
            storage
                .set_item(key, value)
                .map_err(|js_value| Error::StorageSetItem {
                    key: key.to_string(),
                    value: value.to_string(),
                    error: crate::stringify_js_value(js_value),
                })
        })
    }

    /// Runs the provided closure for each item produced by the iterator,
    /// augmented with the web storage.
    pub fn iter_with_storage<F, I, T>(&self, mut iter: I, mut f: F) -> Result<(), Error>
    where
        F: for<'f> FnMut(&'f web_sys::Storage, T) -> Result<(), (String, String, JsValue)>,
        I: Iterator<Item = T>,
    {
        let storage = self.get()?;

        iter.try_for_each(|t| {
            f(&storage, t).map_err(|(key, value, js_value)| Error::StorageSetItem {
                key,
                value,
                error: crate::stringify_js_value(js_value),
            })
        })
    }

    /// Reads a serializable item from the given key.
    ///
    /// # Parameters
    ///
    /// * `key`: Path to read the serialized item.
    /// * `f_map_err`: Maps the deserialization error (if any) to an [`Error`].
    pub async fn serialized_read_opt<T, F>(
        &self,
        key: &str,
        f_map_err: F,
    ) -> Result<Option<T>, Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        self.get_item_opt(key)?
            .map(|s| serde_yaml::from_str::<T>(&s).map_err(f_map_err))
            .transpose()
    }

    /// Writes a serializable item to the given key.
    ///
    /// # Parameters
    ///
    /// * `key`: Path to store the serialized item.
    /// * `t`: Item to serialize.
    /// * `f_map_err`: Maps the serialization error (if any) to an [`Error`].
    pub async fn serialized_write<T, F>(&self, key: &str, t: &T, f_map_err: F) -> Result<(), Error>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
        F: FnOnce(serde_yaml::Error) -> Error + Send,
    {
        self.set_item(key, &serde_yaml::to_string(t).map_err(f_map_err)?)?;

        Ok(())
    }
}
