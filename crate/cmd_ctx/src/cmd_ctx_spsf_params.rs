use std::future::IntoFuture;

use futures::{future::LocalBoxFuture, FutureExt};
use interruptible::Interruptibility;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_flow_rt::Flow;
use peace_item_model::ItemId;
use peace_params::{ParamsSpecs, ParamsValue};
use peace_resource_rt::{
    internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
    paths::{FlowDir, ParamsSpecsFile, ProfileDir, ProfileHistoryDir, StatesCurrentFile},
    resources::ts::Empty,
    states::StatesCurrentStored,
    Resources,
};
use peace_rt_model::{ParamsSpecsSerializer, Workspace, WorkspaceInitializer};
use peace_rt_model_core::params::{FlowParams, ProfileParams, WorkspaceParams};
use peace_state_rt::StatesSerializer;
use type_reg::untagged::TypeReg;
use typed_builder::TypedBuilder;

use crate::{CmdCtxBuilderSupport, CmdCtxSpsf, CmdCtxSpsfFields, CmdCtxTypes, ProfileSelection};

/// Context for a command that works with one profile and one flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a
/// |   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
/// |   |
/// |   |- 🌊 deploy                  # ✅ can read `FlowId`
/// |   |   |- 📝 flow_params.yaml    # ✅ can read or write `FlowParams`
/// |   |   |- 📋 states_goal.yaml    # ✅ can read or write `StatesGoal`
/// |   |   |- 📋 states_current.yaml # ✅ can read or write `StatesCurrentStored`
/// |   |
/// |   |- 🌊 ..                   # ❌ cannot read or write other `Flow` information
/// |
/// |- 🌏 ..                       # ❌ cannot read or write other `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write a single profile's parameters. For multiple profiles, see
///   [`CmdCtxMpsf`].
///
/// [`CmdCtxMpsf`]: crate::CmdCtxMpsf
#[derive(Debug, TypedBuilder)]
#[builder(build_method(vis="", name=build_partial))]
pub struct CmdCtxSpsfParams<'ctx, CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    #[builder(setter(prefix = "with_"))]
    pub output: OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,
    /// The interrupt channel receiver if this `CmdExecution` is interruptible.
    #[builder(setter(prefix = "with_"), default = Interruptibility::NonInterruptible)]
    pub interruptibility: Interruptibility<'static>,
    /// Workspace that the `peace` tool runs in.
    #[builder(setter(prefix = "with_"))]
    pub workspace: OwnedOrRef<'ctx, Workspace>,
    /// The profile this command operates on.
    #[builder(setter(prefix = "with_"))]
    pub profile_selection: ProfileSelection<'ctx, CmdCtxTypesT::WorkspaceParamsKey>,
    /// The chosen process flow.
    #[builder(setter(prefix = "with_"))]
    pub flow: OwnedOrRef<'ctx, Flow<CmdCtxTypesT::AppError>>,
    /// Workspace params.
    //
    // NOTE: When updating this mutator, also update it for all the other `CmdCtx*Params` types.
    #[builder(
        setter(prefix = "with_"),
        via_mutators(init = WorkspaceParams::default()),
        mutators(
            /// Sets the value at the given workspace params key.
            ///
            /// # Parameters
            ///
            /// * `key`: The key to store the given value against.
            /// * `value`: The value to store at the given key. This is an
            ///   `Option` so that you may remove a value if desired.
            ///
            /// # Type Parameters
            ///
            /// * `V`: The serializable type stored at the given key.
            pub fn with_workspace_param<V>(
                &mut self,
                key: CmdCtxTypesT::WorkspaceParamsKey,
                value: Option<V>,
            )
            where
                V: ParamsValue,
            {
                self.workspace_params.insert(key, value);
            }
        )
    )]
    pub workspace_params: WorkspaceParams<<CmdCtxTypesT as CmdCtxTypes>::WorkspaceParamsKey>,
    /// Profile params for the profile.
    #[builder(
        setter(prefix = "with_"),
        via_mutators(init = ProfileParams::default()),
        mutators(
            /// Sets the value at the given profile params key.
            ///
            /// # Parameters
            ///
            /// * `key`: The key to store the given value against.
            /// * `value`: The value to store at the given key. This is an
            ///   `Option` so that you may remove a value if desired.
            ///
            /// # Type Parameters
            ///
            /// * `V`: The serializable type stored at the given key.
            pub fn with_profile_param<V>(
                &mut self,
                key: CmdCtxTypesT::ProfileParamsKey,
                value: Option<V>,
            )
            where
                V: ParamsValue,
            {
                self.profile_params.insert(key, value);
            }
        )
    )]
    pub profile_params: ProfileParams<<CmdCtxTypesT as CmdCtxTypes>::ProfileParamsKey>,
    /// Flow params for the selected flow.
    #[builder(
        setter(prefix = "with_"),
        via_mutators(init = FlowParams::default()),
        mutators(
            /// Sets the value at the given flow params key.
            ///
            /// # Parameters
            ///
            /// * `key`: The key to store the given value against.
            /// * `value`: The value to store at the given key. This is an
            ///   `Option` so that you may remove a value if desired.
            ///
            /// # Type Parameters
            ///
            /// * `V`: The serializable type stored at the given key.
            pub fn with_flow_param<V>(
                &mut self,
                key: CmdCtxTypesT::FlowParamsKey,
                value: Option<V>,
            )
            where
                V: ParamsValue,
            {
                self.flow_params.insert(key, value);
            }
        )
    )]
    pub flow_params: FlowParams<<CmdCtxTypesT as CmdCtxTypes>::FlowParamsKey>,
    /// Item params specs for the selected flow.
    //
    // NOTE: When updating this mutator, also check if `CmdCtxMpsf` needs its mutator updated.
    #[builder(
        via_mutators(init = ParamsSpecs::new()),
        mutators(
            /// Sets an item's parameters.
            ///
            /// Note: this **must** be called for each item in the flow.
            pub fn with_item_params<I>(
                &mut self,
                item_id: ItemId,
                params_spec: <I::Params<'_> as peace_params::Params>::Spec,
            )
            where
                I: peace_cfg::Item,
                CmdCtxTypesT::AppError: From<I::Error>,
            {
                self.params_specs.insert(item_id, params_spec);
            }
        )
    )]
    pub params_specs: ParamsSpecs,
    /// `Resources` for flow execution.
    #[builder(setter(prefix = "with_"), default = Resources::<Empty>::new())]
    pub resources: Resources<Empty>,
}

// Use one of the following to obtain the generated type signature:
//
// ```sh
// cargo expand -p peace_cmd_ctx cmd_ctx_spsf_builder
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
        __resources: ::typed_builder::Optional<Resources<Empty>>,
    >
    CmdCtxSpsfParamsBuilder<
        'ctx,
        CmdCtxTypesT,
        (
            (OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,),
            __interruptibility,
            (OwnedOrRef<'ctx, Workspace>,),
            (ProfileSelection<'ctx, CmdCtxTypesT::WorkspaceParamsKey>,),
            (OwnedOrRef<'ctx, Flow<CmdCtxTypesT::AppError>>,),
            (WorkspaceParams<<CmdCtxTypesT as CmdCtxTypes>::WorkspaceParamsKey>,),
            (ProfileParams<<CmdCtxTypesT as CmdCtxTypes>::ProfileParamsKey>,),
            (FlowParams<<CmdCtxTypesT as CmdCtxTypes>::FlowParamsKey>,),
            (ParamsSpecs,),
            __resources,
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    pub async fn build(self) -> Result<CmdCtxSpsf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError> {
        let CmdCtxSpsfParams {
            output,
            interruptibility,
            workspace,
            profile_selection,
            flow,
            mut workspace_params,
            mut profile_params,
            mut flow_params,
            params_specs: params_specs_provided,
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

        let profile = match profile_selection {
            ProfileSelection::Specified(profile) => profile,
            ProfileSelection::FromWorkspaceParam(workspace_params_k_profile) => workspace_params
                .get(&workspace_params_k_profile)
                .cloned()
                .ok_or(peace_rt_model_core::Error::WorkspaceParamsProfileNone)?,
        };

        let profile_ref = &profile;
        let profile_dir = ProfileDir::from((workspace_dirs.peace_app_dir(), profile_ref));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, flow.flow_id()));

        let dirs_to_create = [
            AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
            AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),
            AsRef::<std::path::Path>::as_ref(&profile_dir),
            AsRef::<std::path::Path>::as_ref(&profile_history_dir),
            AsRef::<std::path::Path>::as_ref(&flow_dir),
        ];

        let storage = workspace.storage();

        // profile_params_deserialize
        let profile_params_file = ProfileParamsFile::from(&profile_dir);
        CmdCtxBuilderSupport::profile_params_merge(
            storage,
            &profile_params_type_reg,
            &mut profile_params,
            &profile_params_file,
        )
        .await?;

        // flow_params_deserialize
        let flow_params_file = FlowParamsFile::from(&flow_dir);
        CmdCtxBuilderSupport::flow_params_merge(
            storage,
            &flow_params_type_reg,
            &mut flow_params,
            &flow_params_file,
        )
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

        CmdCtxBuilderSupport::profile_params_serialize(
            &profile_params,
            storage,
            &profile_params_file,
        )
        .await?;

        CmdCtxBuilderSupport::flow_params_serialize(&flow_params, storage, &flow_params_file)
            .await?;

        // Track items in memory.
        let mut resources = peace_resource_rt::Resources::new();

        CmdCtxBuilderSupport::workspace_params_insert(workspace_params.clone(), &mut resources);
        resources.insert(workspace_params_file);

        CmdCtxBuilderSupport::profile_params_insert(profile_params.clone(), &mut resources);
        resources.insert(profile_params_file);

        CmdCtxBuilderSupport::flow_params_insert(flow_params.clone(), &mut resources);
        resources.insert(flow_params_file);

        // Insert resources
        {
            let (app_name, workspace_dirs, storage) = (*workspace).clone().into_inner();
            let (workspace_dir, peace_dir, peace_app_dir) = workspace_dirs.into_inner();

            resources.insert(app_name);
            resources.insert(storage);
            resources.insert(workspace_dir);
            resources.insert(peace_dir);
            resources.insert(peace_app_dir);
            resources.insert(profile_dir.clone());
            resources.insert(profile_history_dir.clone());
            resources.insert(profile.clone());
            resources.insert(flow_dir.clone());
            resources.insert(flow.flow_id().clone());
        }

        // Set up resources for the flow's item graph
        let flow_ref = &flow;
        let flow_id = flow_ref.flow_id();
        let item_graph = flow_ref.graph();

        let (params_specs_type_reg, states_type_reg) =
            CmdCtxBuilderSupport::params_and_states_type_reg(item_graph);

        // Params specs loading and storage.
        let params_specs_type_reg_ref = &params_specs_type_reg;
        let params_specs_file = ParamsSpecsFile::from(&flow_dir);
        let params_specs_stored = ParamsSpecsSerializer::<peace_rt_model::Error>::deserialize_opt(
            &profile,
            flow_id,
            storage,
            params_specs_type_reg_ref,
            &params_specs_file,
        )
        .await?;

        let params_specs = CmdCtxBuilderSupport::params_specs_merge(
            flow_ref,
            params_specs_provided,
            params_specs_stored,
        )?;

        CmdCtxBuilderSupport::params_specs_serialize(&params_specs, storage, &params_specs_file)
            .await?;

        // States loading and storage.
        let states_type_reg_ref = &states_type_reg;
        let states_current_file = StatesCurrentFile::from(&flow_dir);
        let states_current_stored =
            StatesSerializer::<peace_rt_model::Error>::deserialize_stored_opt(
                flow_id,
                storage,
                states_type_reg_ref,
                &states_current_file,
            )
            .await?
            .map(Into::<StatesCurrentStored>::into);
        if let Some(states_current_stored) = states_current_stored {
            resources.insert(states_current_stored);
        }

        // Call each `Item`'s initialization function.
        let mut resources = CmdCtxBuilderSupport::item_graph_setup(item_graph, resources).await?;

        // output_progress CmdProgressTracker initialization
        #[cfg(feature = "output_progress")]
        let cmd_progress_tracker = {
            let multi_progress =
                indicatif::MultiProgress::with_draw_target(indicatif::ProgressDrawTarget::hidden());
            let progress_trackers = item_graph.iter_insertion().fold(
                peace_rt_model::IndexMap::with_capacity(item_graph.node_count()),
                |mut progress_trackers, item| {
                    let progress_bar = multi_progress.add(indicatif::ProgressBar::hidden());
                    let progress_tracker = peace_progress_model::ProgressTracker::new(progress_bar);
                    progress_trackers.insert(item.id().clone(), progress_tracker);
                    progress_trackers
                },
            );

            peace_rt_model::CmdProgressTracker::new(multi_progress, progress_trackers)
        };

        // Needs to come before `state_example`, because params resolution may need
        // some resources to be inserted for `state_example` to work.
        resources.merge(resources_override.into_inner());

        // Fetching state example inserts it into resources.
        #[cfg(feature = "item_state_example")]
        {
            let () = flow.graph().iter().try_for_each(|item| {
                let _state_example = item.state_example(&params_specs, &resources)?;
                Ok::<_, CmdCtxTypesT::AppError>(())
            })?;
        }

        let cmd_ctx_spsf = CmdCtxSpsf {
            output,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            fields: CmdCtxSpsfFields {
                interruptibility_state,
                workspace,
                profile,
                profile_dir,
                profile_history_dir,
                flow,
                flow_dir,
                workspace_params_type_reg,
                workspace_params,
                profile_params_type_reg,
                profile_params,
                flow_params_type_reg,
                flow_params,
                params_specs_type_reg,
                params_specs,
                states_type_reg,
                resources,
            },
        };

        Ok(cmd_ctx_spsf)
    }
}

#[allow(non_camel_case_types)]
impl<
        'ctx,
        CmdCtxTypesT,
        __interruptibility: ::typed_builder::Optional<Interruptibility<'static>> + 'ctx,
        __resources: ::typed_builder::Optional<Resources<Empty>> + 'ctx,
    > IntoFuture
    for CmdCtxSpsfParamsBuilder<
        'ctx,
        CmdCtxTypesT,
        (
            (OwnedOrMutRef<'ctx, CmdCtxTypesT::Output>,),
            __interruptibility,
            (OwnedOrRef<'ctx, Workspace>,),
            (ProfileSelection<'ctx, CmdCtxTypesT::WorkspaceParamsKey>,),
            (OwnedOrRef<'ctx, Flow<CmdCtxTypesT::AppError>>,),
            (WorkspaceParams<<CmdCtxTypesT as CmdCtxTypes>::WorkspaceParamsKey>,),
            (ProfileParams<<CmdCtxTypesT as CmdCtxTypes>::ProfileParamsKey>,),
            (FlowParams<<CmdCtxTypesT as CmdCtxTypes>::FlowParamsKey>,),
            (ParamsSpecs,),
            __resources,
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Future that returns the `CmdCtxSpsf`.
    ///
    /// This is boxed since [TAIT] is not yet available ([rust#63063]).
    ///
    /// [TAIT]: https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html
    /// [rust#63063]: https://github.com/rust-lang/rust/issues/63063
    type IntoFuture =
        LocalBoxFuture<'ctx, Result<CmdCtxSpsf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError>>;
    type Output = <Self::IntoFuture as std::future::Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        self.build().boxed_local()
    }
}
