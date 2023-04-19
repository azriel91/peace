#![allow(clippy::type_complexity)]

use std::{fmt::Debug, hash::Hash};

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    paths::ItemSpecParamsFile,
    resources::ts::{Empty, SetUp},
    Resources,
};
use peace_rt_model::{
    fn_graph::resman::Resource,
    params::{FlowParams, ProfileParams, WorkspaceParams},
    Error, ItemSpecGraph, ItemSpecParams, ItemSpecParamsTypeReg, StatesTypeReg, Storage, Workspace,
    WorkspaceInitializer,
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
