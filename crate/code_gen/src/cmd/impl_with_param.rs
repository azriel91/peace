use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, FieldValue, Pat, Path, Token, WherePredicate,
};

use crate::cmd::{type_parameters_impl, ParamsScope, Scope, ScopeStruct};

/// Generates the `with_workspace_param` / `with_profile_param` /
/// `with_flow_param` methods.
pub fn impl_with_param(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    ParamsScope::iter().fold(
        proc_macro2::TokenStream::new(),
        |mut impl_tokens, params_scope| {
            let impl_with_param_key_unknown =
                impl_with_param_key_unknown(scope_struct, params_scope);

            let impl_with_param_key_known = impl_with_param_key_known(scope_struct, params_scope);

            impl_tokens.extend(impl_with_param_key_unknown);
            impl_tokens.extend(impl_with_param_key_known);

            impl_tokens
        },
    )
}

fn impl_with_param_key_unknown(
    scope_struct: &ScopeStruct,
    params_scope: ParamsScope,
) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    // ProfileSelection, FlowIdSelection
    let selection_type_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);
        type_params
    };

    let impl_type_params = {
        let mut type_params = selection_type_params.clone();
        type_parameters_impl::params_selection_maybe_push(
            &mut type_params,
            scope,
            params_scope,
            false,
        );
        type_params
    };

    let impl_scope_type_params_none = {
        let mut type_params = selection_type_params.clone();
        type_parameters_impl::params_selection_none_push(&mut type_params, scope, params_scope);
        type_params
    };

    let impl_scope_type_params_some = {
        let mut type_params = selection_type_params;
        type_parameters_impl::params_selection_some_push(&mut type_params, scope, params_scope);
        type_params
    };

    let impl_params_key_unknown_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        type_parameters_impl::params_key_unknown_push(&mut type_params, scope, params_scope);
        type_params
    };

    let impl_params_key_known_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        type_parameters_impl::params_key_known_push(&mut type_params, scope, params_scope);
        type_params
    };

    let param_key_impl_unknown_predicates = param_key_impl_unknown_predicates(scope, params_scope);

    let params_scope_str = params_scope.to_str();
    let with_params_method_name = params_scope.param_method_name();
    let params_map_type = params_scope.params_map_type();
    let param_type_param = params_scope.param_type_param();
    let params_k_method_name = params_scope.params_k_method_name();
    let params_k_type_param = params_scope.params_k_type_param();
    let param_name = params_scope.param_name();
    let param_name_str = param_name.to_string();
    let scope_builder_fields_params_none = scope_builder_fields_params_none(scope, params_scope);
    let scope_builder_fields_params_some_new =
        scope_builder_fields_params_some_new(scope, params_scope);
    let params_type_reg_method_name = params_scope.params_type_reg_method_name();

    let doc_summary = format!("Adds a {params_scope_str} parameter.");
    let doc_param = format!("* `{param_name_str}`: The {params_scope_str} parameter to register.");

    quote! {
        impl<
            'ctx,
            // ProfileSelection,
            // FlowIdSelection,
            // ProfileParamsSelection,
            // ProfileParamsKMaybe,
            // FlowParamsKMaybe,

            #impl_type_params
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    // ProfileSelection,
                    // FlowIdSelection,
                    // WorkspaceParamsNone,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,

                    #impl_scope_type_params_none
                >,
                #params_module::ParamsKeysImpl<
                    // KeyUnknown, ProfileParamsKMaybe, FlowParamsKMaybe
                    #impl_params_key_unknown_params
                >,
            >
        where
            // ProfileParamsKMaybe: KeyMaybe,
            // FlowParamsKMaybe: KeyMaybe,
            #param_key_impl_unknown_predicates
        {
            #[doc = #doc_summary]
            ///
            /// Currently there is no means in code to deliberately unset any previously
            /// stored value. This can be made possibly by defining a `*ParamsBuilder`
            /// that determines a `None` value as a deliberate erasure of any previous
            /// value.
            ///
            /// # Parameters
            ///
            /// * `k`: Key to store the parameter with.
            #[doc = #doc_param]
            // pub fn with_workspace_params<WorkspaceParamsK, WorkspaceParam>
            pub fn #with_params_method_name<#params_k_type_param, #param_type_param>(
                self,
                k: #params_k_type_param,
                #param_name: Option<#param_type_param>,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    // ProfileSelection,
                    // FlowIdSelection,
                    // WorkspaceParamsSome<#params_k_type_param>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,

                    #impl_scope_type_params_some
                >,
                #params_module::ParamsKeysImpl<
                    // KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    #impl_params_key_known_params
                >,
            >
            where
                #params_k_type_param:
                    Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
                #param_type_param: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
            {
                let Self {
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection,
                            // flow_id_selection,
                            // workspace_params_selection: WorkspaceParamsNone,
                            // profile_params_selection,
                            // flow_params_selection,

                            #scope_builder_fields_params_none
                        },
                    params_type_regs_builder,
                } = self;

                // let mut params_type_regs_builder =
                //     params_type_regs_builder.with_workspace_params_k::<WorkspaceParamsK>();
                let mut params_type_regs_builder =
                    params_type_regs_builder.#params_k_method_name::<#params_k_type_param>();
                params_type_regs_builder
                    .#params_type_reg_method_name()
                    .register::<#param_type_param>(k.clone());
                // let mut workspace_params = WorkspaceParams::<WorkspaceParam>::new();
                let mut params_map = #params_map_type::<#params_k_type_param>::new();
                if let Some(#param_name) = #param_name {
                    params_map.insert(k, #param_name);
                }

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_id_selection,
                    // workspace_params_selection: WorkspaceParamsSome(Some(params_map)),
                    // profile_params_selection,
                    // flow_params_selection,

                    #scope_builder_fields_params_some_new
                };

                crate::ctx::CmdCtxBuilder {
                    workspace,
                    scope_builder,
                    params_type_regs_builder,
                }
            }
        }
    }
}

fn impl_with_param_key_known(
    scope_struct: &ScopeStruct,
    params_scope: ParamsScope,
) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    // ProfileSelection, FlowIdSelection
    let selection_type_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);
        type_params
    };

    let impl_type_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();

        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);
        type_parameters_impl::params_selection_maybe_push(
            &mut type_params,
            scope,
            params_scope,
            true,
        );

        type_params
    };

    let impl_scope_type_params_some = {
        let mut type_params = selection_type_params;
        type_parameters_impl::params_selection_some_push(&mut type_params, scope, params_scope);
        type_params
    };

    let impl_params_key_known_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        type_parameters_impl::params_key_known_push(&mut type_params, scope, params_scope);
        type_params
    };

    let param_key_impl_known_predicates = param_key_impl_known_predicates(scope, params_scope);

    let params_scope_str = params_scope.to_str();
    let with_params_method_name = params_scope.param_method_name();
    let param_type_param = params_scope.param_type_param();
    let params_k_type_param = params_scope.params_k_type_param();
    let param_name = params_scope.param_name();
    let param_name_str = param_name.to_string();
    let params_selection_name = params_scope.params_selection_name();
    let scope_builder_fields_params_some = scope_builder_fields_params_some(scope, params_scope);
    let scope_builder_fields = scope_builder_fields(scope, params_scope);
    let params_type_reg_method_name = params_scope.params_type_reg_method_name();

    let doc_summary = format!("Adds a {params_scope_str} parameter.");
    let doc_param = format!("* `{param_name_str}`: The {params_scope_str} parameter to register.");

    quote! {
        impl<
            'ctx,
            // ProfileSelection,
            // FlowIdSelection,
            // ProfileParamsSelection,
            // WorkspaceParamsK,
            // ProfileParamsKMaybe,
            // FlowParamsKMaybe,

            #impl_type_params
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    // ProfileSelection,
                    // FlowIdSelection,
                    // WorkspaceParamsSome<WorkspaceParamsK>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,

                    #impl_scope_type_params_some
                >,
                #params_module::ParamsKeysImpl<
                    // KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    #impl_params_key_known_params
                >,
            >
        where
            // WorkspaceParamsK:
            //     Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
            // ProfileParamsKMaybe: KeyMaybe,
            // FlowParamsKMaybe: KeyMaybe,

            #param_key_impl_known_predicates
        {
            #[doc = #doc_summary]
            ///
            /// Currently there is no means in code to deliberately unset any previously
            /// stored value. This can be made possibly by defining a `*ParamsBuilder`
            /// that determines a `None` value as a deliberate erasure of any previous
            /// value.
            ///
            /// # Parameters
            ///
            /// * `k`: Key to store the parameter with.
            #[doc = #doc_param]
            // pub fn with_workspace_params<WorkspaceParam>
            pub fn #with_params_method_name<#param_type_param>(
                self,
                k: #params_k_type_param,
                #param_name: Option<#param_type_param>,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    // ProfileSelection,
                    // FlowIdSelection,
                    // WorkspaceParamsSome<WorkspaceParamsK>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,

                    #impl_scope_type_params_some
                >,
                #params_module::ParamsKeysImpl<
                    // KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    #impl_params_key_known_params
                >,
            >
            where
                #param_type_param: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
            {
                let Self {
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection,
                            // flow_id_selection,
                            // mut workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,

                            #scope_builder_fields_params_some
                        },
                    mut params_type_regs_builder,
                } = self;

                params_type_regs_builder
                    .#params_type_reg_method_name()
                    .register::<#param_type_param>(k.clone());
                // let Some(workspace_params) = workspace_params_selection.0.as_mut() else {
                let Some(params_map) = #params_selection_name.0.as_mut() else {
                    unreachable!("This is set to `Some` in `Self::{}`", stringify!(#with_params_method_name));
                };
                if let Some(#param_name) = #param_name {
                    params_map.insert(k, #param_name);
                }

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_id_selection,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,

                    #scope_builder_fields
                };

                crate::ctx::CmdCtxBuilder {
                    workspace,
                    scope_builder,
                    params_type_regs_builder,
                }
            }
        }
    }
}

fn param_key_impl_unknown_predicates(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<WherePredicate, Comma> {
    let mut predicates = Punctuated::<WherePredicate, Token![,]>::new();
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    match params_scope {
        ParamsScope::Workspace => {
            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsKMaybe: #params_module::KeyMaybe
                });
            }

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            predicates.push(parse_quote! {
                WorkspaceParamsKMaybe: #params_module::KeyMaybe
            });

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            predicates.push(parse_quote! {
                WorkspaceParamsKMaybe: #params_module::KeyMaybe
            });

            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
    }

    predicates
}

fn param_key_impl_known_predicates(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<WherePredicate, Comma> {
    let mut predicates = Punctuated::<WherePredicate, Token![,]>::new();
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    match params_scope {
        ParamsScope::Workspace => {
            predicates.push(parse_quote! {
                WorkspaceParamsK:
                    Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static
            });

            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsKMaybe: #params_module::KeyMaybe
                });
            }

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            predicates.push(parse_quote! {
                WorkspaceParamsKMaybe: #params_module::KeyMaybe
            });

            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsK:
                        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static
                });
            }

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            predicates.push(parse_quote! {
                WorkspaceParamsKMaybe: #params_module::KeyMaybe
            });

            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsKMaybe: #params_module::KeyMaybe
                });
            }

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsK:
                        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static
                });
            }
        }
    }

    predicates
}

fn scope_builder_fields(scope: Scope, params_scope: ParamsScope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(profile_selection));
    field_values.push(parse_quote!(flow_id_selection));

    match params_scope {
        ParamsScope::Workspace => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Profile => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Flow => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
    }

    field_values
}

fn scope_builder_fields_params_none(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(profile_selection));
    field_values.push(parse_quote!(flow_id_selection));

    match params_scope {
        ParamsScope::Workspace => {
            field_values.push(parse_quote!(
                workspace_params_selection: WorkspaceParamsNone
            ));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Profile => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection: ProfileParamsNone));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Flow => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection: FlowParamsNone));
            }
        }
    }

    field_values
}

fn scope_builder_fields_params_some(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<Pat, Comma> {
    let mut field_values = Punctuated::<Pat, Token![,]>::new();
    field_values.push(parse_quote!(profile_selection));
    field_values.push(parse_quote!(flow_id_selection));

    match params_scope {
        ParamsScope::Workspace => {
            field_values.push(parse_quote!(mut workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Profile => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(mut profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Flow => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(mut flow_params_selection));
            }
        }
    }

    field_values
}

fn scope_builder_fields_params_some_new(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(profile_selection));
    field_values.push(parse_quote!(flow_id_selection));

    match params_scope {
        ParamsScope::Workspace => {
            field_values.push(parse_quote! {
                workspace_params_selection:
                    crate::ctx::cmd_ctx_builder::workspace_params_selection::WorkspaceParamsSome(
                        Some(params_map)
                    )
            });
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Profile => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote! {
                    profile_params_selection:
                        crate::ctx::cmd_ctx_builder::profile_params_selection::ProfileParamsSome(
                            Some(params_map)
                        )
                });
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Flow => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote! {
                    flow_params_selection:
                        crate::ctx::cmd_ctx_builder::flow_params_selection::FlowParamsSome(
                            Some(params_map),
                        )
                });
            }
        }
    }

    field_values
}
