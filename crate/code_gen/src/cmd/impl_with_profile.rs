use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, token::Comma, FieldValue, Path, Token};

use crate::cmd::{
    param_key_impl, type_parameters_impl, FlowCount, ParamsScope, Scope, ScopeStruct,
};

/// Generates the `with_profile` method for the command context builder.
pub fn impl_with_profile(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    let scope_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        if scope.flow_count() == FlowCount::One {
            type_params.push(parse_quote!(FlowIdSelection));
        }
        type_parameters_impl::params_selection_push(&mut type_params, scope);
        type_params
    };

    let scope_builder_fields_profile_not_selected =
        scope_builder_fields_profile_not_selected(scope);
    let scope_builder_fields_profile_selected = scope_builder_fields_profile_selected(scope);

    let mut tokens = quote! {
        impl<
            'ctx,
            // FlowIdSelection,
            // WorkspaceParamsSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            #scope_params,
            PKeys,
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    crate::ctx::cmd_ctx_builder::profile_selection::ProfileNotSelected,
                    // FlowIdSelection,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params
                >,
                PKeys,
            >
        where
            PKeys: #params_module::ParamsKeys + 'static,
        {
            pub fn with_profile(
                self,
                profile: peace_core::Profile,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    crate::ctx::cmd_ctx_builder::profile_selection::ProfileSelected,
                    // FlowIdSelection,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params
                >,
                PKeys,
            > {
                let Self {
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection: ProfileNotSelected,
                            // flow_id_selection,
                            // workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,
                            #scope_builder_fields_profile_not_selected
                        },
                    params_type_regs_builder,
                } = self;

                let scope_builder = #scope_builder_name {
                    // profile_selection: ProfileSelected(profile),
                    // flow_id_selection,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,
                    #scope_builder_fields_profile_selected
                };

                crate::ctx::CmdCtxBuilder {
                    workspace,
                    scope_builder,
                    params_type_regs_builder,
                }
            }
        }
    };

    tokens.extend(impl_with_profile_from_workspace_param(scope_struct));

    tokens
}

/// Generates the `with_profile` method for the command context builder.
pub fn impl_with_profile_from_workspace_param(
    scope_struct: &ScopeStruct,
) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    let impl_params_with_workspace_params_k = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        if scope.flow_count() == FlowCount::One {
            type_params.push(parse_quote!(FlowIdSelection));
        }
        if scope.profile_params_supported() {
            type_params.push(parse_quote!(ProfileParamsSelection));
        }
        if scope.flow_params_supported() {
            type_params.push(parse_quote!(FlowParamsSelection));
        }

        type_params.push(parse_quote!(WorkspaceParamsK));

        if scope.profile_params_supported() {
            type_params.push(parse_quote!(ProfileParamsKMaybe));
        }
        if scope.flow_params_supported() {
            type_params.push(parse_quote!(FlowParamsKMaybe));
        }
        type_params
    };

    let scope_params_with_workspace_params_k = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        if scope.flow_count() == FlowCount::One {
            type_params.push(parse_quote!(FlowIdSelection));
        }
        type_params.push(parse_quote! {
            crate::ctx::cmd_ctx_builder::workspace_params_selection::WorkspaceParamsSome<WorkspaceParamsK>
        });
        if scope.profile_params_supported() {
            type_params.push(parse_quote!(ProfileParamsSelection));
        }
        if scope.flow_params_supported() {
            type_params.push(parse_quote!(FlowParamsSelection));
        }
        type_params
    };

    let impl_params_key_known_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        type_parameters_impl::params_key_known_push(
            &mut type_params,
            scope,
            ParamsScope::Workspace,
        );
        type_params
    };
    let param_key_impl_known_predicates =
        param_key_impl::known_predicates(scope, ParamsScope::Workspace);

    let scope_builder_fields_profile_not_selected =
        scope_builder_fields_profile_not_selected(scope);
    let scope_builder_fields_profile_from_workspace =
        scope_builder_fields_profile_from_workspace(scope);

    quote! {
        impl<
            'ctx,
            // FlowIdSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            // WorkspaceParamsK,
            // ProfileParamsKMaybe,
            // FlowParamsKMaybe,
            #impl_params_with_workspace_params_k
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    crate::ctx::cmd_ctx_builder::profile_selection::ProfileNotSelected,
                    // FlowIdSelection,
                    // WorkspaceParamsSome<WorkspaceParamsK>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params_with_workspace_params_k
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
            pub fn with_profile_from_workspace_param<'key>(
                self,
                workspace_param_k: &'key WorkspaceParamsK,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    crate::ctx::cmd_ctx_builder::profile_selection::ProfileFromWorkspaceParam<'key, WorkspaceParamsK>,
                    // FlowIdSelection,
                    // WorkspaceParamsSome<WorkspaceParamsK>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params_with_workspace_params_k
                >,
                #params_module::ParamsKeysImpl<
                    // KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    #impl_params_key_known_params
                >,
            > {
                let Self {
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection: ProfileNotSelected,
                            // flow_id_selection,
                            // workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,
                            #scope_builder_fields_profile_not_selected
                        },
                    params_type_regs_builder,
                } = self;

                let scope_builder = #scope_builder_name {
                    // profile_selection: ProfileFromWorkspaceParam(workspace_param_k),
                    // flow_id_selection,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,
                    #scope_builder_fields_profile_from_workspace
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

fn scope_builder_fields_profile_not_selected(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(
        profile_selection: crate::ctx::cmd_ctx_builder::profile_selection::ProfileNotSelected
    ));
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_id_selection));
    }
    field_values.push(parse_quote!(workspace_params_selection));
    if scope.profile_params_supported() {
        field_values.push(parse_quote!(profile_params_selection));
    }
    if scope.flow_params_supported() {
        field_values.push(parse_quote!(flow_params_selection));
    }

    field_values
}

fn scope_builder_fields_profile_selected(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(
        profile_selection: crate::ctx::cmd_ctx_builder::profile_selection::ProfileSelected(profile)
    ));
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_id_selection));
    }
    field_values.push(parse_quote!(workspace_params_selection));
    if scope.profile_params_supported() {
        field_values.push(parse_quote!(profile_params_selection));
    }
    if scope.flow_params_supported() {
        field_values.push(parse_quote!(flow_params_selection));
    }

    field_values
}

fn scope_builder_fields_profile_from_workspace(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(
        profile_selection:
            crate::ctx::cmd_ctx_builder::profile_selection::ProfileFromWorkspaceParam(
                workspace_param_k,
            )
    ));
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_id_selection));
    }
    field_values.push(parse_quote!(workspace_params_selection));
    if scope.profile_params_supported() {
        field_values.push(parse_quote!(profile_params_selection));
    }
    if scope.flow_params_supported() {
        field_values.push(parse_quote!(flow_params_selection));
    }

    field_values
}
