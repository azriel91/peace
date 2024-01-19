use quote::quote;

use crate::cmd::{CmdCtxBuilderReturnTypeBuilder, FlowCount, ScopeStruct};

/// Generates functions for the command context builder that are not constrained
/// by type parameters.
pub fn impl_common_fns(scope_struct: &ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();
    let scope_builder_name = &scope_struct.item_struct().ident;

    let return_type = CmdCtxBuilderReturnTypeBuilder::new(scope_builder_name.clone()).build();

    let mut common_fns = quote! {
        /// Sets the interrupt receiver and strategy so `CmdExecution`s can be interrupted.
        pub fn with_interruptibility(
            mut self,
            interruptibility: interruptible::Interruptibility<'static>,
        ) -> #return_type {
            let crate::ctx::CmdCtxBuilder {
                output,
                interruptibility: _,
                workspace,
                scope_builder,
            } = self;

            crate::ctx::CmdCtxBuilder {
                output,
                interruptibility,
                workspace,
                scope_builder,
            }
        }
    };

    if scope.flow_count() == FlowCount::One {
        common_fns.extend(quote! {
            /// Sets an item's parameters.
            ///
            /// Note: this **must** be called for each item in the flow.
            pub fn with_item_params<I>(
                mut self,
                item_id: peace_cfg::ItemId,
                params_spec: <I::Params<'_> as peace_params::Params>::Spec,
            ) -> Self
            where
                I: peace_cfg::Item,
                <CmdCtxBuilderTypeParamsT as crate::ctx::CmdCtxBuilderTypeParams>::AppError: From<I::Error>,
            {
                self.scope_builder.params_specs_provided.insert(item_id, params_spec);
                self
            }
        });
    };

    quote! {
        impl<'ctx, CmdCtxBuilderTypeParamsT> crate::ctx::CmdCtxBuilder<
            'ctx,
            CmdCtxBuilderTypeParamsT,
            #scope_builder_name<CmdCtxBuilderTypeParamsT>,
        >
        where
            CmdCtxBuilderTypeParamsT: crate::ctx::CmdCtxBuilderTypeParams,
            <CmdCtxBuilderTypeParamsT as crate::ctx::CmdCtxBuilderTypeParams>::AppError: peace_value_traits::AppError + From<peace_rt_model::Error>,
            <CmdCtxBuilderTypeParamsT as crate::ctx::CmdCtxBuilderTypeParams>::ParamsKeys:
                peace_rt_model::params::ParamsKeys,
        {
            #common_fns
        }
    }
}
