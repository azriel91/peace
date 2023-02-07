use std::path::PathBuf;

use peace_core::FlowId;

// Remember to add common variants to `rt_model_native/src/error.rs`.

/// Peace web support errors.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to serialize error.
    #[error("Failed to serialize error.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::error_serialize))
    )]
    ErrorSerialize(#[source] serde_yaml::Error),

    /// Failed to serialize a presentable type.
    #[error("Failed to serialize a presentable type.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::presentable_serialize))
    )]
    PresentableSerialize(#[source] serde_yaml::Error),

    /// Failed to serialize progress update.
    #[error("Failed to serialize progress update.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::progress_update_serialize))
    )]
    ProgressUpdateSerialize(#[source] serde_yaml::Error),
    /// Failed to serialize progress update as JSON.
    #[cfg(feature = "output_json")]
    #[error("Failed to serialize progress update.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::progress_update_serialize_json))
    )]
    ProgressUpdateSerializeJson(#[source] serde_json::Error),

    /// Failed to deserialize states.
    #[error("Failed to deserialize states for flow: `{flow_id}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::states_deserialize),
            help(
                "Make sure that all commands using the `{flow_id}` flow, also use the same item spec graph.\n\
                This is because all ItemSpecs are used to deserialize state.\n\
                \n\
                If the item spec graph is different, it may make sense to use a different flow ID."
            )
        )
    )]
    StatesDeserialize {
        /// Flow ID whose states are being deserialized.
        flow_id: FlowId,
        /// Source text to be deserialized.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        states_file_source: miette::NamedSource,
        /// Offset within the source text that the error occurred.
        #[cfg(feature = "error_reporting")]
        #[label("{}", error_message)]
        error_span: Option<miette::SourceOffset>,
        /// Message explaining the error.
        #[cfg(feature = "error_reporting")]
        error_message: String,
        /// Offset within the source text surrounding the error.
        #[cfg(feature = "error_reporting")]
        #[label]
        context_span: Option<miette::SourceOffset>,
        /// Underlying error.
        #[source]
        error: serde_yaml::Error,
    },

    /// Failed to serialize states.
    #[error("Failed to serialize states.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::states_serialize))
    )]
    StatesSerialize(#[source] serde_yaml::Error),

    /// Current states have not been discovered.
    ///
    /// This is returned when `StatesSavedFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Current states have not been discovered.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::states_current_discover_required),
            help("Ensure that `StatesDiscoverCmd` or `StatesCurrentDiscoverCmd` has been called.")
        )
    )]
    StatesCurrentDiscoverRequired,

    /// Desired states have not been written to disk.
    ///
    /// This is returned when `StatesDesiredFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Desired states have not been written to disk.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::states_desired_discover_required),
            help("Ensure that `StatesDiscoverCmd` or `StatesDesiredDiscoverCmd` has been called.")
        )
    )]
    StatesDesiredDiscoverRequired,

    /// Failed to serialize state diffs.
    #[error("Failed to serialize state diffs.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::state_diffs_serialize))
    )]
    StateDiffsSerialize(#[source] serde_yaml::Error),

    /// Failed to serialize error as JSON.
    #[cfg(feature = "output_json")]
    #[error("Failed to serialize error as JSON.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::error_serialize_json))
    )]
    ErrorSerializeJson(#[source] serde_json::Error),

    /// Failed to serialize states as JSON.
    #[cfg(feature = "output_json")]
    #[error("Failed to serialize states as JSON.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::states_current_serialize_json))
    )]
    StatesSerializeJson(#[source] serde_json::Error),

    /// Failed to serialize state diffs as JSON.
    #[cfg(feature = "output_json")]
    #[error("Failed to serialize state diffs as JSON.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::state_diffs_serialize_json))
    )]
    StateDiffsSerializeJson(#[source] serde_json::Error),

    /// Failed to serialize workspace init params.
    #[error("Failed to serialize workspace init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::workspace_init_params_serialize))
    )]
    WorkspaceParamsSerialize(#[source] serde_yaml::Error),

    /// Failed to deserialize workspace init params.
    #[error("Failed to deserialize workspace init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::workspace_init_params_deserialize))
    )]
    WorkspaceParamsDeserialize(#[source] serde_yaml::Error),

    /// Workspace params does not exist, so cannot look up `Profile`.
    #[error("Workspace params does not exist, so cannot look up `Profile`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::workspace_params_none_for_profile))
    )]
    WorkspaceParamsNoneForProfile,

    /// Workspace param for `Profile` does not exist.
    #[error("Workspace param for `Profile` does not exist.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::workspace_params_profile_none))
    )]
    WorkspaceParamsProfileNone,

    /// Failed to serialize profile init params.
    #[error("Failed to serialize profile init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::profile_init_params_serialize))
    )]
    ProfileParamsSerialize(#[source] serde_yaml::Error),

    /// Failed to deserialize profile init params.
    #[error("Failed to deserialize profile init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::profile_init_params_deserialize))
    )]
    ProfileParamsDeserialize(#[source] serde_yaml::Error),

    /// Failed to serialize flow init params.
    #[error("Failed to serialize flow init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::flow_init_params_serialize))
    )]
    FlowParamsSerialize(#[source] serde_yaml::Error),

    /// Failed to deserialize flow init params.
    #[error("Failed to deserialize flow init params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::flow_init_params_deserialize))
    )]
    FlowParamsDeserialize(#[source] serde_yaml::Error),

    /// Item does not exist in storage.
    #[error("Item does not exist in storage: `{}`.", path.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::item_not_exists))
    )]
    ItemNotExists {
        /// Path to the file.
        path: PathBuf,
    },

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

    /// Failed to base64 decode an item from browser storage.
    #[error(
        "Failed to base64 decode an item in browser storage: `{path}`. Value: `{value}` Error: `{error}`"
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

    /// Failed to get an item from browser storage.
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
    #[error("Failed to get an item in browser storage: `{path}`. Error: `{error}`")]
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
    #[error("Failed to set an item in browser storage: `{path}`: `{value}`. Error: `{error}`")]
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
    /// Failed to remove an item from browser storage.
    ///
    /// This failure mode happens when the `get_item` call to the browser fails.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to remove an item from browser storage: `{path}`. Error: `{error}`")]
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
