use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, DeriveInput, Fields, Ident, ImplGenerics, LitInt, Path, TypeGenerics,
    Variant, WhereClause,
};

use crate::util::{
    fields_deconstruct, is_phantom_data, tuple_ident_from_field_index, variant_match_arm,
};

/// `impl From<Params> for ParamsSpec`, so that users can provide
/// `params.into()` when building a cmd_ctx, instead of constructing a
/// `ParamsSpec`.
pub fn impl_from_params_for_params_spec(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    params_name: &Ident,
    params_spec_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let from_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_map_to_value(params_spec_name, fields, peace_params_path)
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_map_to_value(params_spec_name, params_name, variants, peace_params_path)
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_map_to_value(params_spec_name, &fields, peace_params_path)
        }
    };

    quote! {
        impl #impl_generics From<#params_name #ty_generics>
        for #params_spec_name #ty_generics
        #where_clause
        {
            fn from(params: #params_name #ty_generics) -> Self {
                #from_body
            }
        }
    }
}

fn struct_fields_map_to_value(
    params_spec_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // #params_spec_name {
            //     field_1: #peace_params_path::ValueSpec::Value(params.field_1),
            //     field_2: #peace_params_path::ValueSpec::Value(params.field_2),
            //     marker: PhantomData,
            // }

            let fields_map_to_value = fields_named.named.iter().fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, field| {
                    if let Some(field_name) = field.ident.as_ref() {
                        if is_phantom_data(&field.ty) {
                            tokens.extend(quote! {
                                #field_name: std::marker::PhantomData,
                            });
                        } else {
                            tokens.extend(quote! {
                                #field_name: #peace_params_path::ValueSpec::Value(params.#field_name),
                            });
                        }
                    }
                    tokens
                },
            );

            quote! {
                #params_spec_name {
                    #fields_map_to_value
                }
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // #params_spec_name(#peace_params_path::ValueSpec::Value(params.0),
            // #peace_params_path::ValueSpec::Value(params.1), PhantomData)
            let fields_map_to_value = fields_unnamed.unnamed.iter().enumerate().fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, (field_index, field)| {
                    // Need to convert this to a `LitInt`,
                    // because `quote` outputs the index as `0usize` instead of `0`
                    let field_index = LitInt::new(&format!("{field_index}"), Span::call_site());

                    if is_phantom_data(&field.ty) {
                        tokens.extend(quote!(std::marker::PhantomData,));
                    } else {
                        tokens.extend(
                            quote!(#peace_params_path::ValueSpec::Value(params.#field_index),),
                        );
                    }

                    tokens
                },
            );

            quote! {
                #params_spec_name(#fields_map_to_value)
            }
        }
        Fields::Unit => quote!(#params_spec_name),
    }
}

fn variants_map_to_value(
    params_spec_name: &Ident,
    params_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // match params {
    //     Params::Variant1 => Params::Variant1,
    //     Params::Variant2(_0, _1, PhantomData) => {
    //         Params::Variant2(
    //              #peace_params_path::ValueSpec::Value(_0),
    //              #peace_params_path::ValueSpec::Value(_1),
    //              PhantomData,
    //         )
    //     }
    //     Params::Variant3 { field_1, field_2, marker: PhantomData } => {
    //         Params::Variant3 {
    //             field_1: #peace_params_path::ValueSpec::Value(field_1),
    //             field_2: #peace_params_path::ValueSpec::Value(field_2),
    //             marker: PhantomData,
    //         }
    //     }
    // }

    let variant_map_to_value_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let variant_fields = fields_deconstruct(&variant.fields);
                let variant_fields_map_to_value = variant_fields_map_to_value(
                    params_spec_name,
                    &variant.ident,
                    &variant.fields,
                    peace_params_path,
                );
                tokens.extend(variant_match_arm(
                    params_name,
                    variant,
                    &variant_fields,
                    variant_fields_map_to_value,
                ));

                tokens
            });

    quote! {
        match params {
            #variant_map_to_value_arms
        }
    }
}

fn variant_fields_map_to_value(
    params_spec_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // ParamsSpecName {
            //     field_1: #peace_params_path::ValueSpec::Value(field_1),
            //     field_2: #peace_params_path::ValueSpec::Value(field_2),
            //     marker: PhantomData,
            // }

            let fields_map_to_value = fields_named.named.iter().fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, field| {
                    if let Some(field_name) = field.ident.as_ref() {
                        if is_phantom_data(&field.ty) {
                            tokens.extend(quote! {
                                #field_name: std::marker::PhantomData,
                            });
                        } else {
                            tokens.extend(quote! {
                                #field_name: #peace_params_path::ValueSpec::Value(#field_name),
                            });
                        }
                    }
                    tokens
                },
            );

            quote! {
                #params_spec_name::#variant_name {
                    #fields_map_to_value
                }
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // ParamsSpecName(_0, #peace_params_path::ValueSpec::Value(_1), PhantomData)
            let fields_map_to_value = fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(field_index, field)| (tuple_ident_from_field_index(field_index), field))
                .fold(
                    proc_macro2::TokenStream::new(),
                    |mut tokens, (field_index, field)| {
                        if is_phantom_data(&field.ty) {
                            tokens.extend(quote!(std::marker::PhantomData,));
                        } else {
                            tokens.extend(
                                quote!(#peace_params_path::ValueSpec::Value(#field_index),),
                            );
                        }

                        tokens
                    },
                );

            quote! {
                #params_spec_name::#variant_name(#fields_map_to_value)
            }
        }
        Fields::Unit => quote!(Self::#variant_name),
    }
}
