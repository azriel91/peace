use syn::{Data, DeriveInput, Fields, Ident, ImplGenerics, Path, TypeGenerics, WhereClause};

use crate::util::fields_deconstruct_none;

/// Implement `Default` for the given type.
///
/// This avoids the unnecessary `Default` bound on type parameters from
/// `#[derive(Default)]`.
///
/// For enums, the variant annotated with `#[default]` on the original `Params`
/// struct will be used. If none are annotated with `#[default]`, then the last
/// variant is used as the default.
///
/// This does nothing for unions.
///
/// # Parameters
///
/// * `ast`: The `Params` type.
/// * `generics_split`: Generics of the `Params` type.
/// * `type_name`: Name of the type to generate.
pub fn impl_default(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    type_name: &Ident,
) -> Option<proc_macro2::TokenStream> {
    let (impl_generics, ty_generics, _where_clause) = generics_split;
    match &ast.data {
        Data::Struct(data_struct) => {
            let fields = &data_struct.fields;
            let impl_default_body = impl_default_body(&parse_quote!(#type_name), fields);

            let tokens = quote! {
                impl #impl_generics ::std::default::Default for #type_name #ty_generics {
                    fn default() -> Self {
                        #impl_default_body
                    }
                }
            };

            Some(tokens)
        }
        Data::Enum(data_enum) => {
            let variant_for_default = data_enum
                .variants
                .iter()
                .find(|variant| {
                    variant
                        .attrs
                        .iter()
                        .find(|attr| attr.path().is_ident("default"))
                        .is_some()
                })
                .or(data_enum.variants.last());

            if let Some(variant_for_default) = variant_for_default {
                let variant_name = &variant_for_default.ident;
                let fields = &variant_for_default.fields;
                let impl_default_body =
                    impl_default_body(&parse_quote!(#type_name::#variant_name), fields);

                let tokens = quote! {
                    impl #impl_generics ::std::default::Default for #type_name #ty_generics {
                        fn default() -> Self {
                            #impl_default_body
                        }
                    }
                };
                Some(tokens)
            } else {
                // Cannot implement default for an enum with no variants.
                None
            }
        }
        Data::Union(_) => None,
    }
}

fn impl_default_body(type_path: &Path, fields: &Fields) -> proc_macro2::TokenStream {
    let struct_fields_none = fields_deconstruct_none(fields);

    match fields {
        Fields::Named(_) => quote! {
            #type_path {
                #(#struct_fields_none),*
            }
        },
        Fields::Unnamed(_) => quote! {
            #type_path(#(#struct_fields_none),*)
        },
        Fields::Unit => quote!(#type_path),
    }
}
