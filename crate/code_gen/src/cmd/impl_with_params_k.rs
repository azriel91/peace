use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, GenericArgument, Path, Token};

use crate::cmd::{
    param_key_impl, scope_builder_fields, type_parameters_impl, ParamsScope, Scope, ScopeStruct,
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
                // No profile commands do not support profile params.
                Scope::NoProfileNoFlow => {}

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
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    // ProfileSelection, FlowSelection
    let selection_type_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
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

    let impl_scope_builder_type_params_none = {
        let mut type_params = selection_type_params.clone();
        type_parameters_impl::params_selection_none_push(&mut type_params, scope, params_scope);
        type_params
    };

    let impl_scope_builder_type_params_some = {
        let mut type_params = selection_type_params;
        type_parameters_impl::params_selection_some_push(&mut type_params, scope, params_scope);
        type_params
    };

    let param_key_impl_unknown_predicates = param_key_impl::unknown_predicates(scope, params_scope);

    let params_scope_str = params_scope.to_str();
    let with_params_k_method_name = params_scope.params_k_method_name();
    let params_map_type = params_scope.params_map_type();
    let params_k_method_name = params_scope.params_k_method_name();
    let params_k_type_param = params_scope.params_k_type_param();
    let scope_builder_fields_params_none = scope_builder_fields::params_none(scope, params_scope);
    let scope_builder_fields_params_some_new =
        scope_builder_fields::params_some_new(scope, params_scope);

    let doc_summary = format!("Registers a {params_scope_str} parameter type.");

    quote! {
        impl<
            'ctx,
            E,
            O,
            // ProfileSelection,
            // FlowSelection,
            // ProfileParamsSelection,
            // ProfileParamsKMaybe,
            // FlowParamsKMaybe,

            #impl_type_params
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    // ProfileSelection,
                    // FlowSelection,

                    // peace_rt_model::cmd_context_params::ParamsKeysImpl<
                    //     KeyUnknown, ProfileParamsKMaybe, FlowParamsKMaybe
                    // >,

                    // WorkspaceParamsNone,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,

                    #impl_scope_builder_type_params_none
                >,
            >
        where
            // ProfileParamsKMaybe: KeyMaybe,
            // FlowParamsKMaybe: KeyMaybe,
            #param_key_impl_unknown_predicates
        {
            #[doc = #doc_summary]
            ///
            /// This is necessary to deserialize previously stored parameters.
            ///
            // pub fn with_workspace_params_k<WorkspaceParamsK>
            pub fn #with_params_k_method_name<#params_k_type_param>(
                self,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    // ProfileSelection,
                    // FlowSelection,

                    // peace_rt_model::cmd_context_params::ParamsKeysImpl<
                    //     KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    // >,

                    // WorkspaceParamsSome<#params_k_type_param>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,

                    #impl_scope_builder_type_params_some
                >,
            >
            where
                #params_k_type_param:
                    Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static,
            {
                let Self {
                    output,
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection,
                            // flow_selection,
                            // params_type_regs_builder,
                            // workspace_params_selection: WorkspaceParamsNone,
                            // profile_params_selection,
                            // flow_params_selection,
                            // marker,

                            #scope_builder_fields_params_none
                        },
                } = self;

                // let mut params_type_regs_builder =
                //     params_type_regs_builder.with_workspace_params_k::<WorkspaceParamsK>();
                let mut params_type_regs_builder =
                    params_type_regs_builder.#params_k_method_name::<#params_k_type_param>();

                // let mut workspace_params = WorkspaceParams::<WorkspaceParam>::new();
                let params_map = #params_module::#params_map_type::<#params_k_type_param>::new();

                let scope_builder = #scope_builder_name {
                    // profile_selection,
                    // flow_selection,
                    // params_type_regs_builder,
                    // workspace_params_selection: WorkspaceParamsSome(params_map),
                    // profile_params_selection,
                    // flow_params_selection,
                    // marker,

                    #scope_builder_fields_params_some_new
                };

                crate::ctx::CmdCtxBuilder {
                    output,
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

    // ProfileSelection, FlowSelection
    let selection_type_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);
        type_params
    };

    let impl_type_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();

        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);
        type_parameters_impl::params_selection_maybe_push(
            &mut type_params,
            scope,
            params_scope,
            true,
        );

        type_params
    };

    let impl_scope_builder_type_params_some = {
        let mut type_params = selection_type_params;
        type_parameters_impl::params_selection_some_push(&mut type_params, scope, params_scope);
        type_params
    };

    let param_key_impl_known_predicates = param_key_impl::known_predicates(scope, params_scope);

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

    quote! {
        impl<
            'ctx,
            E,
            O,
            // ProfileSelection,
            // FlowSelection,
            // ProfileParamsSelection,
            // WorkspaceParamsK,
            // ProfileParamsKMaybe,
            // FlowParamsKMaybe,

            #impl_type_params
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    // ProfileSelection,
                    // FlowSelection,

                    // peace_rt_model::cmd_context_params::ParamsKeysImpl<
                    //     KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    // >,

                    // WorkspaceParamsSome<WorkspaceParamsK>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,

                    #impl_scope_builder_type_params_some
                >,
            >
        where
            // WorkspaceParamsK:
            //     Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static,
            // ProfileParamsKMaybe: KeyMaybe,
            // FlowParamsKMaybe: KeyMaybe,

            #param_key_impl_known_predicates
        {
            #[doc = #doc_summary]
            ///
            #[doc = #doc_body]
            ///
            /// # Parameters
            ///
            /// * `k`: Key to store the parameter with.
            // pub fn with_workspace_params<WorkspaceParam>
            pub fn #with_param_method_name<#param_type_param>(
                self,
                k: #params_k_type_param,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                #scope_builder_name<
                    E,
                    // ProfileSelection,
                    // FlowSelection,

                    // peace_rt_model::cmd_context_params::ParamsKeysImpl<
                    //     KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe
                    // >,

                    // WorkspaceParamsSome<WorkspaceParamsK>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,

                    #impl_scope_builder_type_params_some
                >,
            >
            where
                #param_type_param: Clone + std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static,
            {
                let Self {
                    output,
                    workspace,
                    scope_builder:
                        #scope_builder_name {
                            // profile_selection,
                            // flow_selection,
                            // mut params_type_regs_builder,
                            // mut workspace_params_selection,
                            // profile_params_selection,
                            // flow_params_selection,
                            // marker,

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
                    // marker,

                    #scope_builder_fields_passthrough
                };

                crate::ctx::CmdCtxBuilder {
                    output,
                    workspace,
                    scope_builder,
                }
            }
        }
    }
}
