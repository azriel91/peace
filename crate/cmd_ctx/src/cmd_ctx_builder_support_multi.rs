use futures::stream::TryStreamExt;
use own::OwnedOrRef;
use peace_flow_model::FlowId;
use peace_flow_rt::Flow;
use peace_params::ParamsSpecs;
use peace_profile_model::Profile;
use peace_resource_rt::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceDirs},
    paths::{
        FlowDir, ParamsSpecsFile, PeaceAppDir, ProfileDir, ProfileHistoryDir, StatesCurrentFile,
    },
    states::{ts::CurrentStored, States, StatesCurrentStored},
};
use peace_rt_model::{
    params::{FlowParamsOpt, ProfileParams, ProfileParamsOpt},
    ParamsSpecsSerializer, StatesTypeReg,
};
use peace_rt_model_core::params::FlowParams;
use peace_state_rt::StatesSerializer;
use std::{collections::BTreeMap, marker::PhantomData};
use type_reg::untagged::TypeReg;

use crate::{CmdCtxBuilderSupport, CmdCtxTypes, ProfileFilterFn};

/// Common code used to build different `CmdCtxM*` types.
pub(crate) struct CmdCtxBuilderSupportMulti<CmdCtxTypesT>(PhantomData<CmdCtxTypesT>);

impl<CmdCtxTypesT> CmdCtxBuilderSupportMulti<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Reads the the `Profile` and `ProfileHistory` directory paths into
    /// memory.
    pub(crate) fn profile_and_history_dirs_read(
        profiles_ref: &[Profile],
        workspace_dirs: &WorkspaceDirs,
    ) -> (
        BTreeMap<Profile, ProfileDir>,
        BTreeMap<Profile, ProfileHistoryDir>,
    )
    where
        CmdCtxTypesT: CmdCtxTypes,
    {
        let (profile_dirs, profile_history_dirs) = profiles_ref.iter().fold(
            (
                BTreeMap::<Profile, ProfileDir>::new(),
                BTreeMap::<Profile, ProfileHistoryDir>::new(),
            ),
            |(mut profile_dirs, mut profile_history_dirs), profile| {
                let profile_dir = ProfileDir::from((workspace_dirs.peace_app_dir(), profile));
                let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

                profile_dirs.insert(profile.clone(), profile_dir);
                profile_history_dirs.insert(profile.clone(), profile_history_dir);

                (profile_dirs, profile_history_dirs)
            },
        );
        (profile_dirs, profile_history_dirs)
    }

    /// Reads the the `Flow`directory paths into memory.
    pub(crate) fn flow_dirs_read(
        profile_dirs: &BTreeMap<Profile, ProfileDir>,
        flow: &OwnedOrRef<'_, Flow<CmdCtxTypesT::AppError>>,
    ) -> BTreeMap<Profile, FlowDir> {
        let flow_dirs = profile_dirs.iter().fold(
            BTreeMap::<Profile, FlowDir>::new(),
            |mut flow_dirs, (profile, profile_dir)| {
                let flow_dir = FlowDir::from((profile_dir, flow.flow_id()));

                flow_dirs.insert(profile.clone(), flow_dir);

                flow_dirs
            },
        );
        flow_dirs
    }

    pub(crate) async fn profile_params_deserialize(
        profile_dirs: &BTreeMap<Profile, ProfileDir>,
        mut profile_to_profile_params_provided: BTreeMap<
            Profile,
            ProfileParamsOpt<CmdCtxTypesT::ProfileParamsKey>,
        >,
        storage: &peace_rt_model::Storage,
        profile_params_type_reg_ref: &TypeReg<CmdCtxTypesT::ProfileParamsKey>,
    ) -> Result<
        BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>>,
        CmdCtxTypesT::AppError,
    > {
        let profile_to_profile_params = futures::stream::iter(
            profile_dirs
                .iter()
                .map(Result::<_, peace_rt_model_core::Error>::Ok),
        )
        .and_then(|(profile, profile_dir)| {
            let profile_params_provided = profile_to_profile_params_provided
                .remove(profile)
                .unwrap_or_default();
            async move {
                let profile_params_file = ProfileParamsFile::from(profile_dir);

                let profile_params = CmdCtxBuilderSupport::profile_params_merge(
                    storage,
                    profile_params_type_reg_ref,
                    profile_params_provided,
                    &profile_params_file,
                )
                .await?;

                Ok((profile.clone(), profile_params))
            }
        })
        .try_collect::<BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>>>()
        .await?;
        Ok(profile_to_profile_params)
    }

    pub(crate) async fn flow_params_deserialize(
        flow_dirs: &BTreeMap<Profile, FlowDir>,
        mut profile_to_flow_params_provided: BTreeMap<
            Profile,
            FlowParamsOpt<CmdCtxTypesT::FlowParamsKey>,
        >,
        storage: &peace_rt_model::Storage,
        flow_params_type_reg_ref: &TypeReg<CmdCtxTypesT::FlowParamsKey>,
    ) -> Result<BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>>, CmdCtxTypesT::AppError>
    {
        let profile_to_flow_params = futures::stream::iter(
            flow_dirs
                .iter()
                .map(Result::<_, peace_rt_model_core::Error>::Ok),
        )
        .and_then(|(profile, flow_dir)| {
            let flow_params_provided = profile_to_flow_params_provided
                .remove(profile)
                .unwrap_or_default();
            async move {
                let flow_params_file = FlowParamsFile::from(flow_dir);

                let flow_params = CmdCtxBuilderSupport::flow_params_merge(
                    storage,
                    flow_params_type_reg_ref,
                    flow_params_provided,
                    &flow_params_file,
                )
                .await?;

                Ok((profile.clone(), flow_params))
            }
        })
        .try_collect::<BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>>>()
        .await?;
        Ok(profile_to_flow_params)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn profiles_from_peace_app_dir(
        peace_app_dir: &PeaceAppDir,
        profile_filter_fn: Option<&ProfileFilterFn>,
    ) -> Result<Vec<Profile>, peace_rt_model_core::Error> {
        use std::{ffi::OsStr, str::FromStr};

        let mut profiles = Vec::new();
        let mut peace_app_read_dir = tokio::fs::read_dir(peace_app_dir).await.map_err(
            #[cfg_attr(coverage_nightly, coverage(off))]
            |error| {
                peace_rt_model_core::Error::Native(peace_rt_model::NativeError::PeaceAppDirRead {
                    peace_app_dir: peace_app_dir.to_path_buf(),
                    error,
                })
            },
        )?;
        while let Some(entry) = peace_app_read_dir.next_entry().await.map_err(
            #[cfg_attr(coverage_nightly, coverage(off))]
            |error| {
                peace_rt_model_core::Error::Native(
                    peace_rt_model::NativeError::PeaceAppDirEntryRead {
                        peace_app_dir: peace_app_dir.to_path_buf(),
                        error,
                    },
                )
            },
        )? {
            let file_type = entry.file_type().await.map_err(
                #[cfg_attr(coverage_nightly, coverage(off))]
                |error| {
                    peace_rt_model_core::Error::Native(
                        peace_rt_model::NativeError::PeaceAppDirEntryFileTypeRead {
                            path: entry.path(),
                            error,
                        },
                    )
                },
            )?;

            if file_type.is_dir() {
                let entry_path = entry.path();
                if let Some(dir_name) = entry_path.file_name().and_then(OsStr::to_str) {
                    // Assume this is a profile directory
                    let profile =
                        peace_profile_model::Profile::from_str(dir_name).map_err(|error| {
                            peace_rt_model_core::Error::Native(
                                peace_rt_model::NativeError::ProfileDirInvalidName {
                                    dir_name: dir_name.to_string(),
                                    path: entry_path.to_path_buf(),
                                    error,
                                },
                            )
                        })?;

                    if let Some(profile_filter_fn) = profile_filter_fn {
                        if !profile_filter_fn.call(&profile) {
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
        _peace_app_dir: &PeaceAppDir,
        _profile_filter_fn: Option<&ProfileFilterFn>,
    ) -> Result<Vec<Profile>, peace_rt_model_core::Error> {
        let profiles = Vec::new();

        // TODO: Not supported yet -- needs a `Storage` abstraction over both native an
        // web assembly.

        Ok(profiles)
    }

    /// Serializes profile params to storage.
    pub(crate) async fn profile_params_serialize(
        profile_to_profile_params: &BTreeMap<
            Profile,
            ProfileParams<CmdCtxTypesT::ProfileParamsKey>,
        >,
        profile_dirs: &BTreeMap<Profile, ProfileDir>,
        storage: &peace_rt_model::Storage,
    ) -> Result<(), CmdCtxTypesT::AppError> {
        futures::stream::iter(
            profile_to_profile_params
                .iter()
                .map(Result::<_, peace_rt_model_core::Error>::Ok),
        )
        .try_for_each(|(profile, profile_params)| {
            let profile_dir = profile_dirs.get(profile).unwrap_or_else(
                #[cfg_attr(coverage_nightly, coverage(off))]
                || {
                    panic!(
                        "`profile_dir` for `{profile}` should exist as it is inserted by \
                        `CmdCtxBuilderSupportMulti::profile_and_history_dirs_read`."
                    )
                },
            );
            let profile_params_file = ProfileParamsFile::from(profile_dir);
            async move {
                CmdCtxBuilderSupport::profile_params_serialize(
                    profile_params,
                    storage,
                    &profile_params_file,
                )
                .await?;
                Ok(())
            }
        })
        .await?;
        Ok(())
    }

    /// Serializes flow params to storage.
    pub(crate) async fn flow_params_serialize(
        profile_to_flow_params: &BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>>,
        flow_dirs: &BTreeMap<Profile, peace_resource_rt::paths::FlowDir>,
        storage: &peace_rt_model::Storage,
    ) -> Result<(), CmdCtxTypesT::AppError> {
        futures::stream::iter(
            profile_to_flow_params
                .iter()
                .map(Result::<_, peace_rt_model_core::Error>::Ok),
        )
        .try_for_each(|(profile, flow_params)| {
            let flow_dir = flow_dirs.get(profile).unwrap_or_else(
                #[cfg_attr(coverage_nightly, coverage(off))]
                || {
                    panic!(
                        "`flow_dir` for `{profile}` should exist as it is inserted by \
                        `CmdCtxBuilderSupportMulti::flow_dirs_read`."
                    )
                },
            );
            let flow_params_file = FlowParamsFile::from(flow_dir);

            async move {
                CmdCtxBuilderSupport::flow_params_serialize(
                    flow_params,
                    storage,
                    &flow_params_file,
                )
                .await?;
                Ok(())
            }
        })
        .await?;
        Ok(())
    }

    /// Reads `StateCurrent` for each profile.
    pub(crate) async fn states_current_read(
        flow_dirs: &BTreeMap<Profile, FlowDir>,
        flow_id: &FlowId,
        storage: &peace_rt_model::Storage,
        states_type_reg_ref: &StatesTypeReg,
    ) -> Result<BTreeMap<Profile, Option<States<CurrentStored>>>, CmdCtxTypesT::AppError> {
        let profile_to_states_current_stored =
            futures::stream::iter(flow_dirs.iter().map(Result::<_, peace_rt_model::Error>::Ok))
                .and_then(|(profile, flow_dir)| async move {
                    let states_current_file = StatesCurrentFile::from(flow_dir);

                    let states_current_stored =
                        StatesSerializer::<peace_rt_model::Error>::deserialize_stored_opt(
                            flow_id,
                            storage,
                            states_type_reg_ref,
                            &states_current_file,
                        )
                        .await?;

                    Ok((profile.clone(), states_current_stored))
                })
                .try_collect::<BTreeMap<Profile, Option<StatesCurrentStored>>>()
                .await?;
        Ok(profile_to_states_current_stored)
    }

    /// Deserializes previously stored params specs, merges them with the
    /// provided params specs, and stores the merged values.
    pub(crate) async fn params_specs_load_merge_and_store(
        flow_dirs: &BTreeMap<Profile, FlowDir>,
        profile_to_params_specs_provided: BTreeMap<Profile, ParamsSpecs>,
        flow: &Flow<CmdCtxTypesT::AppError>,
        storage: &peace_rt_model::Storage,
        params_specs_type_reg_ref: &peace_rt_model::ParamsSpecsTypeReg,
        app_name: &peace_cfg::AppName,
    ) -> Result<BTreeMap<Profile, ParamsSpecs>, CmdCtxTypesT::AppError> {
        let flow_id = flow.flow_id();
        let profile_to_params_specs = futures::stream::iter(
            flow_dirs
                .iter()
                .map(Result::<_, peace_rt_model_core::Error>::Ok),
        )
        .and_then(|(profile, flow_dir)| {
            let params_specs_provided = profile_to_params_specs_provided.get(profile).cloned();
            async move {
                let params_specs_file = ParamsSpecsFile::from(flow_dir);

                let params_specs_stored =
                    ParamsSpecsSerializer::<peace_rt_model_core::Error>::deserialize_opt(
                        profile,
                        flow_id,
                        storage,
                        params_specs_type_reg_ref,
                        &params_specs_file,
                    )
                    .await?;

                // For mapping fns, we still need the developer to provide the params spec
                // so that multi-profile diffs can be done.
                let profile = profile.clone();
                let params_specs = match (params_specs_stored, params_specs_provided) {
                    (None, None) => {
                        return Err(peace_rt_model_core::Error::ItemParamsSpecsFileNotFound {
                            app_name: app_name.clone(),
                            profile,
                            flow_id: flow_id.clone(),
                        });
                    }
                    (None, Some(params_specs_provided)) => params_specs_provided,
                    (Some(params_specs_stored), None) => params_specs_stored,
                    (Some(params_specs_stored), Some(params_specs_provided)) => {
                        CmdCtxBuilderSupport::params_specs_merge(
                            flow,
                            params_specs_provided,
                            Some(params_specs_stored),
                        )?
                    }
                };

                // Serialize params specs back to disk.
                CmdCtxBuilderSupport::params_specs_serialize(
                    &params_specs,
                    storage,
                    &params_specs_file,
                )
                .await?;

                Ok((profile, params_specs))
            }
        })
        .try_collect::<BTreeMap<Profile, ParamsSpecs>>()
        .await?;
        Ok(profile_to_params_specs)
    }
}
