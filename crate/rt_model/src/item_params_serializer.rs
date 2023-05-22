use std::marker::PhantomData;

use peace_cfg::{FlowId, ItemId, Profile};
use peace_resources::{
    paths::ItemParamsFile,
    type_reg::untagged::{BoxDt, TypeReg},
};

use crate::{Error, ItemParams, Storage};

/// Reads and writes [`ItemParams`] to and from storage.
pub struct ItemParamsSerializer<E>(PhantomData<E>);

impl<E> ItemParamsSerializer<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Returns the [`ItemParams`] of all [`Item`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_params`: ItemParams to serialize.
    /// * `item_params_file`: Path to save the serialized item_params to.
    ///
    /// [`Item`]: peace_cfg::Item
    pub async fn serialize(
        storage: &Storage,
        item_params: &ItemParams,
        item_params_file: &ItemParamsFile,
    ) -> Result<(), E> {
        storage
            .serialized_write(
                #[cfg(not(target_arch = "wasm32"))]
                "ItemParamsSerializer::serialize".to_string(),
                item_params_file,
                item_params,
                Error::StatesSerialize,
            )
            .await?;

        Ok(())
    }

    /// Returns the [`ItemParams`] of all [`Item`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_params_type_reg`: Type registry with functions to deserialize
    ///   each item state.
    /// * `item_params_file`: `ItemParamsFile` to deserialize.
    ///
    /// [`Item`]: peace_cfg::Item
    pub async fn deserialize(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        item_params_type_reg: &TypeReg<ItemId, BoxDt>,
        item_params_file: &ItemParamsFile,
    ) -> Result<ItemParams, E> {
        let item_params = Self::deserialize_internal(
            #[cfg(not(target_arch = "wasm32"))]
            "ItemParamsSerializer::deserialize".to_string(),
            profile,
            flow_id,
            storage,
            item_params_type_reg,
            item_params_file,
        )
        .await?;

        item_params.ok_or_else(|| {
            E::from(Error::ItemParamsFileNotExists {
                profile: profile.clone(),
                flow_id: flow_id.clone(),
                item_params_file: item_params_file.clone(),
            })
        })
    }

    /// Returns the [`ItemParams`] of all [`Item`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_params_type_reg`: Type registry with functions to deserialize
    ///   each item state.
    /// * `item_params_file`: `ItemParamsFile` to deserialize.
    ///
    /// [`Item`]: peace_cfg::Item
    pub async fn deserialize_opt(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        item_params_type_reg: &TypeReg<ItemId, BoxDt>,
        item_params_file: &ItemParamsFile,
    ) -> Result<Option<ItemParams>, E> {
        Self::deserialize_internal(
            #[cfg(not(target_arch = "wasm32"))]
            "ItemParamsSerializer::deserialize_opt".to_string(),
            profile,
            flow_id,
            storage,
            item_params_type_reg,
            item_params_file,
        )
        .await
    }

    /// Returns the [`ItemParams`] of all [`Item`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_params_type_reg`: Type registry with functions to deserialize
    ///   each item state.
    /// * `item_params_file`: `ItemParamsFile` to deserialize.
    ///
    /// [`Item`]: peace_cfg::Item
    #[cfg(not(target_arch = "wasm32"))]
    async fn deserialize_internal(
        thread_name: String,
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        item_params_type_reg: &TypeReg<ItemId, BoxDt>,
        item_params_file: &ItemParamsFile,
    ) -> Result<Option<ItemParams>, E> {
        let item_params_opt = storage
            .serialized_typemap_read_opt(
                thread_name,
                item_params_type_reg,
                item_params_file,
                |error| {
                    #[cfg(not(feature = "error_reporting"))]
                    {
                        Error::ItemParamsDeserialize {
                            profile: profile.clone(),
                            flow_id: flow_id.clone(),
                            error,
                        }
                    }
                    #[cfg(feature = "error_reporting")]
                    {
                        use miette::NamedSource;

                        let file_contents = std::fs::read_to_string(item_params_file).unwrap();

                        let (error_span, error_message, context_span) =
                            crate::yaml_error_context_hack::error_and_context(
                                &file_contents,
                                &error,
                            );
                        let item_params_file_source =
                            NamedSource::new(item_params_file.to_string_lossy(), file_contents);

                        Error::ItemParamsDeserialize {
                            profile: profile.clone(),
                            flow_id: flow_id.clone(),
                            item_params_file_source,
                            error_span,
                            error_message,
                            context_span,
                            error,
                        }
                    }
                },
            )
            .await?;

        Ok(item_params_opt)
    }

    /// Returns the [`ItemParams`] of all [`Item`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `item_params_type_reg`: Type registry with functions to deserialize
    ///   each item state.
    /// * `item_params_file`: `ItemParamsFile` to deserialize.
    ///
    /// [`Item`]: peace_cfg::Item
    #[cfg(target_arch = "wasm32")]
    async fn deserialize_internal(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        item_params_type_reg: &TypeReg<ItemId, BoxDt>,
        item_params_file: &ItemParamsFile,
    ) -> Result<Option<ItemParams>, E> {
        let item_params_opt = storage
            .serialized_typemap_read_opt(item_params_type_reg, item_params_file, |error| {
                #[cfg(not(feature = "error_reporting"))]
                {
                    Error::ItemParamsDeserialize {
                        profile: profile.clone(),
                        flow_id: flow_id.clone(),
                        error,
                    }
                }
                #[cfg(feature = "error_reporting")]
                {
                    use miette::NamedSource;

                    let file_contents = std::fs::read_to_string(item_params_file).unwrap();

                    let (error_span, error_message, context_span) =
                        crate::yaml_error_context_hack::error_and_context(&file_contents, &error);
                    let item_params_file_source =
                        NamedSource::new(item_params_file.to_string_lossy(), file_contents);

                    Error::ItemParamsDeserialize {
                        profile: profile.clone(),
                        flow_id: flow_id.clone(),
                        item_params_file_source,
                        error_span,
                        error_message,
                        context_span,
                        error,
                    }
                }
            })
            .await?;

        Ok(item_params_opt)
    }
}
