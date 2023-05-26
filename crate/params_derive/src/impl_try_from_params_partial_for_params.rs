use syn::{
    punctuated::Punctuated, DeriveInput, Fields, Ident, ImplGenerics, TypeGenerics, Variant,
    WhereClause,
};

use crate::util::{
    fields_deconstruct, fields_deconstruct_some, is_phantom_data, tuple_ident_from_field_index,
    variant_match_arm,
};

/// `impl TryFrom<ParamsPartial> for Params`, so that users can use
/// `params_partial.try_into()` in `Item::try_state_*` without needing to
/// deconstruct the `Params::Partial`.
pub fn impl_try_from_params_partial_for_params(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_name: &Ident,
    params_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let try_from_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_try_map_from_value(
                params_name,
                params_partial_name,
                fields,
                TryFromMode::Owned,
            )
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_try_map_from_value(
                params_name,
                params_partial_name,
                variants,
                TryFromMode::Owned,
            )
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_try_map_from_value(
                params_name,
                params_partial_name,
                &fields,
                TryFromMode::Owned,
            )
        }
    };

    let try_from_ref_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_try_map_from_value(
                params_name,
                params_partial_name,
                fields,
                TryFromMode::Ref,
            )
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_try_map_from_value(
                params_name,
                params_partial_name,
                variants,
                TryFromMode::Ref,
            )
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_try_map_from_value(
                params_name,
                params_partial_name,
                &fields,
                TryFromMode::Ref,
            )
        }
    };

    let mut generics_for_ref = ast.generics.clone();
    generics_for_ref.params.insert(0, parse_quote!('partial));
    let (impl_generics_for_ref, _type_generics, _where_clause) = generics_for_ref.split_for_impl();

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

        impl #impl_generics_for_ref ::std::convert::TryFrom<&'partial #params_partial_name #ty_generics>
        for #params_name #ty_generics
        #where_clause
        {
            type Error = &'partial #params_partial_name #ty_generics;

            fn try_from(params_partial: &'partial #params_partial_name #ty_generics) -> Result<Self, Self::Error> {
                #try_from_ref_body
            }
        }
    }
}

fn struct_fields_try_map_from_value(
    params_name: &Ident,
    params_partial_name: &Ident,
    fields: &Fields,
    try_from_mode: TryFromMode,
) -> proc_macro2::TokenStream {
    let fields_deconstructed = fields_deconstruct(fields);
    let fields_deconstructed_some = fields_deconstruct_some(fields);
    let fields_to_owned = try_from_mode.fields_to_owned(fields);

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
            //     #fields_to_owned
            //
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
                    #fields_to_owned

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
            //     #fields_to_owned
            //
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
                    #fields_to_owned

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
    try_from_mode: TryFromMode,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // ```rust
    // match params_partial {
    //     #params_partial_name::Variant1 => Ok(#params_name::Variant1),
    //     #params_partial_name::Variant2(Some(_0), Some(_1), PhantomData) => {
    //         #fields_to_owned
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
    //         #fields_to_owned
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
                let variant_fields_try_map_from_value = variant_fields_try_map_from_value(
                    params_name,
                    &variant.ident,
                    &variant.fields,
                    try_from_mode,
                );
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
    try_from_mode: TryFromMode,
) -> proc_macro2::TokenStream {
    let fields_deconstructed = fields_deconstruct(fields);
    let fields_to_owned = try_from_mode.fields_to_owned(fields);

    match fields {
        Fields::Named(_fields_named) => {
            quote! {
                #fields_to_owned

                Ok(#params_name::#variant_name {
                    #(#fields_deconstructed),*
                })
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            quote! {
                #fields_to_owned

                Ok(#params_name::#variant_name(#(#fields_deconstructed),*))
            }
        }
        Fields::Unit => quote!(Ok(Self::#variant_name)),
    }
}

/// Whether code is generated for an owned or borrowed `Params::Partial`.
#[derive(Clone, Copy, Debug)]
enum TryFromMode {
    /// `Params::Partial` is owned.
    Owned,
    /// `Params::Partial` is borrowed.
    Ref,
}

impl TryFromMode {
    /// Returns `to_owned()` statements for each field for a borrowed
    /// `Params::Partial`.
    ///
    /// Generates:
    ///
    /// ```rust
    /// // may need `to_vec()` or `to_string()` for `&[]` and `str`.
    /// let field_1 = ::std::borrow::ToOwned(field_1);
    /// let field_2 = ::std::borrow::ToOwned(field_2);
    /// ```
    fn fields_to_owned(self, fields: &Fields) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Owned => None,
            Self::Ref => {
                let tokens = fields
                    .iter()
                    .enumerate()
                    .filter(|(_field_index, field)| !is_phantom_data(&field.ty))
                    .fold(
                        proc_macro2::TokenStream::new(),
                        |mut tokens, (field_index, field)| {
                            if let Some(field_name) = field.ident.as_ref() {
                                tokens.extend(quote! {
                                    let #field_name = ::std::borrow::ToOwned::to_owned(#field_name);
                                });
                            } else {
                                let field_name = tuple_ident_from_field_index(field_index);
                                tokens.extend(quote! {
                                    let #field_name = ::std::borrow::ToOwned::to_owned(#field_name);
                                });
                            }
                            tokens
                        },
                    );

                Some(tokens)
            }
        }
    }
}
