use std::path::PathBuf;

use peace_core::{FlowId, ItemSpecId, Profile};
use peace_resources::paths::ItemSpecParamsFile;

use crate::ItemSpecParams;

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        pub use self::native_error::NativeError;

        mod native_error;
    } else {
        pub use self::web_error::WebError;

        mod web_error;
    }
}

/// Peace runtime errors.
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

    /// Item spec params do not match with the item specs in the flow.
    ///
    /// # Symptoms
    ///
    /// * Provided params for an item spec ID has no corresponding item spec ID
    ///   in the flow.
    /// * Stored params for an item spec ID has no corresponding item spec ID in
    ///   the flow.
    /// * ID of an item spec in the flow does not have a corresponding provided
    ///   param.
    /// * ID of an item spec in the flow does not have a corresponding stored
    ///   param.
    ///
    /// # Causes
    ///
    /// These can happen when:
    ///
    /// * An item spec is added.
    ///
    ///    - No corresponding provided param.
    ///    - No corresponding stored param.
    ///
    /// * An item spec ID is renamed.
    ///
    ///    - Provided param ID mismatch.
    ///    - Stored param ID mismatch.
    ///    - No corresponding provided param
    ///
    /// * An item spec is removed.
    ///
    ///    - Provided param ID mismatch.
    ///    - Stored param ID mismatch.
    #[error("Item spec params do not match with the item specs in the flow.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::item_spec_params_mismatch),
            help("{}", item_spec_params_mismatch_display(
                item_spec_ids_with_no_params,
                provided_item_spec_params_mismatches,
                stored_item_spec_params_mismatches.as_ref(),
            ))
        )
    )]
    ItemSpecParamsMismatch {
        /// Item spec IDs for which there are no provided or stored params.
        item_spec_ids_with_no_params: Vec<ItemSpecId>,
        /// Provided item spec params with no matching item spec ID in the flow.
        provided_item_spec_params_mismatches: ItemSpecParams,
        /// Stored item spec params with no matching item spec ID in the flow.
        stored_item_spec_params_mismatches: Option<ItemSpecParams>,
    },

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

    /// Failed to deserialize item spec params.
    #[error("Failed to deserialize item spec params for `{profile}/{flow_id}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::item_spec_params_deserialize),
            help(
                "Make sure that all commands using the `{flow_id}` flow, also use the same item spec graph.\n\
                This is because all ItemSpecs are used to deserialize state.\n\
                \n\
                If the item spec graph is different, it may make sense to use a different flow ID."
            )
        )
    )]
    ItemSpecParamsDeserialize {
        /// Profile of the flow.
        profile: Profile,
        /// Flow ID whose item spec params are being deserialized.
        flow_id: FlowId,
        /// Source text to be deserialized.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        item_spec_params_file_source: miette::NamedSource,
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

    /// Failed to serialize item spec params.
    #[error("Failed to serialize item spec params.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_rt_model::item_spec_params_serialize))
    )]
    ItemSpecParamsSerialize(#[source] serde_yaml::Error),

    /// Item spec params file does not exist.
    ///
    /// This is returned when `ItemSpecParams` is attempted to be
    /// deserialized but the file does not exist.
    ///
    /// The automation tool implementor needs to ensure the
    /// `SingleProfileSingleFlow` command context has been initialized for that
    /// flow previously.
    #[error("Item spec params file does not exist for `{profile}/{flow_id}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::item_spec_params_file_not_exists),
            help(
                "Ensure that a `SingleProfileSingleFlow` command context has previously been built."
            )
        )
    )]
    ItemSpecParamsFileNotExists {
        /// Profile of the flow.
        profile: Profile,
        /// Flow ID whose params are being deserialized.
        flow_id: FlowId,
        /// Path of the item spec params file.
        item_spec_params_file: ItemSpecParamsFile,
    },

    /// Current states have not been discovered.
    ///
    /// This is returned when `StatesSavedFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Current states have not been discovered.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::states_current_discover_required),
            help("Ensure that `StatesDiscoverCmd::current` has been called.")
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
            help("Ensure that `StatesDiscoverCmd::desired` has been called.")
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

    /// Profile to diff does not exist in `MultiProfileSingleFlow` scope.
    ///
    /// This could mean the caller provided a profile that does not exist, or
    /// the profile filter function filtered out the profile from the list of
    /// profiles.
    #[error("Profile `{profile}` not in scope, make sure it exists in `.peace/*/{profile}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::profile_not_in_scope),
            help(
                "Make sure the profile is spelt correctly.\n\
                Available profiles are: [{profiles_in_scope}]",
                profiles_in_scope = profiles_in_scope
                    .iter()
                    .map(|profile| format!("{profile}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        )
    )]
    ProfileNotInScope {
        /// The profile that was not in scope.
        profile: Profile,
        /// The profiles that are in scope.
        profiles_in_scope: Vec<Profile>,
    },

    /// Profile to diff has not had its states current discovered.
    #[error("Profile `{profile}`'s states have not been discovered.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::profile_states_current_not_discovered),
            help("Switch to the profile and run the states discover command.")
        )
    )]
    ProfileStatesCurrentNotDiscovered {
        /// The profile that was not in scope.
        profile: Profile,
    },

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

    /// Native application error occurred.
    #[error("Native application error occurred.")]
    #[cfg(not(target_arch = "wasm32"))]
    Native(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        NativeError,
    ),

    /// Web application error occurred.
    #[error("Web application error occurred.")]
    #[cfg(target_arch = "wasm32")]
    Web(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        WebError,
    ),
}

#[cfg(feature = "error_reporting")]
fn item_spec_params_mismatch_display(
    item_spec_ids_with_no_params: &[ItemSpecId],
    provided_item_spec_params_mismatches: &ItemSpecParams,
    stored_item_spec_params_mismatches: Option<&ItemSpecParams>,
) -> String {
    let mut items = Vec::<String>::new();

    if !item_spec_ids_with_no_params.is_empty() {
        items.push(format!(
            "The following item specs do not have parameters provided:\n\
            \n\
            {}\n",
            item_spec_ids_with_no_params
                .iter()
                .map(|item_spec_id| format!("* {item_spec_id}"))
                .collect::<Vec<String>>()
                .join("\n")
        ));
    }

    if !provided_item_spec_params_mismatches.is_empty() {
        items.push(format!(
            "The following provided params do not correspond to any item specs in the flow:\n\
                            \n\
                            {}\n",
            provided_item_spec_params_mismatches
                .keys()
                .map(|item_spec_id| format!("* {item_spec_id}"))
                .collect::<Vec<String>>()
                .join("\n")
        ))
    }

    if let Some(stored_item_spec_params_mismatches) = stored_item_spec_params_mismatches {
        if !stored_item_spec_params_mismatches.is_empty() {
            items.push(format!(
                "The following stored params do not correspond to any item specs in the flow:\n\
                        \n\
                        {}\n",
                stored_item_spec_params_mismatches
                    .keys()
                    .map(|item_spec_id| format!("* {item_spec_id}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }
    }

    items.join("\n")
}
