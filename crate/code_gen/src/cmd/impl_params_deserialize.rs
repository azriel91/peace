use quote::quote;
use syn::parse_quote;

use crate::cmd::{CmdCtxBuilderTypeBuilder, ImplHeaderBuilder, ParamsScope, ScopeStruct};

/// Generates the `CmdCtxBuilder::*_params_deserialize` methods for each params
/// type.
///
/// The generated method attempts to load params from storage.
pub fn impl_params_deserialize(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    ParamsScope::iter().fold(
        proc_macro2::TokenStream::new(),
        |mut impl_tokens, params_scope| {
            if (!scope_struct.scope().profile_params_supported()
                && params_scope == ParamsScope::Profile)
                || (!scope_struct.scope().flow_params_supported()
                    && params_scope == ParamsScope::Flow)
            {
                // Skip `*_params_deserialize` implementation if it is not supported.
                return impl_tokens;
            }

            impl_tokens.extend(impl_params_deserialize_for(scope_struct, params_scope));

            impl_tokens
        },
    )
}

fn impl_params_deserialize_for(
    scope_struct: &ScopeStruct,
    params_scope: ParamsScope,
) -> proc_macro2::TokenStream {
    let scope_builder_name = &scope_struct.item_struct().ident;

    let params_deserialize_method_name = params_scope.params_deserialize_method_name();
    let params_map_type = params_scope.params_map_type();
    let p_keys_key_maybe_key = params_scope.p_keys_key_maybe_key();
    let params_type_reg_method_name = params_scope.params_type_reg_method_name();
    let params_file_name = params_scope.params_file_name();
    let params_file_type = params_scope.params_file_type();

    let doc_summary = {
        let params_scope_str = params_scope.to_str();
        format!(
            "Merges {params_scope_str} params provided by the caller with the {params_scope_str} params on disk."
        )
    };

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

    let params_keys_impl_type_params = match params_scope {
        ParamsScope::Workspace => quote!(
            peace_rt_model::params::KeyKnown<WorkspaceParamsK>,
            ProfileParamsKMaybe,
            FlowParamsKMaybe,
        ),
        ParamsScope::Profile => quote!(
            WorkspaceParamsKMaybe,
            peace_rt_model::params::KeyKnown<ProfileParamsK>,
            FlowParamsKMaybe,
        ),
        ParamsScope::Flow => quote!(
            WorkspaceParamsKMaybe,
            ProfileParamsKMaybe,
            peace_rt_model::params::KeyKnown<FlowParamsK>,
        ),
    };

    quote! {
        #impl_header
        {
            #[doc = #doc_summary]
            // async fn workspace_params_deserialize
            async fn #params_deserialize_method_name(
                storage: &peace_rt_model::Storage,
                params_type_regs_builder: &peace_rt_model::params::ParamsTypeRegsBuilder<
                    peace_rt_model::params::ParamsKeysImpl<
                        #params_keys_impl_type_params
                    >,
                >,
                // workspace_params_file: &peace_resources::internal::WorkspaceParamsFile,
                #params_file_name: &peace_resources::internal::#params_file_type,
            // ) -> Result<Option<WorkspaceParams<K>, peace_rt_model::Error> {
            ) -> Result<Option<peace_rt_model::params::#params_map_type<#p_keys_key_maybe_key>>, peace_rt_model::Error> {
                let params_deserialized = peace_rt_model::WorkspaceInitializer::#params_deserialize_method_name::<
                    // <ParamsKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key,
                    #p_keys_key_maybe_key
                >(
                    storage,
                    // self.params_type_regs_builder.workspace_params_type_reg(),
                    params_type_regs_builder.#params_type_reg_method_name(),
                    #params_file_name,
                )
                .await?;

                Ok(params_deserialized)
            }
        }
    }
}
