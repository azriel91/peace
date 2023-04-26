use std::marker::PhantomData;

use peace_cfg::{FlowId, ItemSpecId, Profile};
use peace_params::ParamsSpecs;
use peace_resources::{
    paths::ParamsSpecsFile,
    type_reg::untagged::{BoxDt, TypeReg},
};

use crate::{Error, Storage};

/// Reads and writes [`ParamsSpecs`] to and from storage.
pub struct ParamsSpecsSerializer<E>(PhantomData<E>);

impl<E> ParamsSpecsSerializer<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Returns the [`ParamsSpecs`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `params_specs`: ParamsSpecs to serialize.
    /// * `params_specs_file`: Path to save the serialized params_specs to.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn serialize(
        storage: &Storage,
        params_specs: &ParamsSpecs,
        params_specs_file: &ParamsSpecsFile,
    ) -> Result<(), E> {
        storage
            .serialized_write(
                #[cfg(not(target_arch = "wasm32"))]
                "ParamsSpecsSerializer::serialize".to_string(),
                params_specs_file,
                params_specs,
                Error::StatesSerialize,
            )
            .await?;

        Ok(())
    }

    /// Returns the [`ParamsSpecs`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `params_specs_type_reg`: Type registry with functions to deserialize
    ///   each params spec.
    /// * `params_specs_file`: `ParamsSpecsFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn deserialize(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        params_specs_type_reg: &TypeReg<ItemSpecId, BoxDt>,
        params_specs_file: &ParamsSpecsFile,
    ) -> Result<ParamsSpecs, E> {
        let params_specs = Self::deserialize_internal(
            #[cfg(not(target_arch = "wasm32"))]
            "ParamsSpecsSerializer::deserialize".to_string(),
            profile,
            flow_id,
            storage,
            params_specs_type_reg,
            params_specs_file,
        )
        .await?;

        params_specs.ok_or_else(|| {
            E::from(Error::ParamsSpecsFileNotExists {
                profile: profile.clone(),
                flow_id: flow_id.clone(),
                params_specs_file: params_specs_file.clone(),
            })
        })
    }

    /// Returns the [`ParamsSpecs`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `params_specs_type_reg`: Type registry with functions to deserialize
    ///   each params spec.
    /// * `params_specs_file`: `ParamsSpecsFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn deserialize_opt(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        params_specs_type_reg: &TypeReg<ItemSpecId, BoxDt>,
        params_specs_file: &ParamsSpecsFile,
    ) -> Result<Option<ParamsSpecs>, E> {
        Self::deserialize_internal(
            #[cfg(not(target_arch = "wasm32"))]
            "ParamsSpecsSerializer::deserialize_opt".to_string(),
            profile,
            flow_id,
            storage,
            params_specs_type_reg,
            params_specs_file,
        )
        .await
    }

    /// Returns the [`ParamsSpecs`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `params_specs_type_reg`: Type registry with functions to deserialize
    ///   each params spec.
    /// * `params_specs_file`: `ParamsSpecsFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    #[cfg(not(target_arch = "wasm32"))]
    async fn deserialize_internal(
        thread_name: String,
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        params_specs_type_reg: &TypeReg<ItemSpecId, BoxDt>,
        params_specs_file: &ParamsSpecsFile,
    ) -> Result<Option<ParamsSpecs>, E> {
        let params_specs_opt = storage
            .serialized_typemap_read_opt(
                thread_name,
                params_specs_type_reg,
                params_specs_file,
                |error| {
                    #[cfg(not(feature = "error_reporting"))]
                    {
                        Error::ParamsSpecsDeserialize {
                            profile: profile.clone(),
                            flow_id: flow_id.clone(),
                            error,
                        }
                    }
                    #[cfg(feature = "error_reporting")]
                    {
                        use miette::NamedSource;

                        let file_contents = std::fs::read_to_string(params_specs_file).unwrap();

                        let (error_span, error_message, context_span) =
                            crate::yaml_error_context_hack::error_and_context(
                                &file_contents,
                                &error,
                            );
                        let params_specs_file_source =
                            NamedSource::new(params_specs_file.to_string_lossy(), file_contents);

                        Error::ParamsSpecsDeserialize {
                            profile: profile.clone(),
                            flow_id: flow_id.clone(),
                            params_specs_file_source,
                            error_span,
                            error_message,
                            context_span,
                            error,
                        }
                    }
                },
            )
            .await?;

        Ok(params_specs_opt)
    }

    /// Returns the [`ParamsSpecs`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `params_specs_type_reg`: Type registry with functions to deserialize
    ///   each params spec.
    /// * `params_specs_file`: `ParamsSpecsFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    #[cfg(target_arch = "wasm32")]
    async fn deserialize_internal(
        profile: &Profile,
        flow_id: &FlowId,
        storage: &Storage,
        params_specs_type_reg: &TypeReg<ItemSpecId, BoxDt>,
        params_specs_file: &ParamsSpecsFile,
    ) -> Result<Option<ParamsSpecs>, E> {
        let params_specs_opt = storage
            .serialized_typemap_read_opt(params_specs_type_reg, params_specs_file, |error| {
                #[cfg(not(feature = "error_reporting"))]
                {
                    Error::ParamsSpecsDeserialize {
                        flow_id: flow_id.clone(),
                        error,
                    }
                }
                #[cfg(feature = "error_reporting")]
                {
                    use miette::NamedSource;

                    let file_contents = std::fs::read_to_string(params_specs_file).unwrap();

                    let (error_span, error_message, context_span) =
                        crate::yaml_error_context_hack::error_and_context(&file_contents, &error);
                    let params_specs_file_source =
                        NamedSource::new(params_specs_file.to_string_lossy(), file_contents);

                    Error::ParamsSpecsDeserialize {
                        profile: profile.clone(),
                        flow_id: flow_id.clone(),
                        params_specs_file_source,
                        error_span,
                        error_message,
                        context_span,
                        error,
                    }
                }
            })
            .await?;

        Ok(params_specs_opt)
    }
}
