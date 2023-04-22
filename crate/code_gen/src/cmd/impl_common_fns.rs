use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, GenericArgument, Path, Token};

use crate::cmd::{type_parameters_impl, Scope, ScopeStruct};

/// Generates functions for the command context builder that are not constrained
/// by type parameters.
pub fn impl_common_fns(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;
    let params_module: Path = parse_quote!(peace_rt_model::params);

    let scope_builder_type_params = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();

        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);
        type_parameters_impl::params_selection_push(&mut type_params, scope);

        type_params
    };

    let common_fns = if scope == Scope::SingleProfileSingleFlow {
        quote! {
            /// Sets an item spec's parameters.
            ///
            /// Note: this **must** be called for each item spec in the flow.
            pub fn with_item_spec_params<IS>(
                mut self,
                item_spec_id: peace_cfg::ItemSpecId,
                param: IS::Params<'_>,
            ) -> Self
            where
                IS: peace_cfg::ItemSpec,
                E: From<IS::Error>
            {
                self.scope_builder.item_spec_params_provided.insert(item_spec_id, param);
                self
            }
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    quote! {
        impl<'ctx, E, O,
            // ProfileSelection,
            // FlowSelection,
            // PKeys,
            // WorkspaceParamsSelection,
            // ProfileParamsSelection,
            // FlowParamsSelection,
            #scope_builder_type_params
        >
            crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                // SingleProfileSingleFlowBuilder<
                #scope_builder_name<
                    E,
                    // ProfileSelection,
                    // FlowSelection,
                    // PKeys,
                    // WorkspaceParamsSelection,
                    // ProfileParamsSelection,
                    // FlowParamsSelection,
                    #scope_builder_type_params
                >,
            >
        where
            E: std::error::Error + From<peace_rt_model::Error>,
            PKeys: #params_module::ParamsKeys + 'static,
        {
            #common_fns
        }
    }
}
