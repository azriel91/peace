use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, token::Comma, FieldValue, GenericArgument, Token};

use crate::cmd::{
    type_params_selection::{
        FlowParamsSelection, FlowSelection, ProfileParamsSelection, ProfileSelection,
        WorkspaceParamsSelection,
    },
    CmdCtxBuilderReturnTypeBuilder, FlowCount, ParamsScope, ProfileCount, Scope, ScopeStruct,
};

/// Generates the `CmdCtxBuilder::build` methods for each type param selection.
///
/// For a command with `ProfileSelection`, `FlowSelection`, and
/// `*ParamsSelection`s type parameters, `2 * 1 * 2 * 2 * 2` = 16 variations of
/// the `build` method need to be generated, which is tedious to keep
/// consistently correct by hand:
///
/// * `ProfileSelected`, `ProfileFromWorkspaceParams`
/// * `FlowSelected<'ctx, E>`
/// * `WorkspaceParamsNone`, `WorkspaceParamsSome`
/// * `ProfileParamsNone`, `ProfileParamsSome`
/// * `FlowParamsNone`, `FlowParamsSome`
pub fn impl_build(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    ProfileSelection::iter().fold(
        proc_macro2::TokenStream::new(),
        |tokens, profile_selection| {
            match (scope_struct.scope().profile_count(), profile_selection) {
                // For `ProfileCount::None` it only makes sense to have `ProfileSelection::NotSelected`
                (
                    ProfileCount::None,
                    ProfileSelection::Selected
                    | ProfileSelection::FromWorkspaceParam
                    | ProfileSelection::FilterFunction
                ) |

                // It doesn't make sense to have `NotSelected` or `FilterFunction`
                // when profile is single.
                (
                    ProfileCount::One,
                    ProfileSelection::NotSelected
                    | ProfileSelection::FilterFunction
                ) |

                // It doesn't make sense to have `profile_from_workpace_param`
                // when profile is none or multi.
                (
                    ProfileCount::Multiple,
                    ProfileSelection::Selected | ProfileSelection::FromWorkspaceParam
                ) => return tokens,
                _ => {} // impl build
            }

            FlowSelection::iter().fold(tokens, |tokens, flow_selection| {
                WorkspaceParamsSelection::iter().fold(
                    tokens,
                    |tokens, workspace_params_selection| {
                        if profile_selection == ProfileSelection::FromWorkspaceParam
                            && workspace_params_selection != WorkspaceParamsSelection::Some
                        {
                            // Don't implement build for `ProfileFromWorkspaceParam` if the user
                            // hasn't selected a workspace parameter key.
                            return tokens;
                        }

                        ProfileParamsSelection::iter().fold(
                            tokens,
                            |tokens, profile_params_selection| {
                                if !scope_struct.scope().profile_params_supported()
                                    && profile_params_selection == ProfileParamsSelection::Some
                                {
                                    // Skip ProfileParamsSome when it isn't supported.
                                    return tokens;
                                }

                                FlowParamsSelection::iter().fold(
                                    tokens,
                                    |mut tokens, flow_params_selection| {
                                        if !scope_struct.scope().flow_params_supported()
                                            && flow_params_selection == FlowParamsSelection::Some
                                        {
                                            // Skip FlowParamsSome when it isn't supported.
                                            return tokens;
                                        }

                                        let next_build_tokens = impl_build_for(
                                            scope_struct,
                                            profile_selection,
                                            flow_selection,
                                            workspace_params_selection,
                                            profile_params_selection,
                                            flow_params_selection,
                                        );

                                        tokens.extend(next_build_tokens);

                                        tokens
                                    },
                                )
                            },
                        )
                    },
                )
            })
        },
    )
}

fn impl_build_for(
    scope_struct: &ScopeStruct,
    profile_selection: ProfileSelection,
    flow_selection: FlowSelection,
    workspace_params_selection: WorkspaceParamsSelection,
    profile_params_selection: ProfileParamsSelection,
    flow_params_selection: FlowParamsSelection,
) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let scope_type_path = scope.type_path();

    let scope_type_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();

        match scope {
            Scope::SingleProfileSingleFlow | Scope::MultiProfileSingleFlow => {
                type_params.push(parse_quote!(peace_resources::resources::ts::SetUp));
            }
            Scope::MultiProfileNoFlow | Scope::NoProfileNoFlow | Scope::SingleProfileNoFlow => {}
        }

        type_params
    };

    let workspace_dirs_and_storage_borrow = quote! {
        let workspace_dirs = self.workspace.dirs();
        let storage = self.workspace.storage();
    };
    let (workspace_params_deserialize, workspace_params_serialize, workspace_params_insert) =
        workspace_params_load_save(workspace_params_selection);

    let (profile_params_deserialize, profile_params_serialize, profile_params_insert) =
        profile_params_load_save(scope, profile_params_selection);
    let (flow_params_deserialize, flow_params_serialize, flow_params_insert) =
        flow_params_load_save(scope, flow_params_selection);

    let profile_from_workspace = profile_from_workspace(profile_selection);
    let profiles_from_peace_app_dir = profiles_from_peace_app_dir(scope, profile_selection);
    let profile_s_ref = profile_s_ref(scope, profile_selection);
    let cmd_dirs = cmd_dirs(scope);
    let dirs_to_create = dirs_to_create(scope);
    let scope_fields = scope_fields(scope);
    let states_and_params_read_and_pg_init = states_and_params_read_and_pg_init(scope);
    let resources_insert = resources_insert(scope);

    let scope_builder_deconstruct = scope_builder_deconstruct(
        scope_struct,
        scope,
        profile_selection,
        flow_selection,
        workspace_params_selection,
        profile_params_selection,
        flow_params_selection,
    );

    let workspace_params_selection_type_param = workspace_params_selection.type_param();
    let profile_params_selection_type_param = profile_params_selection.type_param();
    let flow_params_selection_type_param = flow_params_selection.type_param();
    let profile_selection_type_param = profile_selection.type_param();
    let flow_selection_type_param = flow_selection.type_param();

    // ```rust,ignore
    // crate::ctx::CmdCtxBuilder<
    //     'ctx,
    //     crate::ctx::CmdCtxBuilderTypeParamsCollector<
    //         Output,
    //         AppError,
    //         peace_rt_model::params::ParamsKeysImpl<
    //             <ParamsKeysT as peace_rt_model::params::ParamsKeys>::WorkspaceParamsKMaybe,
    //             <ParamsKeysT as peace_rt_model::params::ParamsKeys>::ProfileParamsKMaybe,
    //             <ParamsKeysT as peace_rt_model::params::ParamsKeys>::FlowParamsKMaybe,
    //         >,
    //         crate::scopes::type_params::WorkspaceParamsSome<
    //             <ParamsKeysT::WorkspaceParamsKMaybe as KeyMaybe>::Key
    //         >,
    //         crate::scopes::type_params::ProfileParamsSome<
    //             <ParamsKeysT::ProfileParamsKMaybe as KeyMaybe>::Key
    //         >,
    //         crate::scopes::type_params::FlowParamsSome<
    //             <ParamsKeysT::FlowParamsKMaybe as KeyMaybe>::Key
    //         >,
    //         crate::scopes::type_params::ProfileFromWorkspaceParam<
    //             'key,
    //             <ParamsKeysT::WorkspaceParamsKMaybe as peace_rt_model::params::KeyMaybe>::Key,
    //         >,
    //         crate::scopes::type_params::FlowNotSelected,
    //     >,
    // >
    // ```

    let builder_type = CmdCtxBuilderReturnTypeBuilder::new(scope_builder_name.clone())
        .with_output(parse_quote!(Output))
        .with_app_error(parse_quote!(AppError))
        .with_workspace_params_k(parse_quote!(
            <ParamsKeysT as peace_rt_model::params::ParamsKeys>::WorkspaceParamsKMaybe
        ))
        .with_profile_params_k(parse_quote!(
            <ParamsKeysT as peace_rt_model::params::ParamsKeys>::ProfileParamsKMaybe
        ))
        .with_flow_params_k(parse_quote!(
            <ParamsKeysT as peace_rt_model::params::ParamsKeys>::FlowParamsKMaybe
        ))
        .with_workspace_params_selection(parse_quote!(#workspace_params_selection_type_param))
        .with_profile_params_selection(parse_quote!(#profile_params_selection_type_param))
        .with_flow_params_selection(parse_quote!(#flow_params_selection_type_param))
        .with_profile_selection(parse_quote!(#profile_selection_type_param))
        .with_flow_selection(parse_quote!(#flow_selection_type_param))
        .build();

    let impl_header_and_constraints = quote! {
        impl<
            'ctx,
            'key,
            Output,
            AppError,
            ParamsKeysT,
        > #builder_type
        where
            Output: peace_rt_model::output::OutputWrite<AppError> + 'static,
            AppError: peace_value_traits::AppError + From<peace_rt_model::Error>,
            ParamsKeysT: peace_rt_model::params::ParamsKeys,

            crate::ctx::CmdCtxTypeParamsCollector<Output, AppError, ParamsKeysT>:
                crate::ctx::CmdCtxTypeParamsConstrained,
    };

    let cmd_ctx_return_type = quote! {
        crate::ctx::CmdCtx<
            #scope_type_path<
                'ctx,
                crate::ctx::CmdCtxTypeParamsCollector<Output, AppError, ParamsKeysT>,

                // MultiProfileSingleFlow / SingleProfileSingleFlow
                // peace_resources::resources::ts::SetUp
                #scope_type_params
            >,
        >
    };

    quote! {
        #impl_header_and_constraints
        {
            /// Builds the command context.
            ///
            /// This includes creating directories and deriving values based on the
            /// given parameters
            pub async fn build(
                mut self,
            ) -> Result<#cmd_ctx_return_type, AppError>
            {
                use futures::stream::TryStreamExt;

                // Values shared by subsequent function calls.
                // let workspace_dirs = self.workspace.dirs();
                // let storage = self.workspace.storage();
                #workspace_dirs_and_storage_borrow

                // let workspace_params_file = WorkspaceParamsFile::from(workspace_dirs.peace_app_dir());
                // self.workspace_params_merge(&workspace_params_file).await?;
                #workspace_params_deserialize

                // let profile = self
                //     .scope_builder
                //     .workspace_params_selection
                //     .0
                //     .get(self.scope_builder.profile_selection.0)
                //     .cloned()
                //     .ok_or(Error::WorkspaceParamsProfileNone)?;
                #profile_from_workspace

                // MultiProfile
                #profiles_from_peace_app_dir

                // === Profile(s) ref === //
                // --- Single --- //
                // // ProfileSelected
                // let profile_s_ref = &self.scope_builder.profile_selection.0;
                // // ProfileFromWorkspaceParam
                // let profile_s_ref = &profile;
                // --- Multi --- //
                // let profile_s_ref = &profiles;
                #profile_s_ref

                // === Cmd dirs === //
                // --- Single Profile --- //
                // let profile_dir = ProfileDir::from((workspace_dirs.peace_app_dir(), profile_s_ref));
                // let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
                // --- Multi Profile --- //
                // let (profile_dirs, profile_history_dirs) = profile_s_ref
                //     .iter()
                //     .fold((
                //         std::collections::BTreeMap::<
                //             peace_core::Profile,
                //             peace_resources::paths::ProfileDir
                //         >::new(),
                //         std::collections::BTreeMap::<
                //             peace_core::Profile,
                //             peace_resources::paths::ProfileHistoryDir
                //         >::new()
                //     ), |(mut profile_dirs, mut profile_history_dirs), profile| {
                //         let profile_dir = peace_resources::paths::ProfileDir::from(
                //             (workspace_dirs.peace_app_dir(), profile)
                //         );
                //         let profile_history_dir = peace_resources::paths::ProfileHistoryDir::from(&profile_dir);
                //
                //         profile_dirs.insert(profile.clone(), profile_dir);
                //         profile_history_dirs.insert(profile.clone(), profile_history_dir);
                //
                //         (profile_dirs, profile_history_dirs)
                //     });
                // --- Single Profile Single Flow --- //
                // let flow_dir = FlowDir::from((
                //     &profile_dir,
                //     self.scope_builder.flow_selection.0.flow_id()
                // ));
                // --- Multi Profile Single Flow --- //
                // let flow_dirs = profile_dirs
                //     .iter()
                //     .fold(std::collections::BTreeMap::<
                //             peace_core::Profile,
                //             peace_resources::paths::ProfileDir
                //         >::new(
                //     ), |mut flow_dirs, (profile, profile_dir)| {
                //         let flow_dir = peace_resources::paths::FlowDir::from((
                //             profile_dir,
                //             self.scope_builder.flow_selection.0.flow_id()
                //         ));
                //
                //         flow_dirs.insert(profile.clone(), flow_dir);
                //
                //         flow_dirs
                //     });
                #cmd_dirs

                let dirs_to_create = [
                    // AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
                    // AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
                    // AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),

                    // === Profile === //
                    // --- Single --- //
                    // AsRef::<std::path::Path>::as_ref(&profile_dir),
                    // AsRef::<std::path::Path>::as_ref(&profile_history_dir),
                    // === Flow ID === //
                    // --- Single --- //
                    // AsRef::<std::path::Path>::as_ref(&flow_dir),
                    #dirs_to_create
                ];

                // === Profile Params === //
                // --- Single --- //
                // let profile_params_file = ProfileParamsFile::from(&profile_dir);
                // self.profile_params_merge(&profile_params_file).await?;
                // --- Multi --- //
                // let profile_to_profile_params = futures::stream::iter(
                //     profile_dirs
                //         .iter()
                //         .map(Result::<_, peace_rt_model::Error>::Ok)
                //     )
                //     .and_then(|(profile, profile_dir)| async move {
                //         let profile_params_file =
                //             peace_resources::internal::ProfileParamsFile::from(profile_dir);
                //
                //         let profile_params = self
                //             .#params_deserialize_method_name(&profile_params_file)
                //             .await?;
                //
                //         Ok((profile.clone(), profile_params))
                //     })
                //     .try_collect::<
                //         std::collections::BTreeMap<
                //             peace_core::Profile,
                //             _ // peace_rt_model::params::ProfileParams<K>
                //         >
                //     >()
                //     .await?;
                #profile_params_deserialize

                // === Flow Params === //
                // --- Single --- //
                // let flow_params_file = ProfileParamsFile::from(&flow_dir);
                // self.flow_params_merge(&flow_params_file).await?;
                // --- Multi --- //
                // let profile_to_flow_params = futures::stream::iter(
                //     flow_dirs
                //         .iter()
                //         .map(Result::<_, peace_rt_model::Error>::Ok)
                //     )
                //     .and_then(|(profile, flow_dir)| async move {
                //         let flow_params_file =
                //             peace_resources::internal::FlowParamsFile::from(flow_dir);
                //
                //         let flow_params = self
                //             .#params_deserialize_method_name(&flow_params_file)
                //             .await?;
                //
                //         Ok((profile.clone(), flow_params))
                //     })
                //     .try_collect::<
                //         std::collections::BTreeMap<
                //             peace_core::Profile,
                //             _ // peace_rt_model::params::FlowParams<K>
                //         >
                //     >()
                //     .await?;
                #flow_params_deserialize

                // Create directories and write init parameters to storage.
                #[cfg(target_arch = "wasm32")]
                peace_rt_model::WorkspaceInitializer::dirs_create(storage, dirs_to_create).await?;
                #[cfg(not(target_arch = "wasm32"))]
                {
                    peace_rt_model::WorkspaceInitializer::dirs_create(dirs_to_create).await?;

                    let workspace_dir = workspace_dirs.workspace_dir();
                    std::env::set_current_dir(workspace_dir).map_err(|error| {
                        peace_rt_model::Error::Native(peace_rt_model::NativeError::CurrentDirSet {
                            workspace_dir: workspace_dir.clone(),
                            error,
                        })
                    })?;
                }

                // let crate::ctx::CmdCtxBuilder {
                //     output,
                //     interruptibility,
                //     workspace,
                //     scope_builder:
                //         #scope_builder_name {
                //             profile_selection: ProfileSelected(profile)
                //                             // ProfileFromWorkspaceParam(_workspace_params_k),
                //                             // ProfileFilterFn(profiles_filter_fn)
                //
                //         flow_selection: FlowSelected(flow),
                //         params_type_regs_builder,
                //         workspace_params_selection: WorkspaceParamsSome(workspace_params),
                //         profile_params_selection: ProfileParamsSome(profile_params),
                //         flow_params_selection: FlowParamsNone,
                //
                //         // === SingleProfileSingleFlow === //
                //         params_specs_provided,
                //     },
                // } = self;
                #scope_builder_deconstruct
                let interruptibility_state = interruptibility.into();

                // Serialize params to `PeaceAppDir`.

                // crate::ctx::cmd_ctx_builder::workspace_params_serialize(
                //     &workspace_params,
                //     storage,
                //     &workspace_params_file,
                // )
                // .await?;
                #workspace_params_serialize

                // crate::ctx::cmd_ctx_builder::profile_params_serialize(
                //     &profile_params,
                //     storage,
                //     &profile_params_file
                // )
                // .await?;
                #profile_params_serialize

                // crate::ctx::cmd_ctx_builder::flow_params_serialize(
                //     &flow_params,
                //     storage,
                //     &flow_params_file
                // )
                // .await?;
                #flow_params_serialize

                // Track items in memory.
                let mut resources = peace_resources::Resources::new();
                // === WorkspaceParamsSelected === //
                // crate::ctx::cmd_ctx_builder::workspace_params_insert(workspace_params, &mut resources);
                // resources.insert(workspace_params_file);
                #workspace_params_insert

                // === Single Profile === //
                // crate::ctx::cmd_ctx_builder::profile_params_insert(profile_params, &mut resources);
                // resources.insert(profile_params_file);
                #profile_params_insert

                // === Single Flow === //
                // crate::ctx::cmd_ctx_builder::flow_params_insert(flow_params, &mut resources);
                // resources.insert(flow_params_file);
                #flow_params_insert

                // Insert resources
                //
                // === MultiProfileSingleFlow === //
                // {
                //     let (app_name, workspace_dirs, storage) = workspace.clone().into_inner();
                //     let (workspace_dir, peace_dir, peace_app_dir) = workspace_dirs.into_inner();
                //
                //     resources.insert(app_name);
                //     resources.insert(storage);
                //     resources.insert(workspace_dir);
                //     resources.insert(peace_dir);
                //     resources.insert(peace_app_dir);
                //     resources.insert(flow.flow_id().clone());
                // }
                // === SingleProfileSingleFlow === //
                // {
                //     let (app_name, workspace_dirs, storage) = workspace.clone().into_inner();
                //     let (workspace_dir, peace_dir, peace_app_dir) = workspace_dirs.into_inner();
                //
                //     resources.insert(app_name);
                //     resources.insert(storage);
                //     resources.insert(workspace_dir);
                //     resources.insert(peace_dir);
                //     resources.insert(peace_app_dir);
                //     resources.insert(profile_dir.clone());
                //     resources.insert(profile_history_dir.clone());
                //     resources.insert(profile.clone());
                //     resources.insert(flow_dir.clone());
                //     resources.insert(flow.flow_id().clone());
                // }
                #resources_insert

                // === MultiProfileSingleFlow === //
                // let flow_id = flow.flow_id();
                // let item_graph = flow.graph();
                //
                // let (params_specs_type_reg, states_type_reg) =
                //     crate::ctx::cmd_ctx_builder::params_and_states_type_reg(item_graph);
                //
                // let params_specs_type_reg_ref = &params_specs_type_reg;
                // let profile_to_params_specs = futures::stream::iter(
                //     flow_dirs
                //         .iter()
                //         .map(Result::<_, peace_rt_model::Error>::Ok)
                //     )
                //     .and_then(|(profile, flow_dir)| {
                //         let params_specs_provided = params_specs_provided.clone();
                //         async move {
                //             let params_specs_file =
                //                 peace_resources::paths::ParamsSpecsFile::from(flow_dir);
                //
                //             let params_specs_stored = peace_rt_model::ParamsSpecsSerializer::<
                //                 peace_rt_model::Error
                //             >::deserialize_opt(
                //                 profile,
                //                 flow_id,
                //                 storage,
                //                 params_specs_type_reg_ref,
                //                 &params_specs_file,
                //             )
                //             .await?;
                //
                //             // For mapping fns, we still need the developer to provide the params spec
                //             // so that multi-profile diffs can be done.
                //             let params_specs = params_specs_stored.map(|params_specs_stored| {
                //                 crate::ctx::cmd_ctx_builder::params_specs_merge(
                //                     &flow,
                //                     params_specs_provided,
                //                     Some(params_specs_stored),
                //                 )
                //             })
                //             .transpose()?;
                //
                //             // Note: we don't serialize params specs back to disk.
                //
                //             Ok((profile.clone(), params_specs))
                //         }
                //     })
                //     .try_collect::<
                //         std::collections::BTreeMap<
                //             peace_core::Profile,
                //             Option<peace_params::ParamsSpecs>
                //         >
                //     >()
                //     .await?;
                //
                // let states_type_reg_ref = &states_type_reg;
                // let profile_to_states_current_stored = futures::stream::iter(
                //     flow_dirs
                //         .iter()
                //         .map(Result::<_, peace_rt_model::Error>::Ok)
                //     )
                //     .and_then(|(profile, flow_dir)| async move {
                //         let states_current_file = peace_resources::paths::StatesCurrentFile::from(flow_dir);
                //
                //         let states_current_stored = peace_rt_model::StatesSerializer::<
                //             peace_rt_model::Error
                //         >::deserialize_stored_opt(
                //             flow_id,
                //             storage,
                //             states_type_reg_ref,
                //             &states_current_file,
                //         )
                //         .await?
                //         .map(Into::<peace_resources::states::StatesCurrentStored>::into);
                //
                //         Ok((profile.clone(), states_current_stored))
                //     })
                //     .try_collect::<
                //         std::collections::BTreeMap<
                //             peace_core::Profile,
                //             Option<peace_resources::states::StatesCurrentStored>
                //         >
                //     >()
                //     .await?;
                //
                // // Call each `Item`'s initialization function.
                // let resources = crate::ctx::cmd_ctx_builder::item_graph_setup(
                //     item_graph,
                //     resources
                // )
                // .await?;
                //
                // === SingleProfileSingleFlow === //
                // // Set up resources for the flow's item graph
                // let flow_id = flow.flow_id();
                // let item_graph = flow.graph();
                //
                // let (params_specs_type_reg, states_type_reg) =
                //     crate::ctx::cmd_ctx_builder::params_and_states_type_reg(item_graph);
                //
                // // Params specs loading and storage.
                // let params_specs_type_reg_ref = &params_specs_type_reg;
                // let params_specs_file = peace_resources::paths::ParamsSpecsFile::from(&flow_dir);
                // let params_specs_stored = peace_rt_model::ParamsSpecsSerializer::<
                //     peace_rt_model::Error
                // >::deserialize_opt(
                //     &profile,
                //     flow_id,
                //     storage,
                //     params_specs_type_reg_ref,
                //     &params_specs_file,
                // )
                // .await?;
                //
                // let params_specs = crate::ctx::cmd_ctx_builder::params_specs_merge(
                //     &flow,
                //     params_specs_provided,
                //     params_specs_stored,
                // )?;
                //
                // crate::ctx::cmd_ctx_builder::params_specs_serialize(
                //     &params_specs,
                //     storage,
                //     &params_specs_file,
                // )
                // .await?;
                //
                // // States loading and storage.
                // let states_type_reg_ref = &states_type_reg;
                // let states_current_file = peace_resources::paths::StatesCurrentFile::from(&flow_dir);
                // let states_current_stored = peace_rt_model::StatesSerializer::<
                //     peace_rt_model::Error
                // >::deserialize_stored_opt(
                //     flow_id,
                //     storage,
                //     states_type_reg_ref,
                //     &states_current_file,
                // )
                // .await?
                // .map(Into::<peace_resources::states::StatesCurrentStored>::into);
                // if let Some(states_current_stored) = states_current_stored {
                //     resources.insert(states_current_stored);
                // }
                //
                // // Call each `Item`'s initialization function.
                // let resources = crate::ctx::cmd_ctx_builder::item_graph_setup(
                //     item_graph,
                //     resources
                // )
                // .await?;
                //
                // // output_progress CmdProgressTracker initialization
                // #[cfg(feature = "output_progress")]
                // let cmd_progress_tracker = {
                //     let multi_progress = indicatif::MultiProgress::with_draw_target(
                //         indicatif::ProgressDrawTarget::hidden()
                //     );
                //     let progress_trackers = item_graph.iter_insertion().fold(
                //         peace_rt_model::IndexMap::with_capacity(item_graph.node_count()),
                //         |mut progress_trackers, item| {
                //             let progress_bar = multi_progress.add(indicatif::ProgressBar::hidden());
                //             let progress_tracker = peace_core::progress::ProgressTracker::new(progress_bar);
                //             progress_trackers.insert(item.id().clone(), progress_tracker);
                //             progress_trackers
                //         },
                //     );
                //
                //     peace_rt_model::CmdProgressTracker::new(multi_progress, progress_trackers)
                // };
                #states_and_params_read_and_pg_init

                let params_type_regs = params_type_regs_builder.build();

                let scope = #scope_type_path::new(
                    // output,
                    // interruptibility_state,
                    // workspace,

                    // === SingleProfileSingleFlow === //
                    // #[cfg(feature = "output_progress")]
                    // cmd_progress_tracker,

                    // params_type_regs,

                    // workspace_params

                    // === Profile === //
                    // --- Single --- //
                    // profile,
                    // profile_dir,
                    // profile_history_dir,
                    // workspace_params
                    // profile_params,
                    // --- Multi --- //
                    // profiles,
                    // profile_dirs,
                    // profile_history_dirs,
                    // workspace_params
                    // profile_to_profile_params,

                    // === Flow ID === //
                    // --- Single --- //
                    // flow,
                    // flow_dir,
                    // flow_params,
                    // --- Multi --- //
                    // flow,
                    // flow_dirs,
                    // profile_to_flow_params,

                    // === MultiProfileSingleFlow === //
                    // profile_to_states_current_stored,
                    // params_specs_type_reg,
                    // profile_to_params_specs,
                    // states_type_reg,
                    // resources,
                    // === SingleProfileSingleFlow === //
                    // params_specs_type_reg,
                    // params_specs,
                    // states_type_reg,
                    // resources,

                    #scope_fields
                );

                Ok(crate::ctx::CmdCtx {
                    scope,
                })
            }
        }

        impl<
            'ctx,
            'key: 'ctx,
            Output,
            AppError,
            ParamsKeysT,
        > std::future::IntoFuture for #builder_type
        where
            Output: peace_rt_model::output::OutputWrite<AppError> + 'static,
            AppError: peace_value_traits::AppError + From<peace_rt_model::Error>,
            ParamsKeysT: peace_rt_model::params::ParamsKeys,

            crate::ctx::CmdCtxTypeParamsCollector<Output, AppError, ParamsKeysT>:
                crate::ctx::CmdCtxTypeParamsConstrained,
        {
            /// Future that returns the `CmdCtx`.
            ///
            /// This is boxed since [TAIT] is not yet available.
            ///
            /// [TAIT]: https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html
            type IntoFuture = std::pin::Pin<
                Box<
                    dyn std::future::Future<
                        Output = Result<#cmd_ctx_return_type, AppError>
                    >
                    + 'ctx,
                >,
            >;
            type Output = <Self::IntoFuture as std::future::Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.build())
            }
        }

    }
}

fn scope_builder_deconstruct(
    scope_struct: &ScopeStruct,
    scope: Scope,
    profile_selection: ProfileSelection,
    flow_selection: FlowSelection,
    workspace_params_selection: WorkspaceParamsSelection,
    profile_params_selection: ProfileParamsSelection,
    flow_params_selection: FlowParamsSelection,
) -> proc_macro2::TokenStream {
    let scope_builder_name = &scope_struct.item_struct().ident;
    let mut scope_builder_fields = Punctuated::<FieldValue, Token![,]>::new();

    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One | ProfileCount::Multiple => match profile_selection {
            ProfileSelection::NotSelected => scope_builder_fields.push(parse_quote! {
                profile_selection: crate::scopes::type_params::ProfileNotSelected
            }),
            ProfileSelection::Selected => scope_builder_fields.push(parse_quote! {
                profile_selection: crate::scopes::type_params::ProfileSelected(profile)
            }),
            ProfileSelection::FromWorkspaceParam => scope_builder_fields.push(parse_quote! {
                profile_selection:
                    crate::scopes::type_params::ProfileFromWorkspaceParam(
                        _workspace_params_k
                    )
            }),
            ProfileSelection::FilterFunction => scope_builder_fields.push(parse_quote! {
                profile_selection:
                    crate::scopes::type_params::ProfileFilterFn(profiles_filter_fn)
            }),
        },
    }

    if scope.flow_count() == FlowCount::One {
        match flow_selection {
            FlowSelection::Selected => scope_builder_fields.push(parse_quote! {
                flow_selection: crate::scopes::type_params::FlowSelected(flow)
            }),
        }
    }

    scope_builder_fields.push(parse_quote!(params_type_regs_builder));
    scope_builder_fields.push(workspace_params_selection.deconstruct());
    if scope.profile_params_supported() {
        scope_builder_fields.push(profile_params_selection.deconstruct());
    }
    if scope.flow_params_supported() {
        scope_builder_fields.push(flow_params_selection.deconstruct());
    }

    if scope.flow_count() == FlowCount::One {
        scope_builder_fields.push(parse_quote! {
            params_specs_provided
        });
    }

    quote! {
        let crate::ctx::CmdCtxBuilder {
            output,
            interruptibility,
            workspace,
            scope_builder: #scope_builder_name {
                // profile_selection: ProfileSelected(profile),
                // flow_selection: FlowSelected(flow),
                // params_type_regs_builder,
                // workspace_params_selection: WorkspaceParamsSome(workspace_params),
                // profile_params_selection: ProfileParamsSome(profile_params),
                // flow_params_selection: FlowParamsNone,

                // // === SingleProfileSingleFlow === //
                // params_specs_provided,

                #scope_builder_fields,
            },
        } = self;
    }
}

/// Load from `workspace_params_file` and serialize when
/// `WorkspaceParamsSelection` is `Some`.
fn workspace_params_load_save(
    workspace_params_selection: WorkspaceParamsSelection,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    match workspace_params_selection {
        WorkspaceParamsSelection::None => {
            let workspace_params_deserialize = quote! {
                let workspace_params = peace_rt_model::params::WorkspaceParams::<
                    <
                        ParamsKeysT::WorkspaceParamsKMaybe
                        as peace_rt_model::params::KeyMaybe
                    >::Key
                >::new();
            };
            (
                workspace_params_deserialize,
                proc_macro2::TokenStream::new(),
                proc_macro2::TokenStream::new(),
            )
        }
        WorkspaceParamsSelection::Some => {
            let workspace_params_deserialize = quote! {
                let workspace_params_file = peace_resources::internal::WorkspaceParamsFile::from(
                    workspace_dirs.peace_app_dir()
                );

                self.workspace_params_merge(&workspace_params_file).await?;
            };
            let workspace_params_serialize = quote! {
                crate::ctx::cmd_ctx_builder::workspace_params_serialize(
                    &workspace_params,
                    storage,
                    &workspace_params_file,
                )
                .await?;
            };
            let workspace_params_insert = quote! {
                crate::ctx::cmd_ctx_builder::workspace_params_insert(workspace_params.clone(), &mut resources);
                resources.insert(workspace_params_file);
            };

            (
                workspace_params_deserialize,
                workspace_params_serialize,
                workspace_params_insert,
            )
        }
    }
}

/// Load from `profile_params_file` and serialize when
/// `ProfileParamsSelection` is `Some`.
fn profile_params_load_save(
    scope: Scope,
    profile_params_selection: ProfileParamsSelection,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    match scope.profile_count() {
        ProfileCount::None => (
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
        ),
        ProfileCount::One => match profile_params_selection {
            ProfileParamsSelection::None => {
                let profile_params_deserialize = quote! {
                    let profile_params = peace_rt_model::params::ProfileParams::<
                        <
                            ParamsKeysT::ProfileParamsKMaybe
                            as peace_rt_model::params::KeyMaybe
                        >::Key
                    >::new();
                };
                (
                    profile_params_deserialize,
                    proc_macro2::TokenStream::new(),
                    proc_macro2::TokenStream::new(),
                )
            }
            ProfileParamsSelection::Some => {
                let profile_params_deserialize = quote! {
                    let profile_params_file = peace_resources::internal::ProfileParamsFile::from(
                        &profile_dir
                    );

                    self.profile_params_merge(&profile_params_file).await?;
                };
                let profile_params_serialize = quote! {
                    crate::ctx::cmd_ctx_builder::profile_params_serialize(
                        &profile_params,
                        storage,
                        &profile_params_file,
                    )
                    .await?;
                };
                let profile_params_insert = quote! {
                    crate::ctx::cmd_ctx_builder::profile_params_insert(profile_params.clone(), &mut resources);
                    resources.insert(profile_params_file);
                };

                (
                    profile_params_deserialize,
                    profile_params_serialize,
                    profile_params_insert,
                )
            }
        },
        ProfileCount::Multiple => {
            let profile_params_deserialize = match profile_params_selection {
                ProfileParamsSelection::None => quote! {
                    let profile_to_profile_params = std::collections::BTreeMap::<
                        peace_core::Profile,
                        peace_rt_model::params::ProfileParams<
                            <
                                ParamsKeysT::ProfileParamsKMaybe as
                                peace_rt_model::params::KeyMaybe
                            >::Key
                        >
                    >::new();
                },
                ProfileParamsSelection::Some => {
                    let params_deserialize_method_name =
                        ParamsScope::Profile.params_deserialize_method_name();

                    quote! {
                        let storage = self.workspace.storage();
                        let params_type_regs_builder = &self.scope_builder.params_type_regs_builder;
                        let profile_to_profile_params = futures::stream::iter(
                            profile_dirs
                                .iter()
                                .map(Result::<_, peace_rt_model::Error>::Ok)
                            )
                            .and_then(|(profile, profile_dir)| async move {
                                let profile_params_file =
                                    peace_resources::internal::ProfileParamsFile::from(profile_dir);

                                let profile_params = Self::#params_deserialize_method_name(
                                    storage,
                                    params_type_regs_builder,
                                    &profile_params_file
                                )
                                .await?
                                .unwrap_or_default();

                                Ok((profile.clone(), profile_params))
                            })
                            .try_collect::<
                                std::collections::BTreeMap<
                                    peace_core::Profile,
                                    peace_rt_model::params::ProfileParams<
                                        <
                                            ParamsKeysT::ProfileParamsKMaybe as
                                            peace_rt_model::params::KeyMaybe
                                        >::Key
                                    >
                                >
                            >()
                            .await?;
                    }
                }
            };

            // Storage is not supported.
            let profile_params_serialize = proc_macro2::TokenStream::new();

            // Insertion into resources is not supported.
            let profile_params_insert = proc_macro2::TokenStream::new();

            (
                profile_params_deserialize,
                profile_params_serialize,
                profile_params_insert,
            )
        }
    }
}

/// Load from `flow_params_file` and serialize when
/// `FlowParamsSelection` is `Some`.
fn flow_params_load_save(
    scope: Scope,
    flow_params_selection: FlowParamsSelection,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    match scope.profile_count() {
        ProfileCount::None => (
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
        ),
        ProfileCount::One => match flow_params_selection {
            FlowParamsSelection::None => {
                let flow_params_deserialize = quote! {
                    let flow_params = peace_rt_model::params::FlowParams::<
                        <
                            ParamsKeysT::FlowParamsKMaybe as
                            peace_rt_model::params::KeyMaybe
                        >::Key
                    >::new();
                };
                (
                    flow_params_deserialize,
                    proc_macro2::TokenStream::new(),
                    proc_macro2::TokenStream::new(),
                )
            }
            FlowParamsSelection::Some => {
                let flow_params_deserialize = quote! {
                    let flow_params_file = peace_resources::internal::FlowParamsFile::from(
                        &flow_dir
                    );

                    self.flow_params_merge(&flow_params_file).await?;
                };
                let flow_params_serialize = quote! {
                    crate::ctx::cmd_ctx_builder::flow_params_serialize(
                        &flow_params,
                        storage,
                        &flow_params_file,
                    )
                    .await?;
                };
                let flow_params_insert = quote! {
                    crate::ctx::cmd_ctx_builder::flow_params_insert(flow_params.clone(), &mut resources);
                    resources.insert(flow_params_file);
                };

                (
                    flow_params_deserialize,
                    flow_params_serialize,
                    flow_params_insert,
                )
            }
        },
        ProfileCount::Multiple => {
            let flow_params_deserialize = match flow_params_selection {
                FlowParamsSelection::None => quote! {
                    let profile_to_flow_params = std::collections::BTreeMap::<
                        peace_core::Profile,
                        peace_rt_model::params::FlowParams<
                            <
                                ParamsKeysT::FlowParamsKMaybe as
                                peace_rt_model::params::KeyMaybe
                            >::Key
                        >
                    >::new();
                },
                FlowParamsSelection::Some => {
                    let params_deserialize_method_name =
                        ParamsScope::Flow.params_deserialize_method_name();
                    quote! {
                        let storage = self.workspace.storage();
                        let params_type_regs_builder = &self.scope_builder.params_type_regs_builder;
                        let profile_to_flow_params = futures::stream::iter(
                            flow_dirs
                                .iter()
                                .map(Result::<_, peace_rt_model::Error>::Ok)
                            )
                            .and_then(|(profile, flow_dir)| async move {
                                let flow_params_file =
                                    peace_resources::internal::FlowParamsFile::from(flow_dir);

                                let flow_params = Self::#params_deserialize_method_name(
                                    storage,
                                    params_type_regs_builder,
                                    &flow_params_file
                                )
                                .await?
                                .unwrap_or_default();

                                Ok((profile.clone(), flow_params))
                            })
                            .try_collect::<
                                std::collections::BTreeMap<
                                    peace_core::Profile,
                                    peace_rt_model::params::FlowParams<
                                        <
                                            ParamsKeysT::FlowParamsKMaybe as
                                            peace_rt_model::params::KeyMaybe
                                        >::Key
                                    >
                                >
                            >()
                            .await?;
                    }
                }
            };
            // Storage is not supported.
            let flow_params_serialize = proc_macro2::TokenStream::new();

            // Insertion into resources is not supported.
            let flow_params_insert = proc_macro2::TokenStream::new();

            (
                flow_params_deserialize,
                flow_params_serialize,
                flow_params_insert,
            )
        }
    }
}

fn profile_from_workspace(profile_selection: ProfileSelection) -> proc_macro2::TokenStream {
    if profile_selection == ProfileSelection::FromWorkspaceParam {
        quote! {
            let profile = self
                .scope_builder
                .workspace_params_selection
                .0
                .get(self.scope_builder.profile_selection.0)
                .cloned()
                .ok_or(peace_rt_model::Error::WorkspaceParamsProfileNone)?;
        }
    } else {
        proc_macro2::TokenStream::new()
    }
}

fn profiles_from_peace_app_dir(
    scope: Scope,
    profile_selection: ProfileSelection,
) -> proc_macro2::TokenStream {
    match scope.profile_count() {
        ProfileCount::None | ProfileCount::One => proc_macro2::TokenStream::new(),
        ProfileCount::Multiple => match profile_selection {
            ProfileSelection::NotSelected => quote! {
                let profiles = crate::ctx::cmd_ctx_builder::profiles_from_peace_app_dir(
                    workspace_dirs.peace_app_dir(),
                    None,
                ).await?;
            },
            ProfileSelection::Selected | ProfileSelection::FromWorkspaceParam => unreachable!(
                "Multiple profiles should not reach `ProfileSelection::Single` | \
                `ProfileSelection::FromWorkspaceParam`."
            ),
            ProfileSelection::FilterFunction => quote! {
                let profiles_filter_fn = self.scope_builder.profile_selection.0.as_ref();
                let profiles = crate::ctx::cmd_ctx_builder::profiles_from_peace_app_dir(
                    workspace_dirs.peace_app_dir(),
                    Some(profiles_filter_fn),
                ).await?;
            },
        },
    }
}

fn profile_s_ref(scope: Scope, profile_selection: ProfileSelection) -> proc_macro2::TokenStream {
    match scope.profile_count() {
        ProfileCount::None => proc_macro2::TokenStream::new(),
        ProfileCount::One => {
            if profile_selection == ProfileSelection::FromWorkspaceParam {
                quote!(let profile_s_ref = &profile;)
            } else {
                quote!(let profile_s_ref = &self.scope_builder.profile_selection.0;)
            }
        }
        ProfileCount::Multiple => quote!(let profile_s_ref = &profiles;),
    }
}

/// * SingleProfile:
///
///     `profile_s_ref` is expected to be a `&Profile`.
///
///     ```rust,ignore
///     profile_dir
///     profile_history_dir
///     ```
///
/// * MultiProfile:
///
///     `profile_s_ref` is expected to be a `&Vec<Profile>`.
///
///     ```rust,ignore
///     profile_dirs
///     profile_history_dirs
///     ```
///
/// * SingleFlow:
///
///     ```rust,ignore
///     flow_dir
///     ```
fn cmd_dirs(scope: Scope) -> proc_macro2::TokenStream {
    let mut dirs_tokens = proc_macro2::TokenStream::new();

    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One => {
            dirs_tokens.extend(quote! {
                let profile_dir = peace_resources::paths::ProfileDir::from((workspace_dirs.peace_app_dir(), profile_s_ref));
                let profile_history_dir = peace_resources::paths::ProfileHistoryDir::from(&profile_dir);
            });
        }
        ProfileCount::Multiple => {
            dirs_tokens.extend(quote! {
                let (profile_dirs, profile_history_dirs) = profile_s_ref
                    .iter()
                    .fold((
                        std::collections::BTreeMap::<
                            peace_core::Profile,
                            peace_resources::paths::ProfileDir
                        >::new(),
                        std::collections::BTreeMap::<
                            peace_core::Profile,
                            peace_resources::paths::ProfileHistoryDir
                        >::new()
                    ), |(mut profile_dirs, mut profile_history_dirs), profile| {
                        let profile_dir = peace_resources::paths::ProfileDir::from(
                            (workspace_dirs.peace_app_dir(), profile)
                        );
                        let profile_history_dir = peace_resources::paths::ProfileHistoryDir::from(&profile_dir);

                        profile_dirs.insert(profile.clone(), profile_dir);
                        profile_history_dirs.insert(profile.clone(), profile_history_dir);

                        (profile_dirs, profile_history_dirs)
                    });
            });
        }
    }

    if scope.flow_count() == FlowCount::One {
        match scope.profile_count() {
            ProfileCount::None => {}
            ProfileCount::One => {
                dirs_tokens.extend(quote! {
                    let flow_dir = peace_resources::paths::FlowDir::from((
                        &profile_dir,
                        self.scope_builder.flow_selection.0.flow_id()
                    ));
                });
            }
            ProfileCount::Multiple => {
                dirs_tokens.extend(quote! {
                    let flow_dirs = profile_dirs
                        .iter()
                        .fold(std::collections::BTreeMap::<
                                peace_core::Profile,
                                peace_resources::paths::FlowDir
                            >::new(
                        ), |mut flow_dirs, (profile, profile_dir)| {
                            let flow_dir = peace_resources::paths::FlowDir::from((
                                profile_dir,
                                self.scope_builder.flow_selection.0.flow_id()
                            ));

                            flow_dirs.insert(profile.clone(), flow_dir);

                            flow_dirs
                        });
                });
            }
        }
    }

    dirs_tokens
}

fn dirs_to_create(scope: Scope) -> proc_macro2::TokenStream {
    let mut dirs_tokens = quote! {
        AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
        AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
        AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),
    };

    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One => {
            dirs_tokens.extend(quote! {
                AsRef::<std::path::Path>::as_ref(&profile_dir),
                AsRef::<std::path::Path>::as_ref(&profile_history_dir),
            });
        }
        ProfileCount::Multiple => {
            // Don't create any directories
        }
    }

    if scope.flow_count() == FlowCount::One {
        match scope.profile_count() {
            ProfileCount::None => {}
            ProfileCount::One => {
                dirs_tokens.extend(quote! {
                    AsRef::<std::path::Path>::as_ref(&flow_dir),
                });
            }
            ProfileCount::Multiple => {
                // Don't create any directories
            }
        }
    }

    dirs_tokens
}

fn scope_fields(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut scope_fields = Punctuated::<FieldValue, Token![,]>::new();

    scope_fields.push(parse_quote!(output));
    scope_fields.push(parse_quote!(interruptibility_state));
    scope_fields.push(parse_quote!(workspace));

    // progress tracker
    match scope {
        Scope::MultiProfileNoFlow
        | Scope::NoProfileNoFlow
        | Scope::SingleProfileNoFlow
        | Scope::MultiProfileSingleFlow => {}
        Scope::SingleProfileSingleFlow => {
            scope_fields.push(parse_quote! {
                #[cfg(feature = "output_progress")]
                cmd_progress_tracker
            });
        }
    }

    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One => {
            scope_fields.push(parse_quote!(profile));
            scope_fields.push(parse_quote!(profile_dir));
            scope_fields.push(parse_quote!(profile_history_dir));
        }
        ProfileCount::Multiple => {
            scope_fields.push(parse_quote!(profiles));
            scope_fields.push(parse_quote!(profile_dirs));
            scope_fields.push(parse_quote!(profile_history_dirs));
        }
    }
    match scope.flow_count() {
        FlowCount::None => {}
        FlowCount::One => match scope.profile_count() {
            ProfileCount::None => {}
            ProfileCount::One => {
                scope_fields.push(parse_quote!(flow));
                scope_fields.push(parse_quote!(flow_dir));
            }
            ProfileCount::Multiple => {
                scope_fields.push(parse_quote!(flow));
                scope_fields.push(parse_quote!(flow_dirs));
            }
        },
    }

    scope_fields.push(parse_quote!(params_type_regs));

    match scope.profile_count() {
        ProfileCount::None => {
            scope_fields.push(parse_quote!(workspace_params));
        }
        ProfileCount::One => {
            scope_fields.push(parse_quote!(workspace_params));
            scope_fields.push(parse_quote!(profile_params));
        }
        ProfileCount::Multiple => {
            scope_fields.push(parse_quote!(workspace_params));
            scope_fields.push(parse_quote!(profile_to_profile_params));
        }
    }
    match scope.flow_count() {
        FlowCount::None => {}
        FlowCount::One => match scope.profile_count() {
            ProfileCount::None => {}
            ProfileCount::One => {
                scope_fields.push(parse_quote!(flow_params));
            }
            ProfileCount::Multiple => {
                scope_fields.push(parse_quote!(profile_to_flow_params));
            }
        },
    }

    match scope {
        Scope::MultiProfileNoFlow | Scope::NoProfileNoFlow | Scope::SingleProfileNoFlow => {}
        Scope::MultiProfileSingleFlow => {
            scope_fields.push(parse_quote!(profile_to_states_current_stored));
            scope_fields.push(parse_quote!(params_specs_type_reg));
            scope_fields.push(parse_quote!(profile_to_params_specs));
            scope_fields.push(parse_quote!(states_type_reg));
            scope_fields.push(parse_quote!(resources));
        }
        Scope::SingleProfileSingleFlow => {
            scope_fields.push(parse_quote!(params_specs_type_reg));
            scope_fields.push(parse_quote!(params_specs));
            scope_fields.push(parse_quote!(states_type_reg));
            scope_fields.push(parse_quote!(resources));
        }
    }

    scope_fields
}

fn states_and_params_read_and_pg_init(scope: Scope) -> proc_macro2::TokenStream {
    match scope {
        Scope::MultiProfileNoFlow | Scope::NoProfileNoFlow | Scope::SingleProfileNoFlow => {
            proc_macro2::TokenStream::new()
        }
        Scope::MultiProfileSingleFlow => {
            // * Reads previous item params and stores them in a `Map<Profile, ItemParams>`.
            // * Reads previously stored current states and stores them in a `Map<Profile,
            //   StatesCurrentStored>`.
            //
            // These are then held in the scope for easy access for consumers.
            quote! {
                let flow_id = flow.flow_id();
                let item_graph = flow.graph();

                let (params_specs_type_reg, states_type_reg) =
                    crate::ctx::cmd_ctx_builder::params_and_states_type_reg(item_graph);

                let params_specs_type_reg_ref = &params_specs_type_reg;
                let profile_to_params_specs = futures::stream::iter(
                    flow_dirs
                        .iter()
                        .map(Result::<_, peace_rt_model::Error>::Ok)
                    )
                    .and_then(|(profile, flow_dir)| {
                        let params_specs_provided = params_specs_provided.clone();
                        async move {
                            let params_specs_file =
                                peace_resources::paths::ParamsSpecsFile::from(flow_dir);

                            let params_specs_stored = peace_rt_model::ParamsSpecsSerializer::<
                                peace_rt_model::Error
                            >::deserialize_opt(
                                profile,
                                flow_id,
                                storage,
                                params_specs_type_reg_ref,
                                &params_specs_file,
                            )
                            .await?;

                            // For mapping fns, we still need the developer to provide the params spec
                            // so that multi-profile diffs can be done.
                            let params_specs = params_specs_stored.map(|params_specs_stored| {
                                crate::ctx::cmd_ctx_builder::params_specs_merge(
                                    &flow,
                                    params_specs_provided,
                                    Some(params_specs_stored),
                                )
                            })
                            .transpose()?;

                            // Note: we don't serialize params specs back to disk.

                            Ok((profile.clone(), params_specs))
                        }
                    })
                    .try_collect::<
                        std::collections::BTreeMap<
                            peace_core::Profile,
                            Option<peace_params::ParamsSpecs>
                        >
                    >()
                    .await?;

                let states_type_reg_ref = &states_type_reg;
                let profile_to_states_current_stored = futures::stream::iter(
                    flow_dirs
                        .iter()
                        .map(Result::<_, peace_rt_model::Error>::Ok)
                    )
                    .and_then(|(profile, flow_dir)| async move {
                        let states_current_file = peace_resources::paths::StatesCurrentFile::from(flow_dir);

                        let states_current_stored = peace_rt_model::StatesSerializer::<
                            peace_rt_model::Error
                        >::deserialize_stored_opt(
                            flow_id,
                            storage,
                            states_type_reg_ref,
                            &states_current_file,
                        )
                        .await?
                        .map(Into::<peace_resources::states::StatesCurrentStored>::into);

                        Ok((profile.clone(), states_current_stored))
                    })
                    .try_collect::<
                        std::collections::BTreeMap<
                            peace_core::Profile,
                            Option<peace_resources::states::StatesCurrentStored>
                        >
                    >()
                    .await?;

                // Call each `Item`'s initialization function.
                let resources = crate::ctx::cmd_ctx_builder::item_graph_setup(
                    item_graph,
                    resources
                )
                .await?;
            }
        }
        Scope::SingleProfileSingleFlow => {
            // Reads and inserts previously stored current states, and sets up resources
            // using the flow graph.
            //
            // It is not possible to insert stored current states into resources when
            // running a command with multiple flows, as the flows will have
            // different items and their state (type)s will be different.
            //
            // An example is workspace initialization, where the stored current states per
            // item for workspace initialization are likely different to application
            // specific flows.
            //
            // We currently don't support inserting resources for `MultiProfileSingleFlow`
            // commands. That would require either multiple `Resources` maps, or a
            // `Resources` map that contains `Map<Profile, _>`.
            //
            // It also requires multiple item graph setups to work without conflicting
            // with each other.
            quote! {
                let flow_id = flow.flow_id();
                let item_graph = flow.graph();

                let (params_specs_type_reg, states_type_reg) =
                    crate::ctx::cmd_ctx_builder::params_and_states_type_reg(item_graph);

                // Params specs loading and storage.
                let params_specs_type_reg_ref = &params_specs_type_reg;
                let params_specs_file = peace_resources::paths::ParamsSpecsFile::from(&flow_dir);
                let params_specs_stored = peace_rt_model::ParamsSpecsSerializer::<
                    peace_rt_model::Error
                >::deserialize_opt(
                    &profile,
                    flow_id,
                    storage,
                    params_specs_type_reg_ref,
                    &params_specs_file,
                )
                .await?;

                let params_specs = crate::ctx::cmd_ctx_builder::params_specs_merge(
                    &flow,
                    params_specs_provided,
                    params_specs_stored,
                )?;

                crate::ctx::cmd_ctx_builder::params_specs_serialize(
                    &params_specs,
                    storage,
                    &params_specs_file,
                )
                .await?;

                // States loading and storage.
                let states_type_reg_ref = &states_type_reg;
                let states_current_file = peace_resources::paths::StatesCurrentFile::from(&flow_dir);
                let states_current_stored = peace_rt_model::StatesSerializer::<
                    peace_rt_model::Error
                >::deserialize_stored_opt(
                    flow_id,
                    storage,
                    states_type_reg_ref,
                    &states_current_file,
                )
                .await?
                .map(Into::<peace_resources::states::StatesCurrentStored>::into);
                if let Some(states_current_stored) = states_current_stored {
                    resources.insert(states_current_stored);
                }

                // Call each `Item`'s initialization function.
                let resources = crate::ctx::cmd_ctx_builder::item_graph_setup(
                    item_graph,
                    resources
                )
                .await?;

                // output_progress CmdProgressTracker initialization
                #[cfg(feature = "output_progress")]
                let cmd_progress_tracker = {
                    let multi_progress = indicatif::MultiProgress::with_draw_target(
                        indicatif::ProgressDrawTarget::hidden()
                    );
                    let progress_trackers = item_graph.iter_insertion().fold(
                        peace_rt_model::IndexMap::with_capacity(item_graph.node_count()),
                        |mut progress_trackers, item| {
                            let progress_bar = multi_progress.add(indicatif::ProgressBar::hidden());
                            let progress_tracker = peace_core::progress::ProgressTracker::new(progress_bar);
                            progress_trackers.insert(item.id().clone(), progress_tracker);
                            progress_trackers
                        },
                    );

                    peace_rt_model::CmdProgressTracker::new(multi_progress, progress_trackers)
                };
            }
        }
    }
}

fn resources_insert(scope: Scope) -> proc_macro2::TokenStream {
    match scope {
        Scope::MultiProfileSingleFlow => {
            quote! {
                {
                    let (app_name, workspace_dirs, storage) = workspace.clone().into_inner();
                    let (workspace_dir, peace_dir, peace_app_dir) = workspace_dirs.into_inner();

                    resources.insert(app_name);
                    resources.insert(storage);
                    resources.insert(workspace_dir);
                    resources.insert(peace_dir);
                    resources.insert(peace_app_dir);
                    resources.insert(flow.flow_id().clone());
                }
            }
        }
        Scope::SingleProfileSingleFlow => {
            quote! {
                {
                    let (app_name, workspace_dirs, storage) = workspace.clone().into_inner();
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
            }
        }
        Scope::MultiProfileNoFlow | Scope::NoProfileNoFlow | Scope::SingleProfileNoFlow => {
            proc_macro2::TokenStream::new()
        }
    }
}
