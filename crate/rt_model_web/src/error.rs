// Remember to add common variants to `rt_model_native/src/error.rs`.

/// Peace web support errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to deserialize current states.
    #[error("Failed to deserialize current states.")]
    StatesCurrentDeserialize(#[source] serde_yaml::Error),
    /// Failed to serialize current states.
    #[error("Failed to serialize current states.")]
    StatesCurrentSerialize(#[source] serde_yaml::Error),
    /// Current states have not been written to disk.
    ///
    /// This is returned when `StatesCurrentFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Current states have not been written to disk.")]
    StatesCurrentDiscoverRequired,

    /// Failed to deserialize desired states.
    #[error("Failed to deserialize desired states.")]
    StatesDesiredDeserialize(#[source] serde_yaml::Error),
    /// Failed to serialize desired states.
    #[error("Failed to serialize desired states.")]
    StatesDesiredSerialize(#[source] serde_yaml::Error),
    /// Desired states have not been written to disk.
    ///
    /// This is returned when `StatesDesiredFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Desired states have not been written to disk.")]
    StatesDesiredDiscoverRequired,

    /// Failed to serialize state diffs.
    #[error("Failed to serialize state diffs.")]
    StateDiffsSerialize(#[source] serde_yaml::Error),

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
    /// Failed to get an item from browser storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    ///
    /// Instead of doing that, we could either:
    ///
    /// * Update `resman::Resource` to be `!Send` when compiling to WASM, or
    /// * Use <https://docs.rs/send_wrapper/> to wrap the `JsValue`.
    ///
    /// This is because browsers are generally single threaded. The assumption
    /// would no longer be true if multiple threads are used, e.g. web workers.
    #[error("Failed to get an item in browser storage: `{key}`. Error: `{error}`")]
    StorageGetItem {
        /// Key to get.
        key: String,
        /// Stringified JS error.
        error: String,
    },
    /// Failed to set an item in browser storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    ///
    /// Instead of doing that, we could either:
    ///
    /// * Update `resman::Resource` to be `!Send` when compiling to WASM, or
    /// * Use <https://docs.rs/send_wrapper/> to wrap the `JsValue`.
    ///
    /// This is because browsers are generally single threaded. The assumption
    /// would no longer be true if multiple threads are used, e.g. web workers.
    #[error("Failed to set an item in browser storage: `{key}`: `{value}`. Error: `{error}`")]
    StorageSetItem {
        /// Key to set.
        key: String,
        /// Value which failed to be set.
        value: String,
        /// Stringified JS error.
        error: String,
    },
    /// Failed to fetch browser Window object.
    #[error("Failed to fetch browser Window object.")]
    WindowNone,
}
