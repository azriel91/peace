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
    /// passed into `Workspace::init`.
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

    /// Returns  web storage.
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
}
