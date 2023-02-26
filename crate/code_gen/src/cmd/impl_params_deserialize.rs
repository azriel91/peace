use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, GenericArgument, Path, Token};

use crate::cmd::{
    type_parameters_impl,
    type_params_selection::{
        FlowParamsSelection, ProfileParamsSelection, WorkspaceParamsSelection,
    },
    ParamsScope, ScopeStruct,
};

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
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::cmd_context_params);

    let (workspace_params_selection, profile_params_selection, flow_params_selection) =
        match params_scope {
            ParamsScope::Workspace => (
                WorkspaceParamsSelection::Some.type_param(),
                parse_quote!(ProfileParamsSelection),
                parse_quote!(FlowParamsSelection),
            ),
            ParamsScope::Profile => (
                parse_quote!(WorkspaceParamsSelection),
                ProfileParamsSelection::Some.type_param(),
                parse_quote!(FlowParamsSelection),
            ),
            ParamsScope::Flow => (
                parse_quote!(WorkspaceParamsSelection),
                parse_quote!(ProfileParamsSelection),
                FlowParamsSelection::Some.type_param(),
            ),
        };

    let impl_type_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);

        match params_scope {
            ParamsScope::Workspace => {
                if scope.profile_params_supported() {
                    type_params.push(profile_params_selection.clone());
                }

                if scope.flow_params_supported() {
                    type_params.push(flow_params_selection.clone());
                }
            }
            ParamsScope::Profile => {
                type_params.push(workspace_params_selection.clone());

                if scope.flow_params_supported() {
                    type_params.push(flow_params_selection.clone());
                }
            }
            ParamsScope::Flow => {
                type_params.push(workspace_params_selection.clone());

                if scope.profile_params_supported() {
                    type_params.push(profile_params_selection.clone());
                }
            }
        }

        type_params
    };
    let scope_builder_type_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();
        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);

        type_params.push(workspace_params_selection);
        if scope.profile_params_supported() {
            type_params.push(profile_params_selection);
        }
        if scope.flow_params_supported() {
            type_params.push(flow_params_selection);
        }

        type_params
    };
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

    quote! {
        impl<
            'ctx,
            'key,
            PKeys,
            // ProfileSelection,
            // FlowSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            #impl_type_params
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #scope_builder_name<
                    // ProfileSelection,
                    // FlowSelection,
                    // WorkspaceParamsSome<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_builder_type_params
                >,
                PKeys,
            >
        where
            PKeys: #params_module::ParamsKeys + 'static,
        {
            #[doc = #doc_summary]
            // async fn workspace_params_deserialize
            async fn #params_deserialize_method_name(
                storage: &peace_rt_model::Storage,
                params_type_regs_builder: &#params_module::ParamsTypeRegsBuilder<PKeys>,
                // workspace_params_file: &peace_resources::internal::WorkspaceParamsFile,
                #params_file_name: &peace_resources::internal::#params_file_type,
            // ) -> Result<Option<WorkspaceParams<K>, peace_rt_model::Error> {
            ) -> Result<Option<#params_module::#params_map_type<#p_keys_key_maybe_key>>, peace_rt_model::Error> {
                let params_deserialized = peace_rt_model::WorkspaceInitializer::#params_deserialize_method_name::<
                    // <PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key,
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
