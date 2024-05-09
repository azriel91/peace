use quote::quote;
use syn::parse_quote;

use crate::cmd::{
    with_params::cmd_ctx_builder_with_params_selected, ImplHeaderBuilder, ParamsScope,
    ProfileCount, ScopeStruct,
};

/// Generates the `CmdCtxBuilder::*_params_merge` methods for each params type.
///
/// The generated method attempts to load params from storage, and if it is
/// present, merges it with the params passed to the command context builder.
pub fn impl_params_merge(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    ParamsScope::iter().fold(
        proc_macro2::TokenStream::new(),
        |mut impl_tokens, params_scope| {
            let scope = scope_struct.scope();
            if ((scope.profile_count() == ProfileCount::Multiple
                && matches!(params_scope, ParamsScope::Profile | ParamsScope::Flow))
                || !scope.profile_params_supported() && params_scope == ParamsScope::Profile)
                || (!scope_struct.scope().flow_params_supported()
                    && params_scope == ParamsScope::Flow)
            {
                // Skip `*_params_merge` implementation if it is not supported.
                return impl_tokens;
            }

            impl_tokens.extend(impl_params_merge_for(scope_struct, params_scope));

            impl_tokens
        },
    )
}

fn impl_params_merge_for(
    scope_struct: &ScopeStruct,
    params_scope: ParamsScope,
) -> proc_macro2::TokenStream {
    let scope_builder_name = &scope_struct.item_struct().ident;

    let params_merge_method_name = params_scope.params_merge_method_name();
    let params_deserialize_method_name = params_scope.params_deserialize_method_name();
    let p_keys_key_maybe_key = params_scope.p_keys_key_maybe_key();
    let params_type_reg_method_name = params_scope.params_type_reg_method_name();
    let params_file_name = params_scope.params_file_name();
    let params_file_type = params_scope.params_file_type();
    let params_map_type = params_scope.params_map_type();
    let params_k_type_param = params_scope.params_k_type_param();

    let doc_summary = {
        let params_scope_str = params_scope.to_str();
        format!(
            "Merges {params_scope_str} params provided by the caller with the {params_scope_str} params on disk."
        )
    };

    let builder_type =
        cmd_ctx_builder_with_params_selected(scope_builder_name, scope_struct, params_scope);
    let impl_header = {
        let impl_header_builder = ImplHeaderBuilder::new(builder_type);
        match params_scope {
            ParamsScope::Workspace => impl_header_builder
                .with_workspace_params_k_maybe(None)
                .with_workspace_params_k(parse_quote!(WorkspaceParamsK))
                .with_workspace_params_selection(None),
            ParamsScope::Profile => impl_header_builder
                .with_profile_params_k_maybe(None)
                .with_profile_params_k(parse_quote!(ProfileParamsK))
                .with_profile_params_selection(None),
            ParamsScope::Flow => impl_header_builder
                .with_flow_params_k_maybe(None)
                .with_flow_params_k(parse_quote!(FlowParamsK))
                .with_flow_params_selection(None),
        }
        .build()
    };

    let params_keys_type = match params_scope {
        ParamsScope::Workspace => quote! {
            peace_rt_model::params::ParamsKeysImpl<
                peace_rt_model::params::KeyKnown<WorkspaceParamsK>,
                ProfileParamsKMaybe,
                FlowParamsKMaybe,
            >,
        },
        ParamsScope::Profile => quote! {
            peace_rt_model::params::ParamsKeysImpl<
                WorkspaceParamsKMaybe,
                peace_rt_model::params::KeyKnown<ProfileParamsK>,
                FlowParamsKMaybe,
            >,
        },
        ParamsScope::Flow => quote! {
            peace_rt_model::params::ParamsKeysImpl<
                WorkspaceParamsKMaybe,
                ProfileParamsKMaybe,
                peace_rt_model::params::KeyKnown<FlowParamsK>,
            >,
        },
    };

    quote! {
        #impl_header
        {
            #[doc = #doc_summary]
            // async fn workspace_params_merge
            async fn #params_merge_method_name(
                storage: &peace_rt_model::Storage,
                // params_type_regs_builder:
                //     &peace_rt_model::params::ParamsTypeRegsBuilder<
                //         peace_rt_model::params::ParamsKeysImpl<
                //             peace_rt_model::params::KeyKnown<WorkspaceParamsK>,
                //             ProfileParamsKMaybe,
                //             FlowParamsKMaybe,
                //         >,
                //     >,
                params_type_regs_builder:
                    &peace_rt_model::params::ParamsTypeRegsBuilder<#params_keys_type>,
                // params: &mut peace_rt_model::params::WorkspaceParams<WorkspaceParamsK>,
                params: &mut peace_rt_model::params::#params_map_type<#params_k_type_param>,
                // workspace_params_file: &peace_resources::internal::WorkspaceParamsFile,
                #params_file_name: &peace_resources::internal::#params_file_type,
            ) -> Result<(), peace_rt_model::Error> {
                let params_deserialized = peace_rt_model::WorkspaceInitializer::#params_deserialize_method_name::<
                    // WorkspaceParamsK,
                    #p_keys_key_maybe_key
                >(
                    storage,
                    // params_type_regs_builder.workspace_params_type_reg(),
                    params_type_regs_builder.#params_type_reg_method_name(),
                    #params_file_name,
                )
                .await?;
                match params_deserialized {
                    Some(params_deserialized) => {
                        // Merge `params` on top of `params_deserialized`.
                        // or, copy `params_deserialized` to `params` where
                        // there isn't a value.

                        if params.is_empty() {
                            *params = params_deserialized;
                        } else {
                            params_deserialized
                                .into_inner()
                                .into_inner()
                                .into_iter()
                                .for_each(|(key, param)| {
                                    if !params.contains_key(&key) {
                                        params.insert_raw(key, param);
                                    }
                                });
                        }

                    }
                    None => {}
                }

                Ok(())
            }
        }
    }
}
