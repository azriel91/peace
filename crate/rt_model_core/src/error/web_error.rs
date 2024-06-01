use std::path::PathBuf;

/// Peace web support errors.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum WebError {
    // web_sys related errors
    /// Browser local storage unavailable.
    #[error("Browser local storage unavailable.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::local_storage_unavailable))
    )]
    LocalStorageUnavailable,
    /// Failed to get browser local storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to get browser local storage: `{0}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::local_storage_get))
    )]
    LocalStorageGet(String),
    /// Browser local storage is `None`.
    #[error("Browser local storage is none.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::local_storage_none))
    )]
    LocalStorageNone,
    /// Browser session storage unavailable.
    #[error("Browser session storage unavailable.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::session_storage_unavailable))
    )]
    SessionStorageUnavailable,
    /// Failed to get browser session storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to get browser session storage: `{0}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::session_storage_get))
    )]
    SessionStorageGet(String),
    /// Browser session storage is `None`.
    #[error("Browser session storage is none.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::session_storage_none))
    )]
    SessionStorageNone,

    /// Failed to base64 decode a step from browser storage.
    #[error(
        "Failed to base64 decode a step in browser storage: `{path}`. Value: `{value}` Error: `{error}`"
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::storage_b64_decode))
    )]
    StorageB64Decode {
        /// Key to get.
        path: PathBuf,
        /// The base64 encoded value.
        value: String,
        /// Base64 decode error.
        error: base64::DecodeError,
    },

    /// Failed to get a step from browser storage.
    ///
    /// This failure mode happens when the `get_item` call to the browser fails.
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
    #[error("Failed to get a step in browser storage: `{path}`. Error: `{error}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::storage_get_item))
    )]
    StorageGetItem {
        /// Key to get.
        path: PathBuf,
        /// Stringified JS error.
        error: String,
    },
    /// Failed to set a step in browser storage.
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
    #[error("Failed to set a step in browser storage: `{path}`: `{value}`. Error: `{error}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::storage_set_item))
    )]
    StorageSetItem {
        /// Key to set.
        path: PathBuf,
        /// Value which failed to be set.
        value: String,
        /// Stringified JS error.
        error: String,
    },
    /// Failed to remove a step from browser storage.
    ///
    /// This failure mode happens when the `get_item` call to the browser fails.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to remove a step from browser storage: `{path}`. Error: `{error}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::storage_remove_item))
    )]
    StorageRemoveItem {
        /// Key to remove.
        path: PathBuf,
        /// Stringified JS error.
        error: String,
    },
    /// Failed to fetch browser Window object.
    #[error("Failed to fetch browser Window object.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model_web::window_none))
    )]
    WindowNone,
}
