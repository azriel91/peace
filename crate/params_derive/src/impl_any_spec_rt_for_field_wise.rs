use syn::{DeriveInput, Ident, ImplGenerics, Path, TypeGenerics, WhereClause};

use crate::{spec_is_usable::is_usable_body, spec_merge::spec_merge};

/// `impl AnySpecRt for ValueSpec`, so that Peace can tell if a spec is usable,
/// and merge provided and stored params together.
pub fn impl_any_spec_rt_for_field_wise(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    params_field_wise_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let is_usable_body = is_usable_body(ast, params_field_wise_name, peace_params_path);
    let spec_merge = spec_merge(ast, params_field_wise_name, peace_params_path);

    quote! {
        impl #impl_generics #peace_params_path::AnySpecRt
        for #params_field_wise_name #ty_generics
        #where_clause
        {
            fn is_usable(&self) -> bool {
                #is_usable_body
            }

            #spec_merge
        }
    }
}
