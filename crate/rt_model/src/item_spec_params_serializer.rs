use std::marker::PhantomData;

use peace_cfg::{FlowId, ItemSpecId, Profile};
use peace_resources::{
    paths::ItemSpecParamsFile,
    type_reg::untagged::{BoxDt, TypeReg},
};

use crate::{Error, ItemSpecParams, Storage};

/// Reads and writes [`ItemSpecParams`] to and from storage.
pub struct ItemSpecParamsSerializer<E>(PhantomData<E>);

impl<E> ItemSpecParamsSerializer<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Returns the [`ItemSpecParams`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_spec_params`: ItemSpecParams to serialize.
    /// * `item_spec_params_file`: Path to save the serialized item_spec_params
    ///   to.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn serialize(
        storage: &Storage,
        item_spec_params: &ItemSpecParams,
        item_spec_params_file: &ItemSpecParamsFile,
    ) -> Result<(), E> {
        storage
            .serialized_write(
                #[cfg(not(target_arch = "wasm32"))]
                "ItemSpecParamsSerializer::serialize".to_string(),
                item_spec_params_file,
                item_spec_params,
                Error::StatesSerialize,
            )
            .await?;

        Ok(())
    }

    /// Returns the [`ItemSpecParams`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_spec_params_type_reg`: Type registry with functions to
    ///   deserialize each item spec state.
    /// * `item_spec_params_file`: `ItemSpecParamsFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn deserialize(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        item_spec_params_type_reg: &TypeReg<ItemSpecId, BoxDt>,
        item_spec_params_file: &ItemSpecParamsFile,
    ) -> Result<ItemSpecParams, E> {
        let item_spec_params = Self::deserialize_internal(
            #[cfg(not(target_arch = "wasm32"))]
            "ItemSpecParamsSerializer::deserialize".to_string(),
            profile,
            flow_id,
            storage,
            item_spec_params_type_reg,
            item_spec_params_file,
        )
        .await?;

        item_spec_params.ok_or_else(|| {
            E::from(Error::ItemSpecParamsFileNotExists {
                profile: profile.clone(),
                flow_id: flow_id.clone(),
                item_spec_params_file: item_spec_params_file.clone(),
            })
        })
    }

    /// Returns the [`ItemSpecParams`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_spec_params_type_reg`: Type registry with functions to
    ///   deserialize each item spec state.
    /// * `item_spec_params_file`: `ItemSpecParamsFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn deserialize_opt(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        item_spec_params_type_reg: &TypeReg<ItemSpecId, BoxDt>,
        item_spec_params_file: &ItemSpecParamsFile,
    ) -> Result<Option<ItemSpecParams>, E> {
        Self::deserialize_internal(
            #[cfg(not(target_arch = "wasm32"))]
            "ItemSpecParamsSerializer::deserialize_opt".to_string(),
            profile,
            flow_id,
            storage,
            item_spec_params_type_reg,
            item_spec_params_file,
        )
        .await
    }

    /// Returns the [`ItemSpecParams`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_spec_params_type_reg`: Type registry with functions to
    ///   deserialize each item spec state.
    /// * `item_spec_params_file`: `ItemSpecParamsFile` to deserialize.
    ///
    /// # Type Parameters
    ///
    /// * `TS`: The item_spec_params type state to use, such as [`ts::Current`]
    ///   or [`ts::Saved`].
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ts::Current`]: peace_resources::item_spec_params::ts::Current
    /// [`ts::Saved`]: peace_resources::item_spec_params::ts::Saved
    #[cfg(not(target_arch = "wasm32"))]
    async fn deserialize_internal(
        thread_name: String,
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        item_spec_params_type_reg: &TypeReg<ItemSpecId, BoxDt>,
        item_spec_params_file: &ItemSpecParamsFile,
    ) -> Result<Option<ItemSpecParams>, E> {
        let item_spec_params_opt = storage
            .serialized_typemap_read_opt(
                thread_name,
                item_spec_params_type_reg,
                item_spec_params_file,
                |error| {
                    #[cfg(not(feature = "error_reporting"))]
                    {
                        Error::ItemSpecParamsDeserialize {
                            profile: profile.clone(),
                            flow_id: flow_id.clone(),
                            error,
                        }
                    }
                    #[cfg(feature = "error_reporting")]
                    {
                        use miette::NamedSource;

                        let file_contents = std::fs::read_to_string(item_spec_params_file).unwrap();

                        let (error_span, error_message, context_span) =
                            crate::yaml_error_context_hack::error_and_context(
                                &file_contents,
                                &error,
                            );
                        let item_spec_params_file_source = NamedSource::new(
                            item_spec_params_file.to_string_lossy(),
                            file_contents,
                        );

                        Error::ItemSpecParamsDeserialize {
                            profile: profile.clone(),
                            flow_id: flow_id.clone(),
                            item_spec_params_file_source,
                            error_span,
                            error_message,
                            context_span,
                            error,
                        }
                    }
                },
            )
            .await?;

        Ok(item_spec_params_opt)
    }

    /// Returns the [`ItemSpecParams`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_spec_params_type_reg`: Type registry with functions to
    ///   deserialize each item spec state.
    /// * `item_spec_params_file`: `ItemSpecParamsFile` to deserialize.
    ///
    /// # Type Parameters
    ///
    /// * `TS`: The item_spec_params type state to use, such as [`ts::Current`]
    ///   or [`ts::Saved`].
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ts::Current`]: peace_resources::item_spec_params::ts::Current
    /// [`ts::Saved`]: peace_resources::item_spec_params::ts::Saved
    #[cfg(target_arch = "wasm32")]
    async fn deserialize_internal(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        item_spec_params_type_reg: &TypeReg<ItemSpecId, BoxDt>,
        item_spec_params_file: &ItemSpecParamsFile,
    ) -> Result<Option<ItemSpecParams>, E> {
        let item_spec_params_opt = storage
            .serialized_typemap_read_opt(
                item_spec_params_type_reg,
                item_spec_params_file,
                |error| {
                    #[cfg(not(feature = "error_reporting"))]
                    {
                        Error::ItemSpecParamsDeserialize {
                            flow_id: flow_id.clone(),
                            error,
                        }
                    }
                    #[cfg(feature = "error_reporting")]
                    {
                        use miette::NamedSource;

                        let file_contents = std::fs::read_to_string(item_spec_params_file).unwrap();

                        let (error_span, error_message, context_span) =
                            crate::yaml_error_context_hack::error_and_context(
                                &file_contents,
                                &error,
                            );
                        let item_spec_params_file_source = NamedSource::new(
                            item_spec_params_file.to_string_lossy(),
                            file_contents,
                        );

                        Error::ItemSpecParamsDeserialize {
                            profile: profile.clone(),
                            flow_id: flow_id.clone(),
                            item_spec_params_file_source,
                            error_span,
                            error_message,
                            context_span,
                            error,
                        }
                    }
                },
            )
            .await?;

        Ok(item_spec_params_opt)
    }
}
