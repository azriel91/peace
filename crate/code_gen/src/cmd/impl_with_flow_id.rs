use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, token::Comma, FieldValue, Path, Token};

use crate::cmd::{type_parameters_impl, FlowCount, ProfileCount, Scope, ScopeStruct};

/// Generates the `with_flow_id` method for the command context builder.
pub fn impl_with_flow_id(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    if scope.profile_count() == ProfileCount::None || scope.flow_count() == FlowCount::None {
        // `with_flow_id` is not supported.
        return proc_macro2::TokenStream::new();
    };

    let impl_type_params = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        match scope.profile_count() {
            ProfileCount::None => {
                unreachable!("FlowId is not specifiable when there are no profiles.")
            }
            ProfileCount::One | ProfileCount::Multiple => {
                type_params.push(parse_quote!(ProfileSelection));
            }
        }
        type_parameters_impl::params_selection_push(&mut type_params, scope);
        type_params
    };
    let scope_type_params_flow_id_not_selected = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        match scope.profile_count() {
            ProfileCount::None => {
                unreachable!("FlowId is not specifiable when there are no profiles.")
            }
            ProfileCount::One | ProfileCount::Multiple => {
                type_params.push(parse_quote!(ProfileSelection));
            }
        }
        type_params.push(parse_quote!(crate::scopes::type_params::FlowIdNotSelected));
        type_parameters_impl::params_selection_push(&mut type_params, scope);
        type_params
    };
    let scope_type_params_flow_id_selected = {
        let mut type_params = Punctuated::<Path, Token![,]>::new();
        match scope.profile_count() {
            ProfileCount::None => {
                unreachable!("FlowId is not specifiable when there are no profiles.")
            }
            ProfileCount::One | ProfileCount::Multiple => {
                type_params.push(parse_quote!(ProfileSelection));
            }
        }
        type_params.push(parse_quote!(crate::scopes::type_params::FlowIdSelected));
        type_parameters_impl::params_selection_push(&mut type_params, scope);
        type_params
    };

    let scope_builder_fields_flow_id_not_selected =
        scope_builder_fields_flow_id_not_selected(scope);
    let scope_builder_fields_flow_id_selected = scope_builder_fields_flow_id_selected(scope);

    quote! {
        impl<
            'ctx,
            PKeys,
            // ProfileSelection,
            // WorkspaceParamsSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            #impl_type_params
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    // ProfileSelection,
                    // FlowIdNotSelected,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_type_params_flow_id_not_selected
                >,
                PKeys,
            >
        where
            PKeys: #params_module::ParamsKeys + 'static,
        {
            pub fn with_flow_id(
                self,
                flow_id: peace_core::FlowId,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    // ProfileSelection,
                    // FlowIdSelected,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_type_params_flow_id_selected
                >,
                PKeys,
            > {
                let Self {
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection,
                            // flow_id_selection: FlowIdNotSelected,
                            // workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,
                            #scope_builder_fields_flow_id_not_selected
                        },
                    params_type_regs_builder,
                } = self;

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_id_selection: FlowIdSelected(flow_id),
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,
                    #scope_builder_fields_flow_id_selected
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

fn scope_builder_fields_flow_id_not_selected(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(profile_selection));
    field_values.push(parse_quote!(
        flow_id_selection: crate::scopes::type_params::FlowIdNotSelected
    ));
    field_values.push(parse_quote!(workspace_params_selection));
    if scope.profile_params_supported() {
        field_values.push(parse_quote!(profile_params_selection));
    }
    if scope.flow_params_supported() {
        field_values.push(parse_quote!(flow_params_selection));
    }

    field_values
}

fn scope_builder_fields_flow_id_selected(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(profile_selection));
    field_values.push(parse_quote!(
        flow_id_selection: crate::scopes::type_params::FlowIdSelected(flow_id)
    ));
    field_values.push(parse_quote!(workspace_params_selection));
    if scope.profile_params_supported() {
        field_values.push(parse_quote!(profile_params_selection));
    }
    if scope.flow_params_supported() {
        field_values.push(parse_quote!(flow_params_selection));
    }

    field_values
}
