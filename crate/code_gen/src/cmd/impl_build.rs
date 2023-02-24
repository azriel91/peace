use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, FieldValue, Pat, Path, Token};

use crate::cmd::{
    type_params_selection::{
        FlowIdSelection, FlowParamsSelection, ProfileParamsSelection, ProfileSelection,
        WorkspaceParamsSelection,
    },
    FlowCount, ParamsScope, ProfileCount, Scope, ScopeStruct,
};

/// Generates the `CmdCtxBuilder::build` methods for each type param selection.
///
/// For a command with `ProfileSelection`, `FlowIdSelection`, and
/// `*ParamsSelection`s type parameters, `2 * 1 * 2 * 2 * 2` = 16 variations of
/// the `build` method need to be generated, which is tedious to keep
/// consistently correct by hand:
///
/// * `ProfileSelected`, `ProfileFromWorkspaceParams`
/// * `FlowIdSelected`
/// * `WorkspaceParamsNone`, `WorkspaceParamsSome`
/// * `ProfileParamsNone`, `ProfileParamsSome`
/// * `FlowParamsNone`, `FlowParamsSome`
pub fn impl_build(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    ProfileSelection::iter().fold(
        proc_macro2::TokenStream::new(),
        |tokens, profile_selection| {
            match (profile_selection, scope_struct.scope().profile_count()) {
                // It doesn't make sense to have `NotSelected` or `FilterFunction`
                // when profile is single.
                (ProfileSelection::NotSelected | ProfileSelection::FilterFunction, ProfileCount::One) |
                // It doesn't make sense to have `profile_from_workpace_param`
                // when profile is none or multi.
                (
                    ProfileSelection::Selected | ProfileSelection::FromWorkspaceParam,
                    ProfileCount::None | ProfileCount::Multiple
                ) => return tokens,
                _ => {} // impl build
            }

            FlowIdSelection::iter().fold(tokens, |tokens, flow_id_selection| {
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
                                            flow_id_selection,
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
    flow_id_selection: FlowIdSelection,
    workspace_params_selection: WorkspaceParamsSelection,
    profile_params_selection: ProfileParamsSelection,
    flow_params_selection: FlowParamsSelection,
) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let scope_type_path = scope.type_path();
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    let scope_type_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        match scope.profile_count() {
            ProfileCount::None => {}
            ProfileCount::One | ProfileCount::Multiple => {
                type_params.push(profile_selection.type_param());
            }
        }
        if scope.flow_count() == FlowCount::One {
            type_params.push(flow_id_selection.type_param());
        }

        type_params.push(workspace_params_selection.type_param());
        if scope.profile_params_supported() {
            type_params.push(profile_params_selection.type_param());
        }
        if scope.flow_params_supported() {
            type_params.push(flow_params_selection.type_param());
        }

        type_params
    };

    let workspace_dirs_and_storage_borrow =
        workspace_dirs_and_storage_borrow(scope, workspace_params_selection);
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
    let scope_fields = {
        let mut scope_fields = Punctuated::<Pat, Token![,]>::new();

        match scope.profile_count() {
            ProfileCount::None => {
                scope_fields.push(parse_quote!(workspace_params));
            }
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
                    scope_fields.push(parse_quote!(flow_id));
                    scope_fields.push(parse_quote!(flow_dir));
                }
                ProfileCount::Multiple => {
                    scope_fields.push(parse_quote!(flow_id));
                    scope_fields.push(parse_quote!(flow_dirs));
                }
            },
        }

        // Cmd Params
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

        scope_fields
    };

    let scope_builder_deconstruct = scope_builder_deconstruct(
        scope_struct,
        scope,
        profile_selection,
        flow_id_selection,
        workspace_params_selection,
        profile_params_selection,
        flow_params_selection,
    );

    quote! {
        impl<'ctx, 'key, PKeys>
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    // ProfileFromWorkspaceParam<'key, <PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
                    // FlowIdSelected,
                    // WorkspaceParamsSome<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
                    // ProfileParamsSome<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
                    // FlowParamsNone,
                    #scope_type_params
                >,
                PKeys,
            >
        where
            PKeys: #params_module::ParamsKeys + 'static,
        {
            /// Builds the command context.
            ///
            /// This includes creating directories and deriving values based on the
            /// given parameters
            pub async fn build(
                mut self,
            ) -> Result<
                crate::ctx::CmdCtx<
                    'ctx,
                    #scope_type_path<PKeys>,
                    #params_module::ParamsKeysImpl<
                        PKeys::WorkspaceParamsKMaybe,
                        PKeys::ProfileParamsKMaybe,
                        PKeys::FlowParamsKMaybe,
                    >,
                >,
                peace_rt_model::Error,
            > {
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
                // let profile_s_ref = &profile;
                // let profile_s_ref = &self.scope_builder.profile_selection.0;
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
                //         indexmap::IndexMap::<
                //             peace_core::Profile,
                //             peace_resources::paths::ProfileDir
                //         >::with_capacity(profile_s_ref.len()),
                //         indexmap::IndexMap::<
                //             peace_core::Profile,
                //             peace_resources::paths::ProfileHistoryDir
                //         >::with_capacity(profile_s_ref.len())
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
                // let flow_dir = FlowDir::from((&profile_dir, &self.scope_builder.flow_id_selection.0));
                // --- Multi Profile Single Flow --- //
                // dirs_tokens.extend(quote! {
                //     let flow_dirs = profile_dirs
                //         .iter()
                //         .fold(indexmap::IndexMap::<
                //                 peace_core::Profile,
                //                 peace_resources::paths::ProfileDir
                //             >::with_capacity(profile_s_ref.len()
                //         ), |mut flow_dirs, (profile, profile_dir)| {
                //             let flow_dir = peace_resources::paths::FlowDir::from((profile_dir, &self.scope_builder.flow_id_selection.0));
                //
                //             flow_dirs.insert(profile.clone(), flow_dir);
                //
                //             flow_dirs
                //         });
                // });
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
                //         profile_dirs
                //             .iter()
                //             .map(Result::<_, peace_rt_model::Error>::Ok)
                //         )
                //         .and_then(|(profile, profile_dir)| async move {
                //             let profile_params_file =
                //                 peace_resources::internal::ProfileParamsFile::from(profile_dir);
                //
                //             let profile_params = self
                //                 .#params_deserialize_method_name(&profile_params_file)
                //                 .await?;
                //
                //             Ok((profile.clone(), profile_params))
                //         })
                //         .try_collect::<
                //             indexmap::IndexMap<
                //                 peace_core::Profile,
                //                 _ // peace_rt_model::cmd_context_params::ProfileParams<K>
                //             >
                //         >()
                //         .await?;
                #profile_params_deserialize

                // === Flow Params === //
                // --- Single --- //
                // let flow_params_file = ProfileParamsFile::from(&flow_dir);
                // self.flow_params_merge(&flow_params_file).await?;
                // --- Multi --- //
                // let profile_to_flow_params = futures::stream::iter(
                //         flow_dirs
                //             .iter()
                //             .map(Result::<_, peace_rt_model::Error>::Ok)
                //         )
                //         .and_then(|(profile, flow_dir)| async move {
                //             let flow_params_file =
                //                 peace_resources::internal::FlowParamsFile::from(flow_dir);
                //
                //             let flow_params = self
                //                 .#params_deserialize_method_name(&flow_params_file)
                //                 .await?;
                //
                //             Ok((profile.clone(), flow_params))
                //         })
                //         .try_collect::<
                //             indexmap::IndexMap<
                //                 peace_core::Profile,
                //                 _ // peace_rt_model::cmd_context_params::FlowParams<K>
                //             >
                //         >()
                //         .await?;
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
                //     workspace,
                //     scope_builder:
                //         #scope_builder_name {
                //             profile_selection: ProfileSelected(profile)
                //                             // ProfileFromWorkspaceParam(_workspace_params_k),
                //                             // ProfilesFilterFunction(profiles_filter_fn)
                //
                //             flow_id_selection: FlowIdSelected(flow_id),
                //             workspace_params_selection: WorkspaceParamsSome(workspace_params),
                //             profile_params_selection: ProfileParamsSome(profile_params),
                //             flow_params_selection: FlowParamsNone,
                //         },
                //     params_type_regs_builder,
                // } = self;
                #scope_builder_deconstruct

                // Serialize params to `PeaceAppDir`.

                // Self::workspace_params_serialize(
                //     &workspace_params,
                //     storage,
                //     &workspace_params_file,
                // )
                // .await?;
                #workspace_params_serialize

                // Self::profile_params_serialize(
                //     &profile_params,
                //     storage,
                //     &profile_params_file
                // )
                // .await?;
                #profile_params_serialize

                // Self::flow_params_serialize(
                //     &flow_params,
                //     storage,
                //     &flow_params_file
                // )
                // .await?;
                #flow_params_serialize

                // Track items in memory.
                let mut resources = peace_resources::Resources::new();
                // Self::workspace_params_insert(workspace_params, &mut resources);
                #workspace_params_insert
                // === Single Profile === //
                // Self::profile_params_insert(profile_params, &mut resources);
                #profile_params_insert
                // === Single Flow === //
                // Self::flow_params_insert(flow_params, &mut resources);
                #flow_params_insert

                let scope = #scope_type_path::new(
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
                    // flow_id,
                    // flow_dir,
                    // flow_params,
                    // --- Multi --- //
                    // flow_id,
                    // flow_dirs,
                    // profile_to_flow_params,

                    #scope_fields
                );

                let params_type_regs = params_type_regs_builder.build();

                Ok(crate::ctx::CmdCtx {
                    workspace,
                    scope,
                    params_type_regs,
                })
            }
        }
    }
}

fn scope_builder_deconstruct(
    scope_struct: &ScopeStruct,
    scope: Scope,
    profile_selection: ProfileSelection,
    flow_id_selection: FlowIdSelection,
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
                    crate::scopes::type_params::ProfilesFilterFunction(profiles_filter_fn)
            }),
        },
    }

    if scope.flow_count() == FlowCount::One {
        match flow_id_selection {
            FlowIdSelection::Selected => scope_builder_fields.push(parse_quote! {
                flow_id_selection: crate::scopes::type_params::FlowIdSelected(flow_id)
            }),
        }
    }

    scope_builder_fields.push(workspace_params_selection.deconstruct());
    if scope.profile_params_supported() {
        scope_builder_fields.push(profile_params_selection.deconstruct());
    }
    if scope.flow_params_supported() {
        scope_builder_fields.push(flow_params_selection.deconstruct());
    }

    quote! {
        let crate::ctx::CmdCtxBuilder {
            workspace,
            scope_builder:
                #scope_builder_name {
                    // profile_selection: ProfileSelected(profile),
                    // flow_id_selection: FlowIdSelected(flow_id),
                    // workspace_params_selection: WorkspaceParamsSome(workspace_params),
                    // profile_params_selection: ProfileParamsSome(profile_params),
                    // flow_params_selection: FlowParamsNone,
                    #scope_builder_fields,
                },
            params_type_regs_builder,
        } = self;
    }
}

/// Borrow `workspace_dirs` when either:
///
/// * there is at least one profile
/// * there are workspace params
fn workspace_dirs_and_storage_borrow(
    scope: Scope,
    workspace_params_selection: WorkspaceParamsSelection,
) -> proc_macro2::TokenStream {
    if scope.profile_count() != ProfileCount::None
        || workspace_params_selection == WorkspaceParamsSelection::Some
    {
        quote! {
            let workspace_dirs = self.workspace.dirs();
            let storage = self.workspace.storage();
        }
    } else {
        proc_macro2::TokenStream::new()
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
    if workspace_params_selection == WorkspaceParamsSelection::Some {
        let workspace_params_deserialize = quote! {
            let workspace_params_file = peace_resources::internal::WorkspaceParamsFile::from(
                workspace_dirs.peace_app_dir()
            );

            self.workspace_params_merge(&workspace_params_file).await?;
        };
        let workspace_params_serialize = quote! {
            Self::workspace_params_serialize(
                &workspace_params,
                storage,
                &workspace_params_file,
            )
            .await?;
        };
        let workspace_params_insert = quote! {
            Self::workspace_params_insert(workspace_params.clone(), &mut resources);
        };

        (
            workspace_params_deserialize,
            workspace_params_serialize,
            workspace_params_insert,
        )
    } else {
        let workspace_params_deserialize = quote! {
            let workspace_params = peace_rt_model::cmd_context_params::WorkspaceParams::new();
        };

        (
            workspace_params_deserialize,
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
        )
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
        ProfileCount::One => {
            if profile_params_selection == ProfileParamsSelection::Some {
                let profile_params_deserialize = quote! {
                    let profile_params_file = peace_resources::internal::ProfileParamsFile::from(
                        &profile_dir
                    );

                    self.profile_params_merge(&profile_params_file).await?;
                };
                let profile_params_serialize = quote! {
                    Self::profile_params_serialize(
                        &profile_params,
                        storage,
                        &profile_params_file,
                    )
                    .await?;
                };
                let profile_params_insert = quote! {
                    Self::profile_params_insert(profile_params.clone(), &mut resources);
                };

                (
                    profile_params_deserialize,
                    profile_params_serialize,
                    profile_params_insert,
                )
            } else {
                (
                    proc_macro2::TokenStream::new(),
                    proc_macro2::TokenStream::new(),
                    proc_macro2::TokenStream::new(),
                )
            }
        }
        ProfileCount::Multiple => {
            let profile_params_deserialize = match profile_params_selection {
                ProfileParamsSelection::None => quote! {
                    let profile_to_profile_params = indexmap::IndexMap::<
                        peace_core::Profile,
                        peace_rt_model::cmd_context_params::ProfileParams<_>
                    >::new();
                },
                ProfileParamsSelection::Some => {
                    let params_deserialize_method_name =
                        ParamsScope::Profile.params_deserialize_method_name();

                    quote! {
                        let storage = self.workspace.storage();
                        let params_type_regs_builder = &self.params_type_regs_builder;
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
                                indexmap::IndexMap<
                                    peace_core::Profile,
                                    peace_rt_model::cmd_context_params::ProfileParams<_>
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
            FlowParamsSelection::None => (
                proc_macro2::TokenStream::new(),
                proc_macro2::TokenStream::new(),
                proc_macro2::TokenStream::new(),
            ),
            FlowParamsSelection::Some => {
                let flow_params_deserialize = quote! {
                    let flow_params_file = peace_resources::internal::FlowParamsFile::from(
                        &flow_dir
                    );

                    self.flow_params_merge(&flow_params_file).await?;
                };
                let flow_params_serialize = quote! {
                    Self::flow_params_serialize(
                        &flow_params,
                        storage,
                        &flow_params_file,
                    )
                    .await?;
                };
                let flow_params_insert = quote! {
                    Self::flow_params_insert(flow_params.clone(), &mut resources);
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
                    let profile_to_flow_params = indexmap::IndexMap::<
                        peace_core::Profile,
                        peace_rt_model::cmd_context_params::FlowParams<_>
                    >::new();
                },
                FlowParamsSelection::Some => {
                    let params_deserialize_method_name =
                        ParamsScope::Flow.params_deserialize_method_name();
                    quote! {
                        let storage = self.workspace.storage();
                        let params_type_regs_builder = &self.params_type_regs_builder;
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
                                indexmap::IndexMap<
                                    peace_core::Profile,
                                    peace_rt_model::cmd_context_params::FlowParams<_>
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
                        indexmap::IndexMap::<
                            peace_core::Profile,
                            peace_resources::paths::ProfileDir
                        >::with_capacity(profile_s_ref.len()),
                        indexmap::IndexMap::<
                            peace_core::Profile,
                            peace_resources::paths::ProfileHistoryDir
                        >::with_capacity(profile_s_ref.len())
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
                    let flow_dir = peace_resources::paths::FlowDir::from((&profile_dir, &self.scope_builder.flow_id_selection.0));
                });
            }
            ProfileCount::Multiple => {
                dirs_tokens.extend(quote! {
                    let flow_dirs = profile_dirs
                        .iter()
                        .fold(indexmap::IndexMap::<
                                peace_core::Profile,
                                peace_resources::paths::FlowDir
                            >::with_capacity(profile_s_ref.len()
                        ), |mut flow_dirs, (profile, profile_dir)| {
                            let flow_dir = peace_resources::paths::FlowDir::from((profile_dir, &self.scope_builder.flow_id_selection.0));

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
