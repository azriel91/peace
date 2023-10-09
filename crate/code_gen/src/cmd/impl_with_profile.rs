use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, FieldValue, GenericArgument, Path, Token,
};

use crate::cmd::{
    param_key_impl, type_parameters_impl, FlowCount, ParamsScope, ProfileCount, Scope, ScopeStruct,
};

/// Generates the `with_profile` method for the command context builder.
pub fn impl_with_profile(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::params);

    if scope_struct.scope().profile_count() != ProfileCount::One {
        // `with_profile` is not supported.
        return proc_macro2::TokenStream::new();
    };

    let scope_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        if scope.flow_count() == FlowCount::One {
            type_params.push(parse_quote!(FlowSelection));
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
            E,
            O,
            // FlowSelection,
            // PKeys,
            // WorkspaceParamsSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            #scope_params,
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    crate::scopes::type_params::ProfileNotSelected,
                    // FlowSelection,
                    // PKeys,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params
                >,
            >
        where
            PKeys: #params_module::ParamsKeys + 'static,
        {
            pub fn with_profile(
                self,
                profile: peace_core::Profile,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    crate::scopes::type_params::ProfileSelected,
                    // FlowSelection,
                    // PKeys,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params
                >,
            > {
                let Self {
                    output,
                    interrupt_rx,
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection: ProfileNotSelected,
                            // flow_selection,
                            // params_type_regs_builder,
                            // workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,
                            // params_specs_provided,
                            // marker: std::marker::PhantomData,
                            #scope_builder_fields_profile_not_selected
                        },
                } = self;

                let scope_builder = #scope_builder_name {
                    // profile_selection: ProfileSelected(profile),
                    // flow_selection,
                    // params_type_regs_builder,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,
                    // params_specs_provided,
                    // marker: std::marker::PhantomData,
                    #scope_builder_fields_profile_selected
                };

                crate::ctx::CmdCtxBuilder {
                    output,
                    interrupt_rx,
                    workspace,
                    scope_builder,
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

    let impl_params_with_workspace_params_k = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        if scope.flow_count() == FlowCount::One {
            type_params.push(parse_quote!(FlowSelection));
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
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        if scope.flow_count() == FlowCount::One {
            type_params.push(parse_quote!(FlowSelection));
        }

        let impl_params_key_known_params = {
            let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
            type_parameters_impl::params_key_known_push(
                &mut type_params,
                scope,
                ParamsScope::Workspace,
            );
            type_params
        };
        type_params.push(parse_quote! {
            peace_rt_model::params::ParamsKeysImpl<
                // KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                #impl_params_key_known_params
            >
        });

        type_params.push(parse_quote! {
            crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>
        });
        if scope.profile_params_supported() {
            type_params.push(parse_quote!(ProfileParamsSelection));
        }
        if scope.flow_params_supported() {
            type_params.push(parse_quote!(FlowParamsSelection));
        }
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
            E,
            O,
            // FlowSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            // WorkspaceParamsK,
            // ProfileParamsKMaybe,
            // FlowParamsKMaybe,
            #impl_params_with_workspace_params_k
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    crate::scopes::type_params::ProfileNotSelected,
                    // FlowSelection,

                    // peace_rt_model::params::ParamsKeysImpl<
                    //     KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    // >,

                    // WorkspaceParamsSome<WorkspaceParamsK>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params_with_workspace_params_k
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
                O,
                #scope_builder_name<
                    E,
                    crate::scopes::type_params::ProfileFromWorkspaceParam<'key, WorkspaceParamsK>,
                    // FlowSelection,

                    // peace_rt_model::params::ParamsKeysImpl<
                    //     KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    // >,

                    // WorkspaceParamsSome<WorkspaceParamsK>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params_with_workspace_params_k
                >,
            > {
                let Self {
                    output,
                    interrupt_rx,
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection: ProfileNotSelected,
                            // flow_selection,
                            // params_type_regs_builder,
                            // workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,
                            // params_specs_provided,
                            // marker: std::marker::PhantomData,
                            #scope_builder_fields_profile_not_selected
                        },
                } = self;

                let scope_builder = #scope_builder_name {
                    // profile_selection: ProfileFromWorkspaceParam(workspace_param_k),
                    // flow_selection,
                    // params_type_regs_builder,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,
                    // params_specs_provided,
                    // marker: std::marker::PhantomData,
                    #scope_builder_fields_profile_from_workspace
                };

                crate::ctx::CmdCtxBuilder {
                    output,
                    interrupt_rx,
                    workspace,
                    scope_builder,
                }
            }
        }
    }
}

fn scope_builder_fields_profile_not_selected(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(
        profile_selection: crate::scopes::type_params::ProfileNotSelected
    ));
    scope_builder_fields_remainder_push(scope, &mut field_values);

    field_values
}

fn scope_builder_fields_profile_selected(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(
        profile_selection: crate::scopes::type_params::ProfileSelected(profile)
    ));
    scope_builder_fields_remainder_push(scope, &mut field_values);

    field_values
}

fn scope_builder_fields_profile_from_workspace(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(
        profile_selection: crate::scopes::type_params::ProfileFromWorkspaceParam(workspace_param_k)
    ));
    scope_builder_fields_remainder_push(scope, &mut field_values);

    field_values
}

fn scope_builder_fields_remainder_push(
    scope: Scope,
    field_values: &mut Punctuated<FieldValue, Comma>,
) {
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_selection));
    }
    field_values.push(parse_quote!(params_type_regs_builder));
    field_values.push(parse_quote!(workspace_params_selection));
    if scope.profile_params_supported() {
        field_values.push(parse_quote!(profile_params_selection));
    }
    if scope.flow_params_supported() {
        field_values.push(parse_quote!(flow_params_selection));
    }
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(params_specs_provided));
    }
    field_values.push(parse_quote!(marker: std::marker::PhantomData));
}
