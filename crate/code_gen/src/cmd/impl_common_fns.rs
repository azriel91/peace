use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, GenericArgument, Path, Token};

use crate::cmd::{type_parameters_impl, FlowCount, ScopeStruct};

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

    let common_fns = if scope.flow_count() == FlowCount::One {
        quote! {
            /// Sets an item's parameters.
            ///
            /// Note: this **must** be called for each item in the flow.
            pub fn with_item_params<IS>(
                mut self,
                item_id: peace_cfg::ItemId,
                params_spec: <IS::Params<'_> as peace_params::Params>::Spec,
            ) -> Self
            where
                IS: peace_cfg::Item,
                E: From<IS::Error>,
            {
                self.scope_builder.params_specs_provided.insert(item_id, params_spec);
                self
            }

            /// Sets the interrupt channel receiver so `CmdExecution`s can be interrupted.
            pub fn with_interruptibility<IS>(
                mut self,
                interrupt_rx: &'ctx mut tokio::sync::mpsc::Receiver<interruptible::InterruptSignal>,
                interrupt_strategy: IS,
            ) -> crate::ctx::CmdCtxBuilder<
                'ctx,
                O,
                interruptible::interruptibility::Interruptible<IS>,
                #scope_builder_name<
                    E,
                    // ProfileFromWorkspaceParam<'key, <PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
                    // FlowSelected<'ctx, E>,
                    // PKeys,
                    // WorkspaceParamsSome<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
                    // ProfileParamsSome<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
                    // FlowParamsNone,
                    #scope_builder_type_params
                >,
            >
            where
                IS: interruptible::InterruptStrategyT,
            {
                let crate::ctx::CmdCtxBuilder {
                    output,
                    interruptibility: _,
                    workspace,
                    scope_builder,
                } = self;

                let interruptibility = interruptible::interruptibility::Interruptible {
                    interrupt_rx,
                    interrupt_strategy,
                };

                crate::ctx::CmdCtxBuilder {
                    output,
                    interruptibility,
                    workspace,
                    scope_builder,
                }
            }
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    quote! {
        impl<
            'ctx,
            E,
            O,
            Interruptibility,
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
                Interruptibility,
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
            E: std::error::Error + From<peace_rt_model::Error> + 'static,
            Interruptibility: 'ctx,
            PKeys: #params_module::ParamsKeys + 'static,
        {
            #common_fns
        }
    }
}
