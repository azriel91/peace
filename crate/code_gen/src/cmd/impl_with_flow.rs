use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, FieldValue, GenericArgument, Path, Token,
};

use crate::cmd::{type_parameters_impl, FlowCount, ProfileCount, Scope, ScopeStruct};

/// Generates the `with_flow` method for the command context builder.
pub fn impl_with_flow(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::params);

    if scope.profile_count() == ProfileCount::None || scope.flow_count() == FlowCount::None {
        // `with_flow` is not supported.
        return proc_macro2::TokenStream::new();
    };

    let impl_type_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        match scope.profile_count() {
            ProfileCount::None => {
                unreachable!("Flow is not specifiable when there are no profiles.")
            }
            ProfileCount::One | ProfileCount::Multiple => {
                type_params.push(parse_quote!(ProfileSelection));
            }
        }
        type_parameters_impl::params_selection_push(&mut type_params, scope);
        type_params
    };
    let scope_builder_type_params_flow_not_selected = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        match scope.profile_count() {
            ProfileCount::None => {
                unreachable!("Flow is not specifiable when there are no profiles.")
            }
            ProfileCount::One | ProfileCount::Multiple => {
                type_params.push(parse_quote!(ProfileSelection));
            }
        }
        type_params.push(parse_quote!(crate::scopes::type_params::FlowNotSelected));
        type_parameters_impl::params_selection_push(&mut type_params, scope);
        type_params
    };
    let scope_builder_type_params_flow_selected = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        match scope.profile_count() {
            ProfileCount::None => {
                unreachable!("Flow is not specifiable when there are no profiles.")
            }
            ProfileCount::One | ProfileCount::Multiple => {
                type_params.push(parse_quote!(ProfileSelection));
            }
        }
        type_params.push(parse_quote!(
            crate::scopes::type_params::FlowSelected<'ctx, E>
        ));
        type_parameters_impl::params_selection_push(&mut type_params, scope);
        type_params
    };

    let scope_builder_fields_flow_not_selected = scope_builder_fields_flow_not_selected(scope);
    let scope_builder_fields_flow_selected = scope_builder_fields_flow_selected(scope);

    quote! {
        impl<
            'ctx,
            E,
            O,
            // ProfileSelection,
            // PKeys,
            // WorkspaceParamsSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            #impl_type_params
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    // ProfileSelection,
                    // FlowNotSelected,
                    // PKeys,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_builder_type_params_flow_not_selected
                >,
            >
        where
            PKeys: #params_module::ParamsKeys + 'static,
        {
            pub fn with_flow(
                self,
                flow: &'ctx peace_rt_model::Flow<E>,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    // ProfileSelection,
                    // FlowSelected<'ctx, E>,
                    // PKeys,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_builder_type_params_flow_selected
                >,
            > {
                let Self {
                    output,
                    interrupt_rx,
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection,
                            // flow_selection: FlowNotSelected,
                            // params_type_regs_builder,
                            // workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,
                            // params_specs_provided,
                            // marker: std::marker::PhantomData,
                            #scope_builder_fields_flow_not_selected
                        },
                } = self;

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_selection: FlowSelected(flow),
                    // params_type_regs_builder,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,
                    // params_specs_provided,
                    // marker: std::marker::PhantomData,
                    #scope_builder_fields_flow_selected
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

fn scope_builder_fields_flow_not_selected(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(profile_selection));
    field_values.push(parse_quote!(
        flow_selection: crate::scopes::type_params::FlowNotSelected
    ));
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

    field_values
}

fn scope_builder_fields_flow_selected(scope: Scope) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    field_values.push(parse_quote!(profile_selection));
    field_values.push(parse_quote!(
        flow_selection: crate::scopes::type_params::FlowSelected(flow)
    ));
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

    field_values
}
