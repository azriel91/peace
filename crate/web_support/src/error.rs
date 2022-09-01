/// Peace web support errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to serialize states.
    #[error("Failed to serialize states.")]
    StatesSerialize(#[source] serde_yaml::Error),

    // web_sys related errors
    /// Browser local storage unavailable.
    #[error("Browser local storage unavailable.")]
    LocalStorageUnavailable,
    /// Failed to get browser local storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to get browser local storage: `{0}`")]
    LocalStorageGet(String),
    /// Browser local storage is `None`.
    #[error("Browser local storage is none.")]
    LocalStorageNone,
    /// Browser session storage unavailable.
    #[error("Browser session storage unavailable.")]
    SessionStorageUnavailable,
    /// Failed to get browser session storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to get browser session storage: `{0}`")]
    SessionStorageGet(String),
    /// Browser session storage is `None`.
    #[error("Browser session storage is none.")]
    SessionStorageNone,
    /// Failed to set an item in browser storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to set an item in browser storage: `{key}`: `{value}`. Error: `{error}`")]
    StorageSetItem {
        /// Key to set.
        key: String,
        /// Value which failed to be set.
        value: String,
        /// Value which failed to be set.
        error: String,
    },
    /// Failed to fetch browser Window object.
    #[error("Failed to fetch browser Window object.")]
    WindowNone,
}
