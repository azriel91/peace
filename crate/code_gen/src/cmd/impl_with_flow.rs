use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, token::Comma, FieldValue, Token};

use crate::cmd::{CmdCtxBuilderReturnTypeBuilder, FlowCount, ProfileCount, Scope, ScopeStruct};

/// Generates the `with_flow` method for the command context builder.
pub fn impl_with_flow(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;

    if scope.profile_count() == ProfileCount::None || scope.flow_count() == FlowCount::None {
        // `with_flow` is not supported.
        return proc_macro2::TokenStream::new();
    };

    let scope_builder_fields_flow_not_selected = scope_builder_fields_flow_not_selected(scope);
    let scope_builder_fields_flow_selected = scope_builder_fields_flow_selected(scope);

    let return_type = CmdCtxBuilderReturnTypeBuilder::new(scope_builder_name.clone())
        .with_flow_selection(parse_quote!(
            crate::scopes::type_params::FlowSelected<'ctx, CmdCtxBuilderTypeParamsT::AppError>
        ))
        .build();

    quote! {
        impl<'ctx, CmdCtxBuilderTypeParamsT> crate::ctx::CmdCtxBuilder<
            'ctx,
            CmdCtxBuilderTypeParamsT,
            #scope_builder_name<CmdCtxBuilderTypeParamsT>,
        >
        where
            CmdCtxBuilderTypeParamsT: crate::ctx::CmdCtxBuilderTypeParams<
                FlowSelection = crate::scopes::type_params::FlowNotSelected,
            >,
        {
            pub fn with_flow(
                self,
                flow: &'ctx peace_rt_model::Flow<CmdCtxBuilderTypeParamsT::AppError>,
            ) ->
                // crate::ctx::CmdCtxBuilder<
                //     'ctx,
                //     crate::ctx::CmdCtxBuilderTypeParamsCollector<
                //         CmdCtxBuilderTypeParamsT::Output,
                //         CmdCtxBuilderTypeParamsT::AppError,
                //         peace_rt_model::params::ParamsKeysImpl<
                //             <CmdCtxBuilderTypeParamsT::ParamsKeys as ParamsKeys>::WorkspaceParamsKMaybe,
                //             <CmdCtxBuilderTypeParamsT::ParamsKeys as ParamsKeys>::ProfileParamsKMaybe,
                //             <CmdCtxBuilderTypeParamsT::ParamsKeys as ParamsKeys>::FlowParamsKMaybe,
                //         >,
                //         CmdCtxBuilderTypeParamsT::WorkspaceParamsSelection,
                //         CmdCtxBuilderTypeParamsT::ProfileParamsSelection,
                //         CmdCtxBuilderTypeParamsT::FlowParamsSelection,
                //         CmdCtxBuilderTypeParamsT::ProfileSelection,
                //         crate::scopes::type_params::FlowSelected<'ctx, CmdCtxBuilderTypeParamsT::AppError>,
                //     >,
                // >
                #return_type
            {
                let Self {
                    output,
                    interruptibility,
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
                    #scope_builder_fields_flow_selected
                };

                crate::ctx::CmdCtxBuilder {
                    output,
                    interruptibility,
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

    field_values
}
