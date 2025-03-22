use std::{collections::BTreeMap, fmt::Debug, future::IntoFuture};

use futures::{future::LocalBoxFuture, FutureExt};
use interruptible::Interruptibility;
use own::{OwnedOrMutRef, OwnedOrRef};
use peace_flow_rt::Flow;
use peace_item_model::ItemId;
use peace_params::{ParamsSpecs, ParamsValue};
use peace_profile_model::Profile;
use peace_resource_rt::{internal::WorkspaceParamsFile, resources::ts::Empty, Resources};
use peace_rt_model::{
    params::{FlowParams, ProfileParams, WorkspaceParams},
    Workspace, WorkspaceInitializer,
};
use type_reg::untagged::TypeReg;
use typed_builder::TypedBuilder;

use crate::{
    CmdCtxBuilderSupport, CmdCtxBuilderSupportMulti, CmdCtxMpsf, CmdCtxMpsfFields, CmdCtxTypes,
    ProfileFilterFn,
};

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
pub struct CmdCtxMpsfParams<'ctx, CmdCtxTypesT>
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
    /// Function to filter the profiles that are accessible by this command.
    #[builder(setter(
        prefix = "with_",
        transform = |f: impl Fn(&Profile) -> bool + 'static| Some(ProfileFilterFn(Box::new(f)))),
        default = None
    )]
    pub profile_filter_fn: Option<ProfileFilterFn>,
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
    #[builder(setter(prefix = "with_"))]
    pub workspace_params: WorkspaceParams<<CmdCtxTypesT as CmdCtxTypes>::WorkspaceParamsKey>,
    /// Profile params for each profile.
    //
    // NOTE: When updating this mutator, also update it for all the other `CmdCtx*Params` types.
    #[builder(
        setter(prefix = "with_"),
        via_mutators(init = BTreeMap::new()),
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
            pub fn with_profile_param<V>(
                &mut self,
                profile: &Profile,
                key: CmdCtxTypesT::ProfileParamsKey,
                value: Option<V>,
            )
            where
                V: ParamsValue,
            {
                match self.profile_to_profile_params.get_mut(profile) {
                    Some(profile_params) => {
                        profile_params.insert(key, value);
                    }
                    None => {
                        let mut profile_params = ProfileParams::new();
                        profile_params.insert(key, value);
                        self.profile_to_profile_params.insert(profile.clone(), profile_params);
                    }
                }
            }
        )
    )]
    pub profile_to_profile_params:
        BTreeMap<Profile, ProfileParams<<CmdCtxTypesT as CmdCtxTypes>::ProfileParamsKey>>,
    /// Flow params for each profile.
    #[builder(
        setter(prefix = "with_"),
        via_mutators(init = BTreeMap::new()),
        mutators(
            /// Sets a parameter for a given profile.
            ///
            /// # Parameters
            ///
            /// * `profile`: The profile whose parameters to modify.
            /// * `key`: The key to store the given value against.
            /// * `value`: The value to store at the given key. This is an
            ///   `Option` so that you may remove a value if desired.
            ///
            /// # Type Parameters
            ///
            /// * `V`: The serializable type stored at the given key.
            pub fn with_flow_param<V>(
                &mut self,
                profile: &Profile,
                key: CmdCtxTypesT::FlowParamsKey,
                value: Option<V>,
            )
            where
                V: ParamsValue,
            {
                match self.profile_to_flow_params.get_mut(profile) {
                    Some(flow_params) => {
                        flow_params.insert(key, value);
                    }
                    None => {
                        let mut flow_params = FlowParams::new();
                        flow_params.insert(key, value);
                        self.profile_to_flow_params.insert(profile.clone(), flow_params);
                    }
                }
            }
        )
    )]
    pub profile_to_flow_params:
        BTreeMap<Profile, FlowParams<<CmdCtxTypesT as CmdCtxTypes>::FlowParamsKey>>,
    /// Item params specs for the selected flow for each profile.
    //
    // NOTE: When updating this mutator, also check if `CmdCtxSpsf` needs its mutator updated.
    #[builder(
        via_mutators(init = BTreeMap::new()),
        mutators(
            /// Sets an item's parameters at a given profile.
            ///
            /// # Parameters
            ///
            /// * `profile`: The profile whose item parameters to set.
            /// * `item_id`: The ID of the item whose parameters to set.
            /// * `params_spec`: The specification of how to resolve the
            ///   parameters.
            ///
            /// # Type Parameters
            ///
            /// * `I`: The `Item` type.
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
    ///
    /// A "resource" is any object, and `resources` is a map where each object
    /// is keyed by its type. This means only one instance of each type can be
    /// held by the map.
    ///
    /// Resources are made available to `Item`s through their `Data` associated
    /// type.
    //
    // NOTE: When updating this mutator, also check if `CmdCtxMpsf` needs its mutator updated.
    #[builder(
        setter(prefix = "with_"),
        via_mutators(init = Resources::<Empty>::new()),
        mutators(
            /// Adds an object to the in-memory resources.
            pub fn with_resource<R>(
                &mut self,
                resource: R,
            )
            where
                R: peace_resource_rt::Resource,
            {
                self.resources.insert(resource);
            }
        )
    )]
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
    >
    CmdCtxMpsfParamsBuilder<
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
            (Resources<Empty>,),
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    pub async fn build(self) -> Result<CmdCtxMpsf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError> {
        let CmdCtxMpsfParams {
            output,
            interruptibility,
            workspace,
            profile_filter_fn,
            flow,
            mut workspace_params,
            profile_to_profile_params: profile_to_profile_params_provided,
            profile_to_flow_params: profile_to_flow_params_provided,
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

        let profiles = CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::profiles_from_peace_app_dir(
            workspace_dirs.peace_app_dir(),
            profile_filter_fn.as_ref(),
        )
        .await?;

        let (profile_dirs, profile_history_dirs) =
            CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::profile_and_history_dirs_read(
                &profiles,
                workspace_dirs,
            );

        let flow_dirs =
            CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::flow_dirs_read(&profile_dirs, &flow);

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

        // profile_params_deserialize
        let profile_to_profile_params =
            CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::profile_params_deserialize(
                &profile_dirs,
                profile_to_profile_params_provided,
                storage,
                &profile_params_type_reg,
            )
            .await?;

        // flow_params_deserialize
        let profile_to_flow_params =
            CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::flow_params_deserialize(
                &flow_dirs,
                profile_to_flow_params_provided,
                storage,
                &flow_params_type_reg,
            )
            .await?;

        let interruptibility_state = interruptibility.into();

        // Serialize params to `PeaceAppDir`.
        CmdCtxBuilderSupport::workspace_params_serialize(
            &workspace_params,
            storage,
            &workspace_params_file,
        )
        .await?;

        // profile_params_serialize
        CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::profile_params_serialize(
            &profile_to_profile_params,
            &profile_dirs,
            storage,
        )
        .await?;

        // flow_params_serialize
        CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::flow_params_serialize(
            &profile_to_flow_params,
            &flow_dirs,
            storage,
        )
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

        let flow_id = flow.flow_id();
        let item_graph = flow.graph();

        let (params_specs_type_reg, states_type_reg) =
            CmdCtxBuilderSupport::params_and_states_type_reg(item_graph);

        let app_name = workspace.app_name();
        let profile_to_params_specs =
            CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::params_specs_load_merge_and_store(
                &flow_dirs,
                profile_to_params_specs_provided,
                &flow,
                storage,
                &params_specs_type_reg,
                app_name,
            )
            .await?;

        let profile_to_states_current_stored =
            CmdCtxBuilderSupportMulti::<CmdCtxTypesT>::states_current_read(
                &flow_dirs,
                flow_id,
                storage,
                &states_type_reg,
            )
            .await?;

        // Call each `Item`'s initialization function.
        let mut resources = CmdCtxBuilderSupport::item_graph_setup(item_graph, resources).await?;

        // Needs to come before `state_example`, because params resolution may need
        // some resources to be inserted for `state_example` to work.
        resources.merge(resources_override.into_inner());

        let cmd_ctx_mpsf = CmdCtxMpsf {
            output,
            fields: CmdCtxMpsfFields {
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
            },
        };

        Ok(cmd_ctx_mpsf)
    }
}

#[allow(non_camel_case_types)]
impl<
        'ctx,
        CmdCtxTypesT,
        __interruptibility: ::typed_builder::Optional<Interruptibility<'static>> + 'ctx,
        __profile_filter_fn: ::typed_builder::Optional<Option<ProfileFilterFn>> + 'ctx,
    > IntoFuture
    for CmdCtxMpsfParamsBuilder<
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
            (Resources<Empty>,),
        ),
    >
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Future that returns the `CmdCtxMpsf`.
    ///
    /// This is boxed since [TAIT] is not yet available ([rust#63063]).
    ///
    /// [TAIT]: https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html
    /// [rust#63063]: https://github.com/rust-lang/rust/issues/63063
    type IntoFuture =
        LocalBoxFuture<'ctx, Result<CmdCtxMpsf<'ctx, CmdCtxTypesT>, CmdCtxTypesT::AppError>>;
    type Output = <Self::IntoFuture as std::future::Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        self.build().boxed_local()
    }
}
