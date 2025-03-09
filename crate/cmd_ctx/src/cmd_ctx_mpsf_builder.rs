use std::{collections::BTreeMap, fmt::Debug};

use futures::TryStreamExt;
use interruptible::Interruptibility;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_flow_rt::Flow;
use peace_item_model::ItemId;
use peace_params::ParamsSpecs;
use peace_profile_model::Profile;
use peace_resource_rt::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    paths::{FlowDir, ParamsSpecsFile, ProfileDir, ProfileHistoryDir, StatesCurrentFile},
    resources::ts::Empty,
    states::StatesCurrentStored,
    Resources,
};
use peace_rt_model::{
    params::{FlowParams, ProfileParams, WorkspaceParams},
    ParamsSpecsSerializer, Workspace, WorkspaceInitializer,
};
use peace_state_rt::StatesSerializer;
use type_reg::untagged::TypeReg;
use typed_builder::TypedBuilder;

use crate::{CmdCtxBuilderSupport, CmdCtxMpsf, CmdCtxTypes, ProfileFilterFn};

/// A command that works with multiple profiles, and a single flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a           # ✅ can list multiple `Profile`s
/// |   |- 📝 profile_params.yaml  # ✅ can read multiple `ProfileParams`
/// |   |
/// |   |- 🌊 deploy                # ✅ can read `FlowId`
/// |   |   |- 📝 flow_params.yaml  # ✅ can read or write `FlowParams`
/// |   |   |- 📋 states_goal.yaml  # ✅ can read or write `StatesGoal`
/// |   |   |- 📋 states_current.yaml # ✅ can read or write `StatesCurrentStored`
/// |   |
/// |   |- 🌊 ..                       # ❌ cannot read or write other `Flow` information
/// |
/// |- 🌏 customer_a_dev           # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |   |
/// |   |- 🌊 deploy                # ✅
/// |       |- 📝 flow_params.yaml  # ✅
/// |       |- 📋 states_goal.yaml  # ✅
/// |       |- 📋 states_current.yaml # ✅
/// |
/// |- 🌏 customer_a_prod          # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |   |
/// |   |- 🌊 deploy                # ✅
/// |       |- 📝 flow_params.yaml  # ✅
/// |       |- 📋 states_goal.yaml  # ✅
/// |       |- 📋 states_current.yaml # ✅
/// |
/// |
/// |- 🌏 workspace_init           # ✅ can list multiple `Profile`s
///     |- 📝 profile_params.yaml  # ❌ cannot read profile params of different underlying type
/// |   |- 🌊 workspace_init       # ❌ cannot read unrelated flows
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write multiple profiles' parameters &ndash; as long as they are of
///   the same type (same `struct`).
/// * Read or write flow parameters for the same flow.
/// * Read or write flow state for the same flow.
///
/// This kind of command cannot:
///
/// * Read or write flow parameters for different flows.
/// * Read or write flow state for different flows.
#[derive(Debug, TypedBuilder)]
#[builder(build_method(vis="", name=build_partial))]
pub struct CmdCtxMpsfBuilder<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,
    /// The interrupt channel receiver if this `CmdExecution` is interruptible.
    #[builder(default = Interruptibility::NonInterruptible)]
    pub interruptibility: Interruptibility<'static>,
    /// Workspace that the `peace` tool runs in.
    pub workspace: OwnedOrRef<'ctx, Workspace>,
    /// Function to filter the profiles that are accessible by this command.
    #[builder(default = None)]
    pub profile_filter_fn: Option<ProfileFilterFn>,
    /// The chosen process flow.
    pub flow: OwnedOrRef<'ctx, Flow<CmdCtxTypesT::AppError>>,
    /// Workspace params.
    pub workspace_params: WorkspaceParams<CmdCtxTypesT::WorkspaceParamsKey>,
    /// Profile params for each profile.
    pub profile_to_profile_params: BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>>,
    /// Flow params for each profile.
    pub profile_to_flow_params: BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>>,
    /// Item params specs for the selected flow for each profile.
    //
    // NOTE: When updating this mutator, also check if `CmdCtxSpsf` needs its mutator updated.
    #[builder(
        via_mutators(init = BTreeMap::new()),
        mutators(
            /// Sets an item's parameters.
            ///
            /// Note: this **must** be called for each item in the flow.
            pub fn with_item_params<I>(
                &mut self,
                profile: &Profile,
                item_id: ItemId,
                params_spec: <I::Params<'_> as peace_params::Params>::Spec,
            )
            where
                I: peace_cfg::Item,
                CmdCtxTypesT::AppError: From<I::Error>,
            {
                match self.profile_to_params_specs.get_mut(profile) {
                    Some(params_specs) => {
                        params_specs.insert(item_id, params_spec);
                    }
                    None => {
                        let mut params_specs = ParamsSpecs::new();
                        params_specs.insert(item_id, params_spec);
                        self.profile_to_params_specs.insert(profile.clone(), params_specs);
                    }
                }
            }
        )
    )]
    pub profile_to_params_specs: BTreeMap<Profile, ParamsSpecs>,
    /// `Resources` for flow execution.
    #[builder(default = Resources::<Empty>::new())]
    pub resources: Resources<Empty>,
}

// Use one of the following to obtain the generated type signature:
//
// ```sh
// cargo expand -p peace_cmd_ctx cmd_ctx_mpsf_builder
// ```
//
// Sublime text command:
//
// **LSP-rust-analyzer: Expand Macro Recursively** while the caret is on the
// `TypedBuilder` derive.
#[allow(non_camel_case_types)]
impl<
        'ctx,
        CmdCtxTypesT,
        __interruptibility: ::typed_builder::Optional<Interruptibility<'static>>,
        __profile_filter_fn: ::typed_builder::Optional<Option<ProfileFilterFn>>,
        __resources: ::typed_builder::Optional<Resources<Empty>>,
    >
    CmdCtxMpsfBuilderBuilder<
        'ctx,
        CmdCtxTypesT,
        (
            (OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,),
            __interruptibility,
            (OwnedOrRef<'ctx, Workspace>,),
            __profile_filter_fn,
            (OwnedOrRef<'ctx, Flow<CmdCtxTypesT::AppError>>,),
            (WorkspaceParams<CmdCtxTypesT::WorkspaceParamsKey>,),
            (BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>>,),
            (BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>>,),
            (BTreeMap<Profile, ParamsSpecs>,),
            __resources,
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    pub async fn build(self) -> Result<CmdCtxMpsf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError> {
        let CmdCtxMpsfBuilder {
            output,
            interruptibility,
            workspace,
            profile_filter_fn,
            flow,
            mut workspace_params,
            profile_to_profile_params: mut profile_to_profile_params_provided,
            profile_to_flow_params: mut profile_to_flow_params_provided,
            profile_to_params_specs: profile_to_params_specs_provided,
            resources: resources_override,
        } = self.build_partial();

        let workspace_params_type_reg = TypeReg::new();
        let profile_params_type_reg = TypeReg::new();
        let flow_params_type_reg = TypeReg::new();

        let workspace_dirs = workspace.dirs();
        let storage = workspace.storage();

        let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());
        CmdCtxBuilderSupport::workspace_params_merge(
            storage,
            &workspace_params_type_reg,
            &mut workspace_params,
            &workspace_params_file,
        )
        .await?;

        let profiles = CmdCtxBuilderSupport::profiles_from_peace_app_dir(
            workspace_dirs.peace_app_dir(),
            profile_filter_fn.as_ref(),
        )
        .await?;

        let profiles_ref = &profiles;
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

        let flow_dirs = profile_dirs.iter().fold(
            BTreeMap::<Profile, FlowDir>::new(),
            |mut flow_dirs, (profile, profile_dir)| {
                let flow_dir = FlowDir::from((profile_dir, flow.flow_id()));

                flow_dirs.insert(profile.clone(), flow_dir);

                flow_dirs
            },
        );

        let mut dirs_to_create = vec![
            AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),
        ];

        profile_dirs
            .values()
            .map(AsRef::<std::path::Path>::as_ref)
            .chain(
                profile_history_dirs
                    .values()
                    .map(AsRef::<std::path::Path>::as_ref),
            )
            .chain(flow_dirs.values().map(AsRef::<std::path::Path>::as_ref))
            .for_each(|dir| dirs_to_create.push(dir));

        let storage = workspace.storage();

        // profile_params_deserialize
        let profile_params_type_reg_ref = &profile_params_type_reg;
        let profile_to_profile_params = futures::stream::iter(
            profile_dirs
                .iter()
                .map(Result::<_, peace_rt_model::Error>::Ok),
        )
        .and_then(|(profile, profile_dir)| {
            let mut profile_params = profile_to_profile_params_provided
                .remove(profile)
                .unwrap_or_default();
            async move {
                let profile_params_file = ProfileParamsFile::from(profile_dir);

                CmdCtxBuilderSupport::profile_params_merge(
                    storage,
                    profile_params_type_reg_ref,
                    &mut profile_params,
                    &profile_params_file,
                )
                .await?;

                Ok((profile.clone(), profile_params))
            }
        })
        .try_collect::<BTreeMap<Profile, ProfileParams<CmdCtxTypesT::ProfileParamsKey>>>()
        .await?;

        // flow_params_deserialize
        let flow_params_type_reg_ref = &flow_params_type_reg;
        let profile_to_flow_params =
            futures::stream::iter(flow_dirs.iter().map(Result::<_, peace_rt_model::Error>::Ok))
                .and_then(|(profile, flow_dir)| {
                    let mut flow_params = profile_to_flow_params_provided
                        .remove(profile)
                        .unwrap_or_default();
                    async move {
                        let flow_params_file = FlowParamsFile::from(flow_dir);

                        CmdCtxBuilderSupport::flow_params_merge(
                            storage,
                            flow_params_type_reg_ref,
                            &mut flow_params,
                            &flow_params_file,
                        )
                        .await?;

                        Ok((profile.clone(), flow_params))
                    }
                })
                .try_collect::<BTreeMap<Profile, FlowParams<CmdCtxTypesT::FlowParamsKey>>>()
                .await?;

        // Create directories and write init parameters to storage.
        #[cfg(target_arch = "wasm32")]
        {
            WorkspaceInitializer::dirs_create(storage, dirs_to_create).await?;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            WorkspaceInitializer::dirs_create(dirs_to_create).await?;

            let workspace_dir = workspace_dirs.workspace_dir();
            std::env::set_current_dir(workspace_dir).map_err(|error| {
                peace_rt_model::Error::Native(peace_rt_model::NativeError::CurrentDirSet {
                    workspace_dir: workspace_dir.clone(),
                    error,
                })
            })?;
        }

        let interruptibility_state = interruptibility.into();

        // Serialize params to `PeaceAppDir`.
        CmdCtxBuilderSupport::workspace_params_serialize(
            &workspace_params,
            storage,
            &workspace_params_file,
        )
        .await?;

        // profile_params_serialize
        futures::stream::iter(
            profile_to_profile_params
                .iter()
                .map(Result::<_, peace_rt_model_core::Error>::Ok),
        )
        .try_for_each(|(profile, profile_params)| async {
            let profile_dir = profile_dirs.get(profile);
            // Should always exist, but don't panic if it doesn't.
            if let Some(profile_dir) = profile_dir {
                let profile_params_file = ProfileParamsFile::from(profile_dir);

                CmdCtxBuilderSupport::profile_params_serialize(
                    profile_params,
                    storage,
                    &profile_params_file,
                )
                .await?;
            }
            Ok(())
        })
        .await?;

        // flow_params_serialize
        futures::stream::iter(
            profile_to_flow_params
                .iter()
                .map(Result::<_, peace_rt_model_core::Error>::Ok),
        )
        .try_for_each(|(profile, flow_params)| async {
            let flow_dir = flow_dirs.get(profile);
            // Should always exist, but don't panic if it doesn't.
            if let Some(flow_dir) = flow_dir {
                let flow_params_file = FlowParamsFile::from(flow_dir);

                CmdCtxBuilderSupport::flow_params_serialize(
                    flow_params,
                    storage,
                    &flow_params_file,
                )
                .await?;
            }
            Ok(())
        })
        .await?;

        // Track items in memory.
        let mut resources = peace_resource_rt::Resources::new();

        CmdCtxBuilderSupport::workspace_params_insert(workspace_params.clone(), &mut resources);
        resources.insert(workspace_params_file);

        // `profile_params_insert` is not supported for multi-profile `CmdCtx`s.

        // `flow_params_insert` is not supported for multi-profile `CmdCtx`s.

        // Insert resources
        {
            let (app_name, workspace_dirs, storage) = (*workspace).clone().into_inner();
            let (workspace_dir, peace_dir, peace_app_dir) = workspace_dirs.into_inner();

            resources.insert(app_name);
            resources.insert(storage);
            resources.insert(workspace_dir);
            resources.insert(peace_dir);
            resources.insert(peace_app_dir);
            resources.insert(flow.flow_id().clone());
        }

        let flow_ref = &flow;
        let flow_id = flow_ref.flow_id();
        let item_graph = flow_ref.graph();

        let (params_specs_type_reg, states_type_reg) =
            CmdCtxBuilderSupport::params_and_states_type_reg(item_graph);

        let params_specs_type_reg_ref = &params_specs_type_reg;
        let app_name = workspace.app_name();
        let profile_to_params_specs =
            futures::stream::iter(flow_dirs.iter().map(Result::<_, peace_rt_model::Error>::Ok))
                .and_then(|(profile, flow_dir)| {
                    let params_specs_provided =
                        profile_to_params_specs_provided.get(profile).cloned();
                    async move {
                        let params_specs_file = ParamsSpecsFile::from(flow_dir);

                        let params_specs_stored =
                            ParamsSpecsSerializer::<peace_rt_model::Error>::deserialize_opt(
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
                                return Err(
                                    peace_rt_model_core::Error::ProfileParamsSpecsNotPresent {
                                        app_name: app_name.clone(),
                                        profile,
                                    },
                                );
                            }
                            (None, Some(params_specs_provided)) => params_specs_provided,
                            (Some(params_specs_stored), None) => params_specs_stored,
                            (Some(params_specs_stored), Some(params_specs_provided)) => {
                                CmdCtxBuilderSupport::params_specs_merge(
                                    flow_ref,
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

        let states_type_reg_ref = &states_type_reg;
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
                        .await?
                        .map(Into::<StatesCurrentStored>::into);

                    Ok((profile.clone(), states_current_stored))
                })
                .try_collect::<BTreeMap<Profile, Option<StatesCurrentStored>>>()
                .await?;

        // Call each `Item`'s initialization function.
        let mut resources = CmdCtxBuilderSupport::item_graph_setup(item_graph, resources).await?;

        // Needs to come before `state_example`, because params resolution may need
        // some resources to be inserted for `state_example` to work.
        resources.merge(resources_override.into_inner());

        let cmd_ctx_mpsf = CmdCtxMpsf {
            output,
            interruptibility_state,
            workspace,
            profiles,
            profile_dirs,
            profile_history_dirs,
            flow,
            flow_dirs,
            workspace_params_type_reg,
            workspace_params,
            profile_params_type_reg,
            profile_to_profile_params,
            flow_params_type_reg,
            profile_to_flow_params,
            profile_to_states_current_stored,
            params_specs_type_reg,
            profile_to_params_specs,
            states_type_reg,
            resources,
        };

        Ok(cmd_ctx_mpsf)
    }
}
