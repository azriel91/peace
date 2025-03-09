use futures::{StreamExt, TryStreamExt};
use peace_flow_rt::{Flow, ItemGraph};
use peace_item_model::ItemId;
use peace_params::{ParamsKey, ParamsSpecs};
use peace_resource_rt::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    paths::ParamsSpecsFile,
    resources::ts::{Empty, SetUp},
    Resource, Resources,
};
use peace_rt_model::{
    params::{FlowParams, ProfileParams, WorkspaceParams},
    ParamsSpecsSerializer, ParamsSpecsTypeReg, StatesTypeReg, Storage, WorkspaceInitializer,
};
use type_reg::untagged::{BoxDt, TypeReg};

/// Common code used to build different `CmdCtx*` types.
pub(crate) struct CmdCtxBuilderSupport;

impl CmdCtxBuilderSupport {
    /// Merges workspace params provided by the caller with the workspace params
    /// on disk.
    ///
    /// Type registry for [`WorkspaceParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    pub(crate) async fn workspace_params_merge<WorkspaceParamsK>(
        storage: &peace_rt_model::Storage,
        workspace_params_type_reg: &TypeReg<WorkspaceParamsK, BoxDt>,
        params: &mut peace_rt_model::params::WorkspaceParams<WorkspaceParamsK>,
        workspace_params_file: &peace_resource_rt::internal::WorkspaceParamsFile,
    ) -> Result<(), peace_rt_model::Error>
    where
        WorkspaceParamsK: ParamsKey,
    {
        let params_deserialized =
            peace_rt_model::WorkspaceInitializer::workspace_params_deserialize::<WorkspaceParamsK>(
                storage,
                workspace_params_type_reg,
                workspace_params_file,
            )
            .await?;
        match params_deserialized {
            Some(params_deserialized) => {
                // Merge `params` on top of `params_deserialized`.
                // or, copy `params_deserialized` to `params` where
                // there isn't a value.

                if params.is_empty() {
                    *params = params_deserialized;
                } else {
                    params_deserialized
                        .into_inner()
                        .into_inner()
                        .into_iter()
                        .for_each(|(key, param)| {
                            if !params.contains_key(&key) {
                                params.insert_raw(key, param);
                            }
                        });
                }
            }
            None => {}
        }

        Ok(())
    }

    /// Merges profile params provided by the caller with the profile params
    /// on disk.
    ///
    /// Type registry for [`ProfileParams`] deserialization.
    ///
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    pub(crate) async fn profile_params_merge<ProfileParamsK>(
        storage: &peace_rt_model::Storage,
        profile_params_type_reg: &TypeReg<ProfileParamsK, BoxDt>,
        params: &mut peace_rt_model::params::ProfileParams<ProfileParamsK>,
        profile_params_file: &peace_resource_rt::internal::ProfileParamsFile,
    ) -> Result<(), peace_rt_model::Error>
    where
        ProfileParamsK: ParamsKey,
    {
        let params_deserialized =
            peace_rt_model::WorkspaceInitializer::profile_params_deserialize::<ProfileParamsK>(
                storage,
                profile_params_type_reg,
                profile_params_file,
            )
            .await?;
        match params_deserialized {
            Some(params_deserialized) => {
                // Merge `params` on top of `params_deserialized`.
                // or, copy `params_deserialized` to `params` where
                // there isn't a value.

                if params.is_empty() {
                    *params = params_deserialized;
                } else {
                    params_deserialized
                        .into_inner()
                        .into_inner()
                        .into_iter()
                        .for_each(|(key, param)| {
                            if !params.contains_key(&key) {
                                params.insert_raw(key, param);
                            }
                        });
                }
            }
            None => {}
        }

        Ok(())
    }

    /// Merges flow params provided by the caller with the flow params
    /// on disk.
    ///
    /// Type registry for [`FlowParams`] deserialization.
    ///
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    pub(crate) async fn flow_params_merge<FlowParamsK>(
        storage: &peace_rt_model::Storage,
        flow_params_type_reg: &TypeReg<FlowParamsK, BoxDt>,
        params: &mut peace_rt_model::params::FlowParams<FlowParamsK>,
        flow_params_file: &peace_resource_rt::internal::FlowParamsFile,
    ) -> Result<(), peace_rt_model::Error>
    where
        FlowParamsK: ParamsKey,
    {
        let params_deserialized = peace_rt_model::WorkspaceInitializer::flow_params_deserialize::<
            FlowParamsK,
        >(storage, flow_params_type_reg, flow_params_file)
        .await?;
        match params_deserialized {
            Some(params_deserialized) => {
                // Merge `params` on top of `params_deserialized`.
                // or, copy `params_deserialized` to `params` where
                // there isn't a value.

                if params.is_empty() {
                    *params = params_deserialized;
                } else {
                    params_deserialized
                        .into_inner()
                        .into_inner()
                        .into_iter()
                        .for_each(|(key, param)| {
                            if !params.contains_key(&key) {
                                params.insert_raw(key, param);
                            }
                        });
                }
            }
            None => {}
        }

        Ok(())
    }

    /// Serializes workspace params to storage.
    pub(crate) async fn workspace_params_serialize<WorkspaceParamsK>(
        workspace_params: &WorkspaceParams<WorkspaceParamsK>,
        storage: &Storage,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<(), peace_rt_model::Error>
    where
        WorkspaceParamsK: ParamsKey,
    {
        WorkspaceInitializer::workspace_params_serialize(
            storage,
            workspace_params,
            workspace_params_file,
        )
        .await?;

        Ok(())
    }

    /// Inserts workspace params into the `Resources` map.
    pub(crate) fn workspace_params_insert<WorkspaceParamsK>(
        mut workspace_params: WorkspaceParams<WorkspaceParamsK>,
        resources: &mut Resources<Empty>,
    ) where
        WorkspaceParamsK: ParamsKey,
    {
        workspace_params
            .drain(..)
            .for_each(|(_key, workspace_param)| {
                let workspace_param = workspace_param.into_inner().upcast();
                let type_id = Resource::type_id(&*workspace_param);
                resources.insert_raw(type_id, workspace_param);
            });
    }

    /// Serializes profile params to storage.
    pub(crate) async fn profile_params_serialize<ProfileParamsK>(
        profile_params: &ProfileParams<ProfileParamsK>,
        storage: &Storage,
        profile_params_file: &ProfileParamsFile,
    ) -> Result<(), peace_rt_model::Error>
    where
        ProfileParamsK: ParamsKey,
    {
        WorkspaceInitializer::profile_params_serialize(
            storage,
            profile_params,
            profile_params_file,
        )
        .await?;

        Ok(())
    }

    /// Inserts profile params into the `Resources` map.
    pub(crate) fn profile_params_insert<ProfileParamsK>(
        mut profile_params: ProfileParams<ProfileParamsK>,
        resources: &mut Resources<Empty>,
    ) where
        ProfileParamsK: ParamsKey,
    {
        profile_params.drain(..).for_each(|(_key, profile_param)| {
            let profile_param = profile_param.into_inner().upcast();
            let type_id = Resource::type_id(&*profile_param);
            resources.insert_raw(type_id, profile_param);
        });
    }

    /// Serializes flow params to storage.
    pub(crate) async fn flow_params_serialize<FlowParamsK>(
        flow_params: &FlowParams<FlowParamsK>,
        storage: &Storage,
        flow_params_file: &FlowParamsFile,
    ) -> Result<(), peace_rt_model::Error>
    where
        FlowParamsK: ParamsKey,
    {
        WorkspaceInitializer::flow_params_serialize(storage, flow_params, flow_params_file).await?;

        Ok(())
    }

    /// Serializes item params to storage.
    pub(crate) async fn params_specs_serialize(
        params_specs: &ParamsSpecs,
        storage: &Storage,
        params_specs_file: &ParamsSpecsFile,
    ) -> Result<(), peace_rt_model::Error> {
        ParamsSpecsSerializer::serialize(storage, params_specs, params_specs_file).await
    }

    /// Inserts flow params into the `Resources` map.
    pub(crate) fn flow_params_insert<FlowParamsK>(
        mut flow_params: FlowParams<FlowParamsK>,
        resources: &mut Resources<Empty>,
    ) where
        FlowParamsK: ParamsKey,
    {
        flow_params.drain(..).for_each(|(_key, flow_param)| {
            let flow_param = flow_param.into_inner().upcast();
            let type_id = Resource::type_id(&*flow_param);
            resources.insert_raw(type_id, flow_param);
        });
    }

    /// Registers each item's `Params` and `State` for stateful
    /// deserialization.
    pub(crate) fn params_and_states_type_reg<E>(
        item_graph: &ItemGraph<E>,
    ) -> (ParamsSpecsTypeReg, StatesTypeReg)
    where
        E: 'static,
    {
        item_graph.iter().fold(
            (ParamsSpecsTypeReg::new(), StatesTypeReg::new()),
            |(mut params_specs_type_reg, mut states_type_reg), item| {
                item.params_and_state_register(&mut params_specs_type_reg, &mut states_type_reg);

                (params_specs_type_reg, states_type_reg)
            },
        )
    }

    /// Merges provided item parameters with previously stored item
    /// parameters.
    ///
    /// If an item's parameters are not provided, and nothing was previously
    /// stored, then an error is returned.
    pub(crate) fn params_specs_merge<E>(
        flow: &Flow<E>,
        mut params_specs_provided: ParamsSpecs,
        params_specs_stored: Option<ParamsSpecs>,
    ) -> Result<ParamsSpecs, peace_rt_model::Error>
    where
        E: From<peace_rt_model::Error> + 'static,
    {
        // Combine provided and stored params specs. Provided params specs take
        // precedence.
        //
        // We construct a new TypeMap because we want to make sure params specs are
        // serialized in order of the items in the graph.
        let item_graph = flow.graph();
        let mut params_specs = ParamsSpecs::with_capacity(item_graph.node_count());

        // Collected erroneous data -- parameters may have been valid in the past, but:
        //
        // * item IDs may have changed.
        // * items may have been removed, but params specs remain.
        // * items may have been added, but params specs forgotten to be added.
        let mut item_ids_with_no_params_specs = Vec::<ItemId>::new();
        let mut params_specs_stored_mismatches = None;
        let mut params_specs_not_usable = Vec::<ItemId>::new();

        if let Some(mut params_specs_stored) = params_specs_stored {
            item_graph.iter_insertion().for_each(|item_rt| {
                let item_id = item_rt.id();

                // Removing the entry from stored params specs is deliberate, so filtering for
                // stored params specs that no longer have a corresponding item are
                // detected.
                let params_spec_provided = params_specs_provided.shift_remove_entry(item_id);
                let params_spec_stored = params_specs_stored.shift_remove_entry(item_id);

                // Deep merge params specs.
                let params_spec_to_use = match (params_spec_provided, params_spec_stored) {
                    (None, None) => None,
                    (None, Some(params_spec_stored)) => Some(params_spec_stored),

                    // Newly added item, or potentially renamed.
                    (Some(params_spec_provided), None) => Some(params_spec_provided),

                    (
                        Some((item_id, mut params_spec_provided)),
                        Some((_item_id, params_spec_stored)),
                    ) => {
                        params_spec_provided.merge(&*params_spec_stored);
                        Some((item_id, params_spec_provided))
                    }
                };

                if let Some((item_id, params_spec_boxed)) = params_spec_to_use {
                    // `*Spec::MappingFn`s will be present in `params_spec_stored`, but will not
                    // be valid mapping functions as they cannot be serialized / deserialized.
                    //
                    // Also, field wise `ParamsSpec`s may contain `ValueSpec::Stored` for fields
                    // which never had specifications, which are also unusable.
                    if params_spec_boxed.is_usable() {
                        params_specs.insert_raw(item_id, params_spec_boxed);
                    } else {
                        params_specs_not_usable.push(item_id);
                    }
                } else {
                    // Collect items that do not have parameters.
                    item_ids_with_no_params_specs.push(item_id.clone());
                }
            });

            // Stored parameters whose IDs do not correspond to any item IDs in the
            // graph. May be empty.
            params_specs_stored_mismatches = Some(params_specs_stored);
        } else {
            item_graph.iter_insertion().for_each(|item_rt| {
                let item_id = item_rt.id();

                if let Some((item_id, params_spec_boxed)) =
                    params_specs_provided.shift_remove_entry(item_id)
                {
                    params_specs.insert_raw(item_id, params_spec_boxed);
                } else {
                    // Collect items that do not have parameters.
                    item_ids_with_no_params_specs.push(item_id.clone());
                }
            });
        }

        // Provided parameters whose IDs do not correspond to any item IDs in the
        // graph.
        let params_specs_provided_mismatches = params_specs_provided;

        let params_no_issues = item_ids_with_no_params_specs.is_empty()
            && params_specs_provided_mismatches.is_empty()
            && params_specs_stored_mismatches
                .as_ref()
                .map(|params_specs_stored_mismatches| params_specs_stored_mismatches.is_empty())
                .unwrap_or(true)
            && params_specs_not_usable.is_empty();

        if params_no_issues {
            Ok(params_specs)
        } else {
            let params_specs_stored_mismatches = Box::new(params_specs_stored_mismatches);
            Err(peace_rt_model::Error::ParamsSpecsMismatch {
                item_ids_with_no_params_specs,
                params_specs_provided_mismatches,
                params_specs_stored_mismatches,
                params_specs_not_usable,
            })
        }
    }

    pub(crate) async fn item_graph_setup<E>(
        item_graph: &ItemGraph<E>,
        resources: Resources<Empty>,
    ) -> Result<Resources<SetUp>, E>
    where
        E: std::error::Error + 'static,
    {
        let resources = item_graph
            .stream()
            .map(Ok::<_, E>)
            .try_fold(resources, |mut resources, item| async move {
                item.setup(&mut resources).await?;
                Ok(resources)
            })
            .await?;

        Ok(Resources::<SetUp>::from(resources))
    }
}
