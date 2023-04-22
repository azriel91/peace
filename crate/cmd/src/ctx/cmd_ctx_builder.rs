#![allow(clippy::type_complexity)]

use std::{fmt::Debug, hash::Hash};

use futures::stream::{StreamExt, TryStreamExt};
use peace_cfg::ItemSpecId;
use peace_resources::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    paths::ItemSpecParamsFile,
    resources::ts::{Empty, SetUp},
    Resources,
};
use peace_rt_model::{
    fn_graph::resman::Resource,
    params::{FlowParams, ProfileParams, WorkspaceParams},
    Error, Flow, ItemSpecGraph, ItemSpecParams, ItemSpecParamsTypeReg, StatesTypeReg, Storage,
    Workspace, WorkspaceInitializer,
};
use serde::{de::DeserializeOwned, Serialize};

pub use self::{
    multi_profile_no_flow_builder::MultiProfileNoFlowBuilder,
    multi_profile_single_flow_builder::MultiProfileSingleFlowBuilder,
    no_profile_no_flow_builder::NoProfileNoFlowBuilder,
    single_profile_no_flow_builder::SingleProfileNoFlowBuilder,
    single_profile_single_flow_builder::SingleProfileSingleFlowBuilder,
};

mod multi_profile_no_flow_builder;
mod multi_profile_single_flow_builder;
mod no_profile_no_flow_builder;
mod single_profile_no_flow_builder;
mod single_profile_single_flow_builder;

/// Collects parameters and initializes values relevant to the built [`CmdCtx`].
#[derive(Debug)]
pub struct CmdCtxBuilder<'ctx, O, ScopeBuilder> {
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    output: &'ctx mut O,
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Data held while building `CmdCtx`.
    scope_builder: ScopeBuilder,
}

/// Serializes workspace params to storage.
async fn workspace_params_serialize<WorkspaceParamsK>(
    workspace_params: &WorkspaceParams<WorkspaceParamsK>,
    storage: &Storage,
    workspace_params_file: &WorkspaceParamsFile,
) -> Result<(), Error>
where
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
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
fn workspace_params_insert<WorkspaceParamsK>(
    mut workspace_params: WorkspaceParams<WorkspaceParamsK>,
    resources: &mut Resources<Empty>,
) where
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
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
async fn profile_params_serialize<ProfileParamsK>(
    profile_params: &ProfileParams<ProfileParamsK>,
    storage: &Storage,
    profile_params_file: &ProfileParamsFile,
) -> Result<(), Error>
where
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    WorkspaceInitializer::profile_params_serialize(storage, profile_params, profile_params_file)
        .await?;

    Ok(())
}

/// Inserts profile params into the `Resources` map.
fn profile_params_insert<ProfileParamsK>(
    mut profile_params: ProfileParams<ProfileParamsK>,
    resources: &mut Resources<Empty>,
) where
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    profile_params.drain(..).for_each(|(_key, profile_param)| {
        let profile_param = profile_param.into_inner().upcast();
        let type_id = Resource::type_id(&*profile_param);
        resources.insert_raw(type_id, profile_param);
    });
}

/// Serializes flow params to storage.
async fn flow_params_serialize<FlowParamsK>(
    flow_params: &FlowParams<FlowParamsK>,
    storage: &Storage,
    flow_params_file: &FlowParamsFile,
) -> Result<(), Error>
where
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    WorkspaceInitializer::flow_params_serialize(storage, flow_params, flow_params_file).await?;

    Ok(())
}

/// Serializes item spec params to storage.
async fn item_spec_params_serialize(
    item_spec_params: &ItemSpecParams,
    storage: &Storage,
    item_spec_params_file: &ItemSpecParamsFile,
) -> Result<(), Error> {
    storage
        .serialized_write(
            #[cfg(not(target_arch = "wasm32"))]
            "item_spec_params_serialize".to_string(),
            item_spec_params_file,
            item_spec_params,
            Error::ItemSpecParamsSerialize,
        )
        .await
}

/// Inserts flow params into the `Resources` map.
fn flow_params_insert<FlowParamsK>(
    mut flow_params: FlowParams<FlowParamsK>,
    resources: &mut Resources<Empty>,
) where
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    flow_params.drain(..).for_each(|(_key, flow_param)| {
        let flow_param = flow_param.into_inner().upcast();
        let type_id = Resource::type_id(&*flow_param);
        resources.insert_raw(type_id, flow_param);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn profiles_from_peace_app_dir(
    peace_app_dir: &peace_resources::paths::PeaceAppDir,
    profiles_filter_fn: Option<&dyn Fn(&peace_core::Profile) -> bool>,
) -> Result<Vec<peace_core::Profile>, peace_rt_model::Error> {
    use std::{ffi::OsStr, str::FromStr};

    let mut profiles = Vec::new();
    let mut peace_app_read_dir = tokio::fs::read_dir(peace_app_dir).await.map_err(|error| {
        peace_rt_model::Error::Native(peace_rt_model::NativeError::PeaceAppDirRead {
            peace_app_dir: peace_app_dir.to_path_buf(),
            error,
        })
    })?;
    while let Some(entry) = peace_app_read_dir.next_entry().await.map_err(|error| {
        peace_rt_model::Error::Native(peace_rt_model::NativeError::PeaceAppDirEntryRead {
            peace_app_dir: peace_app_dir.to_path_buf(),
            error,
        })
    })? {
        let file_type = entry.file_type().await.map_err(|error| {
            peace_rt_model::Error::Native(
                peace_rt_model::NativeError::PeaceAppDirEntryFileTypeRead {
                    path: entry.path(),
                    error,
                },
            )
        })?;

        if file_type.is_dir() {
            let entry_path = entry.path();
            if let Some(dir_name) = entry_path.file_name().and_then(OsStr::to_str) {
                // Assume this is a profile directory
                let profile = peace_core::Profile::from_str(dir_name).map_err(|error| {
                    peace_rt_model::Error::Native(
                        peace_rt_model::NativeError::ProfileDirInvalidName {
                            dir_name: dir_name.to_string(),
                            path: entry_path.to_path_buf(),
                            error,
                        },
                    )
                })?;

                if let Some(profiles_filter_fn) = profiles_filter_fn {
                    if !profiles_filter_fn(&profile) {
                        // Exclude any profiles that do not pass the filter
                        continue;
                    }
                }

                profiles.push(profile)
            }

            // Assume non-UTF8 file names are not profile directories
        }
    }

    // Ensure profiles are in a consistent, sensible order.
    profiles.sort();

    Ok(profiles)
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn profiles_from_peace_app_dir(
    _peace_app_dir: &peace_resources::paths::PeaceAppDir,
    _profiles_filter_fn: Option<&dyn Fn(&peace_core::Profile) -> bool>,
) -> Result<Vec<peace_core::Profile>, peace_rt_model::Error> {
    let profiles = Vec::new();

    // Not supported yet -- needs a `Storage` abstraction over both native an web
    // assembly.

    Ok(profiles)
}

/// Registers each item spec's `Params` and `State` for stateful
/// deserialization.
fn params_and_states_type_reg<E>(
    item_spec_graph: &ItemSpecGraph<E>,
) -> (ItemSpecParamsTypeReg, StatesTypeReg) {
    item_spec_graph.iter().fold(
        (ItemSpecParamsTypeReg::new(), StatesTypeReg::new()),
        |(mut item_spec_params_type_reg, mut states_type_reg), item_spec| {
            item_spec
                .params_and_state_register(&mut item_spec_params_type_reg, &mut states_type_reg);

            (item_spec_params_type_reg, states_type_reg)
        },
    )
}

/// Merges provided item spec parameters with previously stored item spec
/// parameters.
///
/// If an item spec's parameters are not provided, and nothing was previously
/// stored, then an error is returned.
fn item_spec_params_merge<E>(
    flow: &Flow<E>,
    mut item_spec_params_provided: ItemSpecParams,
    item_spec_params_stored: Option<ItemSpecParams>,
) -> Result<ItemSpecParams, E>
where
    E: From<Error>,
{
    // Combine provided and stored params. Provided params take precedence.
    //
    // We construct a new TypeMap because we want to make sure params are serialized
    // in order of the item specs in the graph.
    let item_spec_graph = flow.graph();
    let mut item_spec_params = ItemSpecParams::with_capacity(item_spec_graph.node_count());

    // Collected erroneous data -- parameters may have been valid in the past, but:
    //
    // * item spec IDs may have changed.
    // * item specs may have been removed, but params remain.
    // * item specs may have been added, but params forgotten to be added.
    let mut item_spec_ids_with_no_params = Vec::<ItemSpecId>::new();
    let mut stored_item_spec_params_mismatches = None;

    if let Some(mut item_spec_params_stored) = item_spec_params_stored {
        item_spec_graph.iter_insertion().for_each(|item_spec_rt| {
            let item_spec_id = item_spec_rt.id();

            // Removing the entry from stored params is deliberate, so filtering for stored
            // params that no longer have a corresponding item spec are
            // detected.
            let provided_params = item_spec_params_provided.remove_entry(item_spec_id);
            let stored_params = item_spec_params_stored.remove_entry(item_spec_id);
            if let Some((item_spec_id, params_boxed)) = provided_params.or(stored_params) {
                item_spec_params.insert_raw(item_spec_id, params_boxed);
            } else {
                // Collect item specs that do not have parameters.
                item_spec_ids_with_no_params.push(item_spec_id.clone());
            }
        });

        // Stored parameters whose IDs do not correspond to any item spec IDs in the
        // graph. May be empty.
        stored_item_spec_params_mismatches = Some(item_spec_params_stored);
    } else {
        item_spec_graph.iter_insertion().for_each(|item_spec_rt| {
            let item_spec_id = item_spec_rt.id();

            if let Some((item_spec_id, params_boxed)) =
                item_spec_params_provided.remove_entry(item_spec_id)
            {
                item_spec_params.insert_raw(item_spec_id, params_boxed);
            } else {
                // Collect item specs that do not have parameters.
                item_spec_ids_with_no_params.push(item_spec_id.clone());
            }
        });
    }

    // Provided parameters whose IDs do not correspond to any item spec IDs in the
    // graph.
    let provided_item_spec_params_mismatches = item_spec_params_provided;

    let params_all_match = item_spec_ids_with_no_params.is_empty()
        && provided_item_spec_params_mismatches.is_empty()
        && stored_item_spec_params_mismatches
            .as_ref()
            .map(|stored_item_spec_params_mismatches| stored_item_spec_params_mismatches.is_empty())
            .unwrap_or(true);

    if params_all_match {
        Ok(item_spec_params)
    } else {
        Err(Error::ItemSpecParamsMismatch {
            item_spec_ids_with_no_params,
            provided_item_spec_params_mismatches,
            stored_item_spec_params_mismatches,
        }
        .into())
    }
}

async fn item_spec_graph_setup<E>(
    item_spec_graph: &ItemSpecGraph<E>,
    resources: Resources<Empty>,
) -> Result<Resources<SetUp>, E>
where
    E: std::error::Error,
{
    let resources = item_spec_graph
        .stream()
        .map(Ok::<_, E>)
        .try_fold(resources, |mut resources, item_spec| async move {
            item_spec.setup(&mut resources).await?;
            Ok(resources)
        })
        .await?;

    Ok(Resources::<SetUp>::from(resources))
}
