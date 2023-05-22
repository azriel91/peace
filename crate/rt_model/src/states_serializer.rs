use std::{marker::PhantomData, path::Path};

use peace_cfg::{FlowId, ItemId};
use peace_resources::{
    paths::{StatesDesiredFile, StatesSavedFile},
    states::{
        ts::{Desired, Saved},
        States, StatesDesired, StatesSaved,
    },
    type_reg::untagged::{BoxDtDisplay, TypeReg},
};

use crate::{Error, Storage};

/// Reads and writes [`StatesSaved`] and [`StatesDesired`] to and from storage.
pub struct StatesSerializer<E>(PhantomData<E>);

impl<E> StatesSerializer<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Returns the [`StatesSaved`] of all [`Item`]s if it exists on disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states`: States to serialize.
    /// * `states_file_path`: Path to save the serialized states to.
    ///
    /// [`Item`]: peace_cfg::Item
    pub async fn serialize<TS>(
        storage: &Storage,
        states: &States<TS>,
        states_file_path: &Path,
    ) -> Result<(), E>
    where
        TS: Send + Sync,
    {
        storage
            .serialized_write(
                #[cfg(not(target_arch = "wasm32"))]
                "StatesSerializer::serialize".to_string(),
                states_file_path,
                states,
                Error::StatesSerialize,
            )
            .await?;

        Ok(())
    }

    /// Returns the [`StatesSaved`] of all [`Item`]s if it exists on disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item state.
    /// * `states_saved_file`: `StatesSavedFile` to deserialize.
    ///
    /// [`Item`]: peace_cfg::Item
    pub async fn deserialize_saved(
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemId, BoxDtDisplay>,
        states_saved_file: &StatesSavedFile,
    ) -> Result<StatesSaved, E> {
        let states = Self::deserialize_internal::<Saved>(
            #[cfg(not(target_arch = "wasm32"))]
            "StatesSerializer::deserialize_saved".to_string(),
            flow_id,
            storage,
            states_type_reg,
            states_saved_file,
        )
        .await?;

        states.ok_or_else(|| E::from(Error::StatesCurrentDiscoverRequired))
    }

    /// Returns the [`StatesDesired`] of all [`Item`]s if it exists on disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item state.
    /// * `states_desired_file`: `StatesDesiredFile` to deserialize.
    ///
    /// [`Item`]: peace_cfg::Item
    pub async fn deserialize_desired(
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemId, BoxDtDisplay>,
        states_desired_file: &StatesDesiredFile,
    ) -> Result<StatesDesired, E> {
        let states = Self::deserialize_internal::<Desired>(
            #[cfg(not(target_arch = "wasm32"))]
            "StatesSerializer::deserialize_desired".to_string(),
            flow_id,
            storage,
            states_type_reg,
            states_desired_file,
        )
        .await?;

        states.ok_or_else(|| E::from(Error::StatesDesiredDiscoverRequired))
    }

    /// Returns the [`StatesSaved`] of all [`Item`]s if it exists on disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item state.
    /// * `states_saved_file`: `StatesSavedFile` to deserialize.
    ///
    /// [`Item`]: peace_cfg::Item
    pub async fn deserialize_saved_opt(
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemId, BoxDtDisplay>,
        states_saved_file: &StatesSavedFile,
    ) -> Result<Option<StatesSaved>, E> {
        Self::deserialize_internal(
            #[cfg(not(target_arch = "wasm32"))]
            "StatesSerializer::deserialize_saved_opt".to_string(),
            flow_id,
            storage,
            states_type_reg,
            states_saved_file,
        )
        .await
    }

    /// Returns the [`States`] of all [`Item`]s if it exists on disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item state.
    /// * `states_saved_file`: `StatesSavedFile` to deserialize.
    ///
    /// # Type Parameters
    ///
    /// * `TS`: The states type state to use, such as [`ts::Current`] or
    ///   [`ts::Saved`].
    ///
    /// [`Item`]: peace_cfg::Item
    /// [`ts::Current`]: peace_resources::states::ts::Current
    /// [`ts::Saved`]: peace_resources::states::ts::Saved
    #[cfg(not(target_arch = "wasm32"))]
    async fn deserialize_internal<TS>(
        thread_name: String,
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemId, BoxDtDisplay>,
        states_file_path: &Path,
    ) -> Result<Option<States<TS>>, E>
    where
        TS: Send + Sync,
    {
        let states_opt = storage
            .serialized_typemap_read_opt(thread_name, states_type_reg, states_file_path, |error| {
                #[cfg(not(feature = "error_reporting"))]
                {
                    Error::StatesDeserialize {
                        flow_id: flow_id.clone(),
                        error,
                    }
                }
                #[cfg(feature = "error_reporting")]
                {
                    use miette::NamedSource;

                    let file_contents = std::fs::read_to_string(states_file_path).unwrap();

                    let (error_span, error_message, context_span) =
                        crate::yaml_error_context_hack::error_and_context(&file_contents, &error);
                    let states_file_source =
                        NamedSource::new(states_file_path.to_string_lossy(), file_contents);

                    Error::StatesDeserialize {
                        flow_id: flow_id.clone(),
                        states_file_source,
                        error_span,
                        error_message,
                        context_span,
                        error,
                    }
                }
            })
            .await?;

        Ok(states_opt)
    }

    /// Returns the [`States`] of all [`Item`]s if it exists on disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item state.
    /// * `states_saved_file`: `StatesSavedFile` to deserialize.
    ///
    /// # Type Parameters
    ///
    /// * `TS`: The states type state to use, such as [`ts::Current`] or
    ///   [`ts::Saved`].
    ///
    /// [`Item`]: peace_cfg::Item
    /// [`ts::Current`]: peace_resources::states::ts::Current
    /// [`ts::Saved`]: peace_resources::states::ts::Saved
    #[cfg(target_arch = "wasm32")]
    async fn deserialize_internal<TS>(
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemId, BoxDtDisplay>,
        states_file_path: &Path,
    ) -> Result<Option<States<TS>>, E>
    where
        TS: Send + Sync,
    {
        let states_opt = storage
            .serialized_typemap_read_opt(states_type_reg, states_file_path, |error| {
                #[cfg(not(feature = "error_reporting"))]
                {
                    Error::StatesDeserialize {
                        flow_id: flow_id.clone(),
                        error,
                    }
                }
                #[cfg(feature = "error_reporting")]
                {
                    use miette::NamedSource;

                    let file_contents = std::fs::read_to_string(states_file_path).unwrap();

                    let (error_span, error_message, context_span) =
                        crate::yaml_error_context_hack::error_and_context(&file_contents, &error);
                    let states_file_source =
                        NamedSource::new(states_file_path.to_string_lossy(), file_contents);

                    Error::StatesDeserialize {
                        flow_id: flow_id.clone(),
                        states_file_source,
                        error_span,
                        error_message,
                        context_span,
                        error,
                    }
                }
            })
            .await?;

        Ok(states_opt)
    }
}
