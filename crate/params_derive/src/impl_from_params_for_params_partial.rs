use syn::{
    punctuated::Punctuated, Data, DeriveInput, Fields, Ident, ImplGenerics, TypeGenerics, Variant,
    WhereClause,
};

use crate::util::{fields_deconstruct, fields_deconstruct_some, variant_match_arm};

/// `impl From<Params> for ParamsPartial`, so that users can use
/// `params_partial.try_into()` in `ItemSpec::try_state_*` without needing to
/// deconstruct the `Params::Partial`.
pub fn impl_from_params_for_params_partial(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_name: &Ident,
    params_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let from_body = match &ast.data {
        Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_map_from_value(params_name, params_partial_name, fields)
        }
        Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_map_from_value(params_name, params_partial_name, variants)
        }
        Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_map_from_value(params_name, params_partial_name, &fields)
        }
    };

    quote! {
        impl #impl_generics ::std::convert::From<#params_name #ty_generics>
        for #params_partial_name #ty_generics
        #where_clause
        {
            fn from(params: #params_name #ty_generics) -> Self {
                #from_body
            }
        }
    }
}

fn struct_fields_map_from_value(
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
            // let #params_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // }
            //
            // #params_partial_name {
            //     field_1: Some(field_1),
            //     field_2: Some(field_2),
            //     marker: PhantomData,
            // }
            // ```
            quote! {
                let #params_name { #(#fields_deconstructed),* } = params;

                #params_partial_name { #(#fields_deconstructed_some),* }
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // let #params_name(field_1, field_2, PhantomData,) = params;
            //
            // #params_partial_name(Some(field_1), Some(field_2), PhantomData,)
            // ```
            quote! {
                let #params_name( #(#fields_deconstructed),* ) = params;

                #params_partial_name(#(#fields_deconstructed_some),*)
            }
        }
        Fields::Unit => quote!(#params_partial_name),
    }
}

fn variants_map_from_value(
    params_name: &Ident,
    params_partial_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // ```rust
    // match params {
    //     #params_name::Variant1 => #params_partial_name::Variant1,
    //     #params_name::Variant2(_0, _1, PhantomData) => {
    //         #fields_to_owned
    //
    //         #params_partial_name::Variant2(
    //              Some(_0),
    //              Some(_1),
    //              PhantomData,
    //         )
    //     }
    //     #params_name::Variant3 {
    //         field_1,
    //         field_2,
    //         marker: PhantomData,
    //     } => {
    //         #fields_to_owned
    //
    //         #params_partial_name::Variant3 {
    //             Some(field_1),
    //             Some(field_2),
    //             marker: PhantomData,
    //         }
    //     }
    // }
    // ```

    let variant_map_from_value_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let variant_fields = fields_deconstruct(&variant.fields);
                let variant_fields_map_from_value = variant_fields_map_from_value(
                    params_partial_name,
                    &variant.ident,
                    &variant.fields,
                );
                tokens.extend(variant_match_arm(
                    params_name,
                    variant,
                    &variant_fields,
                    variant_fields_map_from_value,
                ));

                tokens
            });

    quote! {
        match params {
            #variant_map_from_value_arms
        }
    }
}

fn variant_fields_map_from_value(
    params_partial_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    let fields_deconstructed_some = fields_deconstruct_some(fields);

    match fields {
        Fields::Named(_fields_named) => {
            quote! {
                #params_partial_name::#variant_name {
                    #(#fields_deconstructed_some),*
                }
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            quote! {
                #params_partial_name::#variant_name(#(#fields_deconstructed_some),*)
            }
        }
        Fields::Unit => quote!(Self::#variant_name),
    }
}
