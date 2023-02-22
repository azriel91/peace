use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, FieldValue, Pat, Path, Token};

use crate::cmd::{
    type_params_selection::{
        FlowIdSelection, FlowParamsSelection, ProfileParamsSelection, ProfileSelection,
        WorkspaceParamsSelection,
    },
    FlowCount, ProfileCount, Scope, ScopeStruct,
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
        if scope.profile_count() == ProfileCount::One {
            type_params.push(profile_selection.type_param());
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
    let profile_ref = profile_ref(profile_selection);
    let cmd_dirs = cmd_dirs(scope, &profile_ref);
    let dirs_to_create = dirs_to_create(scope);
    let scope_fields = {
        let mut scope_fields = Punctuated::<Pat, Token![,]>::new();

        if scope.profile_count() == ProfileCount::One {
            scope_fields.push(parse_quote!(profile));
            scope_fields.push(parse_quote!(profile_dir));
            scope_fields.push(parse_quote!(profile_history_dir));
        }

        if scope.flow_count() == FlowCount::One {
            scope_fields.push(parse_quote!(flow_id));
            scope_fields.push(parse_quote!(flow_dir));
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
                    #scope_type_path,
                    #params_module::ParamsKeysImpl<
                        PKeys::WorkspaceParamsKMaybe,
                        PKeys::ProfileParamsKMaybe,
                        PKeys::FlowParamsKMaybe,
                    >,
                >,
                peace_rt_model::Error,
            > {
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
                //     .as_ref()
                //     .ok_or(Error::WorkspaceParamsNoneForProfile)?
                //     .get(self.scope_builder.profile_selection.0)
                //     .cloned()
                //     .ok_or(Error::WorkspaceParamsProfileNone)?;
                #profile_from_workspace

                // let profile_dir = ProfileDir::from((workspace_dirs.peace_app_dir(), #profile_ref));
                // let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
                // let flow_dir = FlowDir::from((&profile_dir, &self.scope_builder.flow_id_selection.0));
                #cmd_dirs

                let dirs_to_create = [
                    // AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
                    // AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
                    // AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),
                    // AsRef::<std::path::Path>::as_ref(&profile_dir),
                    // AsRef::<std::path::Path>::as_ref(&profile_history_dir),
                    // AsRef::<std::path::Path>::as_ref(&flow_dir),
                    #dirs_to_create
                ];

                // let profile_params_file = ProfileParamsFile::from(&profile_dir);
                // self.profile_params_merge(&profile_params_file).await?;
                #profile_params_deserialize

                // let flow_params_file = ProfileParamsFile::from(&flow_dir);
                // self.flow_params_merge(&flow_params_file).await?;
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
                //             profile_selection: ProfileFromWorkspaceParam(_workspace_params_k),
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
                //     workspace_params.as_ref(),
                //     storage,
                //     &workspace_params_file,
                // )
                // .await?;
                #workspace_params_serialize

                // Self::profile_params_serialize(
                //     profile_params.as_ref(),
                //     storage,
                //     &profile_params_file
                // )
                // .await?;
                #profile_params_serialize

                // Self::flow_params_serialize(
                //     flow_params.as_ref(),
                //     storage,
                //     &flow_params_file
                // )
                // .await?;
                #flow_params_serialize

                // Track items in memory.
                let mut resources = peace_resources::Resources::new();
                // Self::workspace_params_insert(workspace_params, &mut resources);
                #workspace_params_insert
                // Self::profile_params_insert(profile_params, &mut resources);
                #profile_params_insert
                // Self::flow_params_insert(profile_params, &mut resources);
                #flow_params_insert

                let scope = #scope_type_path::new(
                    // profile,
                    // profile_dir,
                    // profile_history_dir,
                    // flow_id,
                    // flow_dir,
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

    if scope.profile_count() == ProfileCount::One {
        match profile_selection {
            ProfileSelection::Selected => scope_builder_fields.push(parse_quote! {
                profile_selection:
                    crate::ctx::cmd_ctx_builder::ProfileSelected(profile)
            }),
            ProfileSelection::FromWorkspaceParam => scope_builder_fields.push(parse_quote! {
                profile_selection:
                    crate::ctx::cmd_ctx_builder::ProfileFromWorkspaceParam(
                        _workspace_params_k
                    )
            }),
        }
    }

    if scope.flow_count() == FlowCount::One {
        match flow_id_selection {
            FlowIdSelection::Selected => scope_builder_fields.push(parse_quote! {
                flow_id_selection: crate::ctx::cmd_ctx_builder::FlowIdSelected(flow_id)
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
                workspace_params.as_ref(),
                storage,
                &workspace_params_file,
            )
            .await?;
        };
        let workspace_params_insert = quote! {
            Self::workspace_params_insert(workspace_params, &mut resources);
        };

        (
            workspace_params_deserialize,
            workspace_params_serialize,
            workspace_params_insert,
        )
    } else {
        (
            proc_macro2::TokenStream::new(),
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
    if scope.profile_params_supported() && profile_params_selection == ProfileParamsSelection::Some
    {
        let profile_params_deserialize = quote! {
            let profile_params_file = peace_resources::internal::ProfileParamsFile::from(
                &profile_dir
            );

            self.profile_params_merge(&profile_params_file).await?;
        };
        let profile_params_serialize = quote! {
            Self::profile_params_serialize(
                profile_params.as_ref(),
                storage,
                &profile_params_file,
            )
            .await?;
        };
        let profile_params_insert = quote! {
            Self::profile_params_insert(profile_params, &mut resources);
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
    if scope.flow_params_supported() && flow_params_selection == FlowParamsSelection::Some {
        let flow_params_deserialize = quote! {
            let flow_params_file = peace_resources::internal::FlowParamsFile::from(
                &flow_dir
            );

            self.flow_params_merge(&flow_params_file).await?;
        };
        let flow_params_serialize = quote! {
            Self::flow_params_serialize(
                flow_params.as_ref(),
                storage,
                &flow_params_file,
            )
            .await?;
        };
        let flow_params_insert = quote! {
            Self::flow_params_insert(flow_params, &mut resources);
        };

        (
            flow_params_deserialize,
            flow_params_serialize,
            flow_params_insert,
        )
    } else {
        (
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
        )
    }
}

fn profile_from_workspace(profile_selection: ProfileSelection) -> proc_macro2::TokenStream {
    if profile_selection == ProfileSelection::FromWorkspaceParam {
        quote! {
            let profile = self
                .scope_builder
                .workspace_params_selection
                .0
                .as_ref()
                .ok_or(peace_rt_model::Error::WorkspaceParamsNoneForProfile)?
                .get(self.scope_builder.profile_selection.0)
                .cloned()
                .ok_or(peace_rt_model::Error::WorkspaceParamsProfileNone)?;
        }
    } else {
        proc_macro2::TokenStream::new()
    }
}

fn profile_ref(profile_selection: ProfileSelection) -> proc_macro2::TokenStream {
    if profile_selection == ProfileSelection::FromWorkspaceParam {
        quote!(&profile)
    } else {
        quote!(&self.scope_builder.profile_selection.0)
    }
}

fn cmd_dirs(scope: Scope, profile_ref: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut dirs_tokens = proc_macro2::TokenStream::new();

    if scope.profile_count() == ProfileCount::One {
        dirs_tokens.extend(quote! {
            let profile_dir = peace_resources::paths::ProfileDir::from((workspace_dirs.peace_app_dir(), #profile_ref));
            let profile_history_dir = peace_resources::paths::ProfileHistoryDir::from(&profile_dir);
        });
    }

    if scope.flow_count() == FlowCount::One {
        dirs_tokens.extend(quote! {
            let flow_dir = peace_resources::paths::FlowDir::from((&profile_dir, &self.scope_builder.flow_id_selection.0));
        });
    }

    dirs_tokens
}

fn dirs_to_create(scope: Scope) -> proc_macro2::TokenStream {
    let mut dirs_tokens = quote! {
        AsRef::<std::path::Path>::as_ref(workspace_dirs.workspace_dir()),
        AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_dir()),
        AsRef::<std::path::Path>::as_ref(workspace_dirs.peace_app_dir()),
    };

    if scope.profile_count() == ProfileCount::One {
        dirs_tokens.extend(quote! {
            AsRef::<std::path::Path>::as_ref(&profile_dir),
            AsRef::<std::path::Path>::as_ref(&profile_history_dir),
        });
    }

    if scope.flow_count() == FlowCount::One {
        dirs_tokens.extend(quote! {
            AsRef::<std::path::Path>::as_ref(&flow_dir),
        });
    }

    dirs_tokens
}
