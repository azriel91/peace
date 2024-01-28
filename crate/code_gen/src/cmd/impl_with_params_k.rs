use quote::quote;
use syn::parse_quote;

use crate::cmd::{
    scope_builder_fields, with_params::cmd_ctx_builder_with_params_selected,
    CmdCtxBuilderTypeBuilder, ImplHeaderBuilder, ParamsScope, Scope, ScopeStruct,
};

/// Generates the `with_workspace_params_k` / `with_profile_params_k` /
/// `with_flow_params_k` methods.
pub fn impl_with_params_k(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    ParamsScope::iter().fold(
        proc_macro2::TokenStream::new(),
        |mut impl_tokens, params_scope| {
            match params_scope {
                ParamsScope::Workspace => {}
                ParamsScope::Profile => {
                    if !scope.profile_params_supported() {
                        return impl_tokens;
                    }
                }
                ParamsScope::Flow => {
                    if !scope.flow_params_supported() {
                        return impl_tokens;
                    }
                }
            }

            let impl_with_params_k_key_unknown =
                impl_with_params_k_key_unknown(scope_struct, params_scope);

            impl_tokens.extend(impl_with_params_k_key_unknown);

            match scope {
                // Single profile params technically don't need this, but it is convenient to
                // have a common `with_*_param` registration for each param type.
                //
                // Single profile params will also have `with_*_param_value` from the
                // `impl_with_param_value` module.
                Scope::SingleProfileNoFlow |
                Scope::SingleProfileSingleFlow |
                // Multi profile commands need to register the type of each profile param.
                Scope::MultiProfileNoFlow |
                Scope::MultiProfileSingleFlow => {
                    let impl_with_param_key_known = impl_with_param_key_known(scope_struct, params_scope);
                    impl_tokens.extend(impl_with_param_key_known);
                }
                Scope::NoProfileNoFlow if params_scope == ParamsScope::Workspace => {
                    let impl_with_param_key_known = impl_with_param_key_known(scope_struct, params_scope);
                    impl_tokens.extend(impl_with_param_key_known);
                }
                // No profile commands do not support profile params.
                Scope::NoProfileNoFlow => {}
            }

            impl_tokens
        },
    )
}

fn impl_with_params_k_key_unknown(
    scope_struct: &ScopeStruct,
    params_scope: ParamsScope,
) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;

    let params_scope_str = params_scope.to_str();
    let with_params_k_method_name = params_scope.params_k_method_name();
    let params_k_method_name = params_scope.params_k_method_name();
    let params_map_type = params_scope.params_map_type();
    let params_k_type_param = params_scope.params_k_type_param();
    let scope_builder_fields_params_none = scope_builder_fields::params_none(scope, params_scope);
    let scope_builder_fields_params_some_new =
        scope_builder_fields::params_some_new(scope, params_scope);

    let doc_summary = format!("Registers a {params_scope_str} parameter type.");

    // ```rust,ignore
    // crate::ctx::CmdCtxBuilder<
    //     'ctx,
    //     crate::ctx::CmdCtxBuilderTypesCollector<
    //         Output,
    //         AppError,
    //         peace_rt_model::params::ParamsKeysImpl<
    //             WorkspaceParamsKMaybe = peace_rt_model::params::KeyUnknown,
    //             ProfileParamsKMaybe,
    //             FlowParamsKMaybe,
    //         >,
    //         WorkspaceParamsSelection,
    //         ProfileParamsSelection,
    //         FlowParamsSelection,
    //         ProfileSelection,
    //         FlowSelection,
    //     >,
    // >
    // ```

    let builder_type = {
        let builder_type_builder = CmdCtxBuilderTypeBuilder::new(scope_builder_name.clone());
        match params_scope {
            ParamsScope::Workspace => builder_type_builder
                .with_workspace_params_k_maybe(parse_quote!(peace_rt_model::params::KeyUnknown))
                .with_workspace_params_selection(parse_quote!(
                    crate::scopes::type_params::WorkspaceParamsNone
                )),
            ParamsScope::Profile => builder_type_builder
                .with_profile_params_k_maybe(parse_quote!(peace_rt_model::params::KeyUnknown))
                .with_profile_params_selection(parse_quote!(
                    crate::scopes::type_params::ProfileParamsNone
                )),
            ParamsScope::Flow => builder_type_builder
                .with_flow_params_k_maybe(parse_quote!(peace_rt_model::params::KeyUnknown))
                .with_flow_params_selection(parse_quote!(
                    crate::scopes::type_params::FlowParamsNone
                )),
        }
        .build()
    };
    let impl_header = {
        let impl_header_builder = ImplHeaderBuilder::new(builder_type);
        match params_scope {
            ParamsScope::Workspace => impl_header_builder
                .with_workspace_params_k_maybe(None)
                .with_workspace_params_selection(None),
            ParamsScope::Profile => impl_header_builder
                .with_profile_params_k_maybe(None)
                .with_profile_params_selection(None),
            ParamsScope::Flow => impl_header_builder
                .with_flow_params_k_maybe(None)
                .with_flow_params_selection(None),
        }
        .build()
    };
    let return_type =
        cmd_ctx_builder_with_params_selected(scope_builder_name, scope_struct, params_scope);

    quote! {
        #impl_header
        {
            #[doc = #doc_summary]
            ///
            /// This is necessary to deserialize previously stored parameters.
            ///
            // pub fn with_workspace_params_k<WorkspaceParamsK>
            pub fn #with_params_k_method_name<#params_k_type_param>(
                self,
            ) ->
                // crate::ctx::CmdCtxBuilder<
                //     'ctx,
                //     crate::ctx::CmdCtxBuilderTypesCollector<
                //         CmdCtxBuilderTypesT::Output,
                //         CmdCtxBuilderTypesT::AppError,
                //         peace_rt_model::params::ParamsKeysImpl<
                //             peace_rt_model::params::KeyKnown<WorkspaceParamsK>,
                //             ProfileParamsKMaybe,
                //             FlowParamsKMaybe,
                //         >,
                //         crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>,
                //         CmdCtxBuilderTypesT::ProfileParamsSelection,
                //         CmdCtxBuilderTypesT::FlowParamsSelection,
                //         CmdCtxBuilderTypesT::ProfileSelection,
                //         CmdCtxBuilderTypesT::FlowSelection,
                //     >,
                // >
                #return_type
            where
                #params_k_type_param:
                    Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static,
            {
                let Self {
                    output,
                    interruptibility,
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection,
                            // flow_selection,
                            // params_type_regs_builder,
                            // workspace_params_selection: WorkspaceParamsNone,
                            // profile_params_selection,
                            // flow_params_selection,

                            #scope_builder_fields_params_none
                        },
                } = self;

                // let mut params_type_regs_builder =
                //     params_type_regs_builder.with_workspace_params_k::<WorkspaceParamsK>();
                let mut params_type_regs_builder =
                    params_type_regs_builder.#params_k_method_name::<#params_k_type_param>();

                // let mut workspace_params = WorkspaceParams::<WorkspaceParam>::new();
                let params_map = peace_rt_model::params::#params_map_type::<#params_k_type_param>::new();

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_selection,
                    // params_type_regs_builder,
                    // workspace_params_selection: WorkspaceParamsSome(params_map),
                    // profile_params_selection,
                    // flow_params_selection,

                    #scope_builder_fields_params_some_new
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

fn impl_with_param_key_known(
    scope_struct: &ScopeStruct,
    params_scope: ParamsScope,
) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;

    let params_scope_str = params_scope.to_str();
    let with_param_method_name = params_scope.with_param_method_name();
    let with_param_value_method_name = params_scope.with_param_value_method_name();
    let param_type_param = params_scope.param_type_param();
    let params_k_type_param = params_scope.params_k_type_param();
    let scope_builder_fields_params_some = scope_builder_fields::params_some(scope, params_scope);
    let scope_builder_fields_passthrough = scope_builder_fields::passthrough(scope, params_scope);
    let params_type_reg_method_name = params_scope.params_type_reg_mut_method_name();

    let doc_summary = format!("Registers a {params_scope_str} parameter type.");
    let doc_body =
        format!("See the `{with_param_value_method_name}` method if you want to specify a value.");

    let builder_type =
        cmd_ctx_builder_with_params_selected(scope_builder_name, scope_struct, params_scope);
    let impl_header = {
        let impl_header_builder = ImplHeaderBuilder::new(builder_type);
        match params_scope {
            ParamsScope::Workspace => impl_header_builder
                .with_workspace_params_k_maybe(None)
                .with_workspace_params_k(Some(parse_quote!(WorkspaceParamsK)))
                .with_workspace_params_selection(None),
            ParamsScope::Profile => impl_header_builder
                .with_profile_params_k_maybe(None)
                .with_profile_params_k(Some(parse_quote!(ProfileParamsK)))
                .with_profile_params_selection(None),
            ParamsScope::Flow => impl_header_builder
                .with_flow_params_k_maybe(None)
                .with_flow_params_k(Some(parse_quote!(FlowParamsK)))
                .with_flow_params_selection(None),
        }
        .build()
    };

    quote! {
        #impl_header
        {
            #[doc = #doc_summary]
            ///
            #[doc = #doc_body]
            ///
            /// # Parameters
            ///
            /// * `k`: Key to store the parameter with.
            // pub fn with_workspace_params<WorkspaceParam>
            ///
            /// # Type Parameters
            ///
            /// * `#param_type_param` The type that is stored against `#params_k_type_param`.
            pub fn #with_param_method_name<#param_type_param>(
                self,
                k: #params_k_type_param,
            ) -> Self
            where
                #param_type_param: Clone + std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static,
            {
                let Self {
                    output,
                    interruptibility,
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection,
                            // flow_selection,
                            // mut params_type_regs_builder,
                            // mut workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,

                            #scope_builder_fields_params_some
                        },
                } = self;

                params_type_regs_builder
                    .#params_type_reg_method_name()
                    .register::<#param_type_param>(k.clone());

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_selection,
                    // params_type_regs_builder,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,

                    #scope_builder_fields_passthrough
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
