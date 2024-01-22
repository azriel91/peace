use quote::quote;
use syn::{parse_quote, Path};

use crate::cmd::{
    scope_builder_fields, CmdCtxBuilderTypeBuilder, ImplHeaderBuilder, ParamsScope, Scope,
    ScopeStruct,
};

/// Generates the `with_workspace_param_value` / `with_profile_param_value` /
/// `with_flow_param_value` methods.
pub fn impl_with_param(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    ParamsScope::iter().fold(
        proc_macro2::TokenStream::new(),
        |mut impl_tokens, params_scope| {
            match params_scope {
                ParamsScope::Workspace => {}
                ParamsScope::Profile => {
                    match scope {
                        // Multi profile commands may read, but not write profile params.
                        Scope::MultiProfileNoFlow |
                        Scope::MultiProfileSingleFlow |
                        // No profile commands do not support profile params.
                        Scope::NoProfileNoFlow => return impl_tokens,

                        Scope::SingleProfileNoFlow |
                        Scope::SingleProfileSingleFlow => {
                            // implement method
                        }
                    }
                }
                ParamsScope::Flow => {
                    match scope {
                        // Multi profile commands may read, but not write flow params.
                        Scope::MultiProfileNoFlow |
                        Scope::MultiProfileSingleFlow |
                        // No flow commands do not support flow params.
                        Scope::SingleProfileNoFlow |
                        Scope::NoProfileNoFlow => return impl_tokens,

                        Scope::SingleProfileSingleFlow => {
                            // implement method
                        }
                    }
                }
            }

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
    let params_module: Path = parse_quote!(peace_rt_model::params);

    let params_scope_str = params_scope.to_str();
    let with_param_value_method_name = params_scope.with_param_value_method_name();
    let params_map_type = params_scope.params_map_type();
    let param_type_param = params_scope.param_type_param();
    let params_k_method_name = params_scope.params_k_method_name();
    let params_k_type_param = params_scope.params_k_type_param();
    let param_name = params_scope.param_name();
    let param_name_str = param_name.to_string();
    let scope_builder_fields_params_none = scope_builder_fields::params_none(scope, params_scope);
    let scope_builder_fields_params_some_new =
        scope_builder_fields::params_some_new(scope, params_scope);
    let params_type_reg_mut_method_name = params_scope.params_type_reg_mut_method_name();

    let doc_summary = format!("Adds a {params_scope_str} parameter.");
    let doc_param = format!("* `{param_name_str}`: The {params_scope_str} parameter to register.");

    // ```rust,ignore
    // crate::ctx::CmdCtxBuilder<
    //     'ctx,
    //     crate::ctx::CmdCtxBuilderTypeParamsCollector<
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
    let return_type = {
        let builder_type_builder = CmdCtxBuilderTypeBuilder::new(scope_builder_name.clone());
        match params_scope {
            ParamsScope::Workspace => builder_type_builder
                .with_workspace_params_k_maybe(parse_quote!(
                    peace_rt_model::params::KeyKnown<WorkspaceParamsK>
                ))
                .with_workspace_params_selection(parse_quote!(
                    crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>
                )),
            ParamsScope::Profile => builder_type_builder
                .with_profile_params_k_maybe(parse_quote!(
                    peace_rt_model::params::KeyKnown<ProfileParamsK>
                ))
                .with_profile_params_selection(parse_quote!(
                    crate::scopes::type_params::ProfileParamsSome<ProfileParamsK>
                )),
            ParamsScope::Flow => builder_type_builder
                .with_flow_params_k_maybe(parse_quote!(
                    peace_rt_model::params::KeyKnown<FlowParamsK>
                ))
                .with_flow_params_selection(parse_quote!(
                    crate::scopes::type_params::FlowParamsSome<FlowParamsK>
                )),
        }
        .build()
    };

    quote! {
        #impl_header
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
            // pub fn with_workspace_param_value<WorkspaceParamsK, WorkspaceParam>
            pub fn #with_param_value_method_name<#params_k_type_param, #param_type_param>(
                self,
                k: #params_k_type_param,
                #param_name: Option<#param_type_param>,
            ) ->
                // crate::ctx::CmdCtxBuilder<
                //     'ctx,
                //     crate::ctx::CmdCtxBuilderTypeParamsCollector<
                //         CmdCtxBuilderTypeParamsT::Output,
                //         CmdCtxBuilderTypeParamsT::AppError,
                //         peace_rt_model::params::ParamsKeysImpl<
                //             peace_rt_model::params::KeyKnown<WorkspaceParamsK>,
                //             <CmdCtxBuilderTypeParamsT::ParamsKeys as ParamsKeys>::ProfileParamsKMaybe,
                //             <CmdCtxBuilderTypeParamsT::ParamsKeys as ParamsKeys>::FlowParamsKMaybe,
                //         >,
                //         crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>,
                //         CmdCtxBuilderTypeParamsT::ProfileParamsSelection,
                //         CmdCtxBuilderTypeParamsT::FlowParamsSelection,
                //         CmdCtxBuilderTypeParamsT::ProfileSelection,
                //         CmdCtxBuilderTypeParamsT::FlowSelection,
                //     >,
                // >
                #return_type
            where
                #params_k_type_param:
                    Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static,
                #param_type_param: Clone + std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static,
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
                            // params_specs_provided,

                            #scope_builder_fields_params_none
                        },
                } = self;

                // let mut params_type_regs_builder =
                //     params_type_regs_builder.with_workspace_params_k::<WorkspaceParamsK>();
                let mut params_type_regs_builder =
                    params_type_regs_builder.#params_k_method_name::<#params_k_type_param>();
                params_type_regs_builder
                    .#params_type_reg_mut_method_name()
                    .register::<#param_type_param>(k.clone());
                // let mut workspace_params = WorkspaceParams::<WorkspaceParam>::new();
                let mut params_map = #params_module::#params_map_type::<#params_k_type_param>::new();
                if let Some(#param_name) = #param_name {
                    params_map.insert(k, #param_name);
                }

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_selection,
                    // params_type_regs_builder,
                    // workspace_params_selection: WorkspaceParamsSome(params_map),
                    // profile_params_selection,
                    // flow_params_selection,
                    // params_specs_provided,

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
    let with_param_value_method_name = params_scope.with_param_value_method_name();
    let param_type_param = params_scope.param_type_param();
    let params_k_type_param = params_scope.params_k_type_param();
    let param_name = params_scope.param_name();
    let param_name_str = param_name.to_string();
    let params_selection_name = params_scope.params_selection_name();
    let scope_builder_fields_params_some = scope_builder_fields::params_some(scope, params_scope);
    let scope_builder_fields_passthrough = scope_builder_fields::passthrough(scope, params_scope);
    let params_type_reg_method_name = params_scope.params_type_reg_mut_method_name();

    let doc_summary = format!("Adds a {params_scope_str} parameter.");
    let doc_param = format!("* `{param_name_str}`: The {params_scope_str} parameter to register.");

    // ```rust,ignore
    // crate::ctx::CmdCtxBuilder<
    //     'ctx,
    //     crate::ctx::CmdCtxBuilderTypeParamsCollector<
    //         Output,
    //         AppError,
    //         peace_rt_model::params::ParamsKeysImpl<
    //             WorkspaceParamsKMaybe,
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
                .with_workspace_params_k_maybe(parse_quote!(
                    peace_rt_model::params::KeyKnown<WorkspaceParamsK>
                ))
                .with_workspace_params_selection(parse_quote!(
                    crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>
                )),
            ParamsScope::Profile => builder_type_builder
                .with_profile_params_k_maybe(parse_quote!(
                    peace_rt_model::params::KeyKnown<ProfileParamsK>
                ))
                .with_profile_params_selection(parse_quote!(
                    crate::scopes::type_params::ProfileParamsSome<ProfileParamsK>
                )),
            ParamsScope::Flow => builder_type_builder
                .with_flow_params_k_maybe(parse_quote!(
                    peace_rt_model::params::KeyKnown<FlowParamsK>
                ))
                .with_flow_params_selection(parse_quote!(
                    crate::scopes::type_params::FlowParamsSome<FlowParamsK>
                )),
        }
        .build()
    };
    let return_type = builder_type.clone();
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
            /// Currently there is no means in code to deliberately unset any previously
            /// stored value. This can be made possibly by defining a `*ParamsBuilder`
            /// that determines a `None` value as a deliberate erasure of any previous
            /// value.
            ///
            /// # Parameters
            ///
            /// * `k`: Key to store the parameter with.
            #[doc = #doc_param]
            // pub fn with_workspace_param_value<WorkspaceParam>
            pub fn #with_param_value_method_name<#param_type_param>(
                self,
                k: #params_k_type_param,
                #param_name: Option<#param_type_param>,
            ) ->
                // crate::ctx::CmdCtxBuilder<
                //     'ctx,
                //     crate::ctx::CmdCtxBuilderTypeParamsCollector<
                //         CmdCtxBuilderTypeParamsT::Output,
                //         CmdCtxBuilderTypeParamsT::AppError,
                //         peace_rt_model::params::ParamsKeysImpl<
                //             peace_rt_model::params::KeyKnown<WorkspaceParamsK>,
                //             <CmdCtxBuilderTypeParamsT::ParamsKeys as ParamsKeys>::ProfileParamsKMaybe,
                //             <CmdCtxBuilderTypeParamsT::ParamsKeys as ParamsKeys>::FlowParamsKMaybe,
                //         >,
                //         crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>,
                //         CmdCtxBuilderTypeParamsT::ProfileParamsSelection,
                //         CmdCtxBuilderTypeParamsT::FlowParamsSelection,
                //         CmdCtxBuilderTypeParamsT::ProfileSelection,
                //         CmdCtxBuilderTypeParamsT::FlowSelection,
                //     >,
                // >
                #return_type
            where
                #param_type_param: Clone + std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static,
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
                            // params_specs_provided,

                            #scope_builder_fields_params_some
                        },
                } = self;

                params_type_regs_builder
                    .#params_type_reg_method_name()
                    .register::<#param_type_param>(k.clone());
                // let workspace_params = &mut workspace_params_selection.0;
                let params_map = &mut #params_selection_name.0;
                if let Some(#param_name) = #param_name {
                    params_map.insert(k, #param_name);
                }

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_selection,
                    // params_type_regs_builder,
                    // workspace_params_selection,
                    // profile_params_selection,
                    // flow_params_selection,
                    // params_specs_provided,

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
