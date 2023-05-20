use syn::{DeriveInput, Ident, ImplGenerics, Path, TypeGenerics, WhereClause};

use crate::spec_is_usable::is_usable_body;

/// `impl FieldWiseSpecRt for ValueSpec`, so that Peace can resolve the params
/// type as well as its values from the spec.
pub fn impl_field_wise_spec_rt_for_field_wise_external(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    peace_resources_path: &Path,
    params_name: &Ident,
    params_field_wise_name: &Ident,
    params_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let is_usable_body = is_usable_body(ast, params_field_wise_name, peace_params_path);

    quote! {
        impl #impl_generics #peace_params_path::FieldWiseSpecRt
        for #params_field_wise_name #ty_generics
        #where_clause
        {
            type ValueType = #params_name #ty_generics;
            type Partial = #params_partial_name #ty_generics;

            fn resolve(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>,
                value_resolution_ctx: &mut #peace_params_path::ValueResolutionCtx,
            ) -> Result<#params_name #ty_generics, #peace_params_path::ParamsResolveError> {
                if let Some(params) = self.0.as_ref() {
                    Ok(params.clone())
                } else {
                    match resources.try_borrow::<#params_name #ty_generics>() {
                        Ok(t) => Ok((&*t).clone()),
                        Err(borrow_fail) => match borrow_fail {
                            #peace_resources_path::BorrowFail::ValueNotFound => {
                                Err(#peace_params_path::ParamsResolveError::InMemory {
                                    value_resolution_ctx: value_resolution_ctx.clone(),
                                })
                            }
                            #peace_resources_path::BorrowFail::BorrowConflictImm |
                            #peace_resources_path::BorrowFail::BorrowConflictMut => {
                                Err(#peace_params_path::ParamsResolveError::InMemoryBorrowConflict {
                                    value_resolution_ctx: value_resolution_ctx.clone(),
                                })
                            }
                        },
                    }
                }
            }

            fn resolve_partial(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>,
                value_resolution_ctx: &mut #peace_params_path::ValueResolutionCtx,
            ) -> Result<#params_partial_name #ty_generics, #peace_params_path::ParamsResolveError> {
                if let Some(params) = self.0.as_ref() {
                    Ok(params.clone().into())
                } else {
                    match resources.try_borrow::<#params_name #ty_generics>() {
                        Ok(t) => Ok((&*t).clone().into()),
                        Err(borrow_fail) => match borrow_fail {
                            #peace_resources_path::BorrowFail::ValueNotFound => {
                                Err(#peace_params_path::ParamsResolveError::InMemory {
                                    value_resolution_ctx: value_resolution_ctx.clone(),
                                })
                            }
                            #peace_resources_path::BorrowFail::BorrowConflictImm |
                            #peace_resources_path::BorrowFail::BorrowConflictMut => {
                                Err(#peace_params_path::ParamsResolveError::InMemoryBorrowConflict {
                                    value_resolution_ctx: value_resolution_ctx.clone(),
                                })
                            }
                        },
                    }
                }
            }

            fn is_usable(&self) -> bool {
                #is_usable_body
            }
        }
    }
}
