use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, FieldValue, GenericArgument, Path, Token,
};

use crate::cmd::{type_parameters_impl, FlowCount, ProfileCount, Scope, ScopeStruct};

/// Generates the `with_profile_filter` method for the command context builder.
pub fn impl_with_profile_filter(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    if scope_struct.scope().profile_count() != ProfileCount::Multiple {
        // `with_profile_filter` is not supported.
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
    let scope_builder_fields_profile_filter_fn = scope_builder_fields_profile_filter_fn(scope);

    quote! {
        impl<
            'ctx,
            // FlowSelection,
            // WorkspaceParamsSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            #scope_params,
            PKeys,
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    crate::scopes::type_params::ProfileNotSelected,
                    // FlowSelection,
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
            pub fn with_profile_filter<F>(
                self,
                profile_filter_fn: F,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    crate::scopes::type_params::ProfileFilterFn,
                    // FlowSelection,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_params
                >,
                PKeys,
            >
            where
                F: (Fn(&peace_core::Profile) -> bool) + 'static
            {
                let Self {
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection: ProfileNotSelected,
                            // flow_selection,
                            // workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,
                            #scope_builder_fields_profile_not_selected
                        },
                    params_type_regs_builder,
                } = self;

                let scope_builder = #scope_builder_name {
                    // profile_selection: ProfileFilterFn(Box::new(profile_filter_fn)),
                    // flow_selection,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,
                    #scope_builder_fields_profile_filter_fn
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
        profile_selection: crate::scopes::type_params::ProfileNotSelected
    ));
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_selection));
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

fn scope_builder_fields_profile_filter_fn(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(
        profile_selection: crate::scopes::type_params::ProfileFilterFn(Box::new(profile_filter_fn))
    ));
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_selection));
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
