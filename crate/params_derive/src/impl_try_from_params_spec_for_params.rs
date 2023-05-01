use syn::{
    punctuated::Punctuated, DeriveInput, Fields, Ident, ImplGenerics, TypeGenerics, Variant,
    WhereClause,
};

use crate::util::{fields_deconstruct, fields_deconstruct_some, variant_match_arm};

/// `impl TryFrom<ParamsPartial> for Params`, so that users can use
/// `params_partial.try_into()` in `ItemSpec::try_state_*` without needing to
/// deconstruct the `Params::Partial`.
pub fn impl_try_from_params_spec_for_params(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_name: &Ident,
    params_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let try_from_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_try_map_from_value(params_name, params_partial_name, fields)
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_try_map_from_value(params_name, params_partial_name, variants)
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_try_map_from_value(params_name, params_partial_name, &fields)
        }
    };

    quote! {
        impl #impl_generics ::std::convert::TryFrom<#params_partial_name #ty_generics>
        for #params_name #ty_generics
        #where_clause
        {
            type Error = #params_partial_name #ty_generics;

            fn try_from(params_partial: #params_partial_name #ty_generics) -> Result<Self, Self::Error> {
                #try_from_body
            }
        }
    }
}

fn struct_fields_try_map_from_value(
    params_name: &Ident,
    params_partial_name: &Ident,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    let fields_deconstructed = fields_deconstruct(fields);
    let fields_deconstructed_some = fields_deconstruct_some(fields);

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // if let #params_partial_name {
            //     field_1: Some(field_1),
            //     field_2: Some(field_2),
            //     marker: PhantomData,
            // } = params_partial {
            //     Ok(#params_name {
            //         field_1,
            //         field_2,
            //         marker: PhantomData,
            //     })
            // } else {
            //     Err(params_partial)
            // }
            // ```
            quote! {
                if let #params_partial_name {
                    #(#fields_deconstructed_some),*
                } = params_partial {
                    Ok(#params_name { #(#fields_deconstructed),* })
                } else {
                    Err(params_partial)
                }
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // if let #params_partial_name(
            //     Some(field_1),
            //     Some(field_2),
            //     PhantomData,
            // ) = params_partial {
            //     Ok(#params_name(
            //         field_1,
            //         field_2,
            //         PhantomData,
            //     ))
            // } else {
            //     Err(params_partial)
            // }
            // ```
            quote! {
                if let #params_partial_name(
                    #(#fields_deconstructed_some),*
                ) = params_partial {
                    Ok(#params_name(#(#fields_deconstructed),*))
                } else {
                    Err(params_partial)
                }
            }
        }
        Fields::Unit => quote!(Ok(#params_name)),
    }
}

fn variants_try_map_from_value(
    params_name: &Ident,
    params_partial_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // ```rust
    // match params_partial {
    //     #params_partial_name::Variant1 => Ok(#params_name::Variant1),
    //     #params_partial_name::Variant2(Some(_0), Some(_1), PhantomData) => {
    //         #fields_clone
    //
    //         Ok(#params_name::Variant2(
    //              _0,
    //              _1,
    //              PhantomData,
    //         ))
    //     }
    //     #params_partial_name::Variant3 {
    //         field_1: Some(field_1),
    //         field_2: Some(field_2),
    //         marker: PhantomData,
    //     } => {
    //         #fields_clone
    //
    //         Ok(#params_name::Variant3 {
    //             field_1,
    //             field_2,
    //             marker: PhantomData,
    //         })
    //     }
    //     _ => Err(params_partial),
    // }
    // ```

    let variant_try_map_from_value_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let variant_fields = fields_deconstruct_some(&variant.fields);
                let variant_fields_try_map_from_value =
                    variant_fields_try_map_from_value(params_name, &variant.ident, &variant.fields);
                tokens.extend(variant_match_arm(
                    params_partial_name,
                    variant,
                    &variant_fields,
                    variant_fields_try_map_from_value,
                ));

                tokens
            });

    quote! {
        match params_partial {
            #variant_try_map_from_value_arms

            _ => Err(params_partial),
        }
    }
}

fn variant_fields_try_map_from_value(
    params_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    let fields_deconstructed = fields_deconstruct(fields);
    match fields {
        Fields::Named(_fields_named) => {
            quote! {
                Ok(#params_name::#variant_name {
                    #(#fields_deconstructed),*
                })
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            quote! {
                Ok(#params_name::#variant_name(#(#fields_deconstructed),*))
            }
        }
        Fields::Unit => quote!(Ok(Self::#variant_name)),
    }
}
