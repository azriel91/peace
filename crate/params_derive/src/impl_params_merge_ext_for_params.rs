use syn::{
    punctuated::Punctuated, DeriveInput, Fields, Ident, ImplGenerics, Path, TypeGenerics, Variant,
    WhereClause,
};

use crate::util::{
    field_name_partial, fields_deconstruct, fields_deconstruct_partial, is_phantom_data,
    tuple_ident_from_field_index, variant_and_partial_match_arm,
};

/// `impl ParamsMergeExt for Params`, so that the framework can run
/// `params_example.merge(params_partial_current)` in `Item::try_state_*`
/// without needing to deconstruct the `Params::Partial`.
pub fn impl_params_merge_ext_for_params(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    params_name: &Ident,
    params_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let params_merge_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_merge_partial(params_name, params_partial_name, fields)
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_merge_partial(params_name, params_partial_name, variants)
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_merge_partial(params_name, params_partial_name, &fields)
        }
    };

    let mut generics_for_ref = ast.generics.clone();
    generics_for_ref.params.insert(0, parse_quote!('partial));

    quote! {
        impl #impl_generics #peace_params_path::ParamsMergeExt
        for #params_name #ty_generics
        #where_clause
        {
            fn merge(&mut self, params_partial: #params_partial_name #ty_generics) {
                #params_merge_body
            }
        }
    }
}

fn struct_fields_merge_partial(
    params_name: &Ident,
    params_partial_name: &Ident,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    let fields_deconstructed = fields_deconstruct(fields);
    let fields_deconstructed_partial = fields_deconstruct_partial(fields);
    let fields_merge_partial = fields_merge_partial(fields);

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // let #params_partial_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // } = self;
            //
            // let #params_partial_name {
            //     field_1: field_1_partial,
            //     field_2: field_2_partial,
            //     marker: PhantomData,
            // } = params_partial;
            //
            // if let Some(field_1_partial) = field_1_partial {
            //     *field_1 = field_1_partial;
            // }
            // if let Some(field_2_partial) = field_2_partial {
            //     *field_2 = field_2_partial;
            // }
            // ```
            quote! {
                let #params_name {
                    #(#fields_deconstructed),*
                } = self;
                let #params_partial_name {
                    #(#fields_deconstructed_partial),*
                } = params_partial;

                #fields_merge_partial
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // let #params_partial_name(
            //     _0,
            //     _1,
            //     PhantomData,
            // ) = self;
            //
            // let #params_partial_name(
            //     _0_partial,
            //     _1_partial,
            //     PhantomData,
            // ) = params_partial;
            //
            // if let Some(_0_partial) = _0_partial {
            //     *_0 = _0_partial;
            // }
            // if let Some(_1_partial) = _1_partial {
            //     *_1 = _1_partial;
            // }
            // ```
            quote! {
                let #params_name(#(#fields_deconstructed),*) = self;
                let #params_partial_name(#(#fields_deconstructed_partial),*) = params_partial;

                #fields_merge_partial
            }
        }
        Fields::Unit => proc_macro2::TokenStream::new(),
    }
}

fn variants_merge_partial(
    params_name: &Ident,
    params_partial_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // ```rust
    // match (self, params_partial) {
    //     (
    //         #params_name::Variant1,
    //         #params_partial_name::Variant1
    //     ) => {}
    //     (
    //         #params_name::Variant2(_0, _1, PhantomData),
    //         #params_partial_name::Variant2(_0_partial, _1_partial, PhantomData),
    //     ) => {
    //         if let Some(_0_partial) = _0_partial {
    //             *_0 = _0_partial;
    //         }
    //         if let Some(_1_partial) = _1_partial {
    //             *_1 = _1_partial;
    //         }
    //     }
    //     (
    //         #params_name::Variant3 {
    //             field_1,
    //             field_2,
    //             marker: PhantomData,
    //         },
    //         #params_partial_name::Variant3 {
    //             field_1: field_1_partial,
    //             field_2: field_2_partial,
    //             marker: PhantomData,
    //         },
    //     ) => {
    //         if let Some(field_1_partial) = field_1_partial {
    //             *field_1 = field_1_partial;
    //         }
    //         if let Some(field_2_partial) = field_2_partial {
    //             *field_2 = field_2_partial;
    //         }
    //     }
    //     _ => {} // Merging different variants is not supported.
    // }
    // ```

    let variant_merge_partial_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let fields_merge_partial = fields_merge_partial(&variant.fields);

                tokens.extend(variant_and_partial_match_arm(
                    params_name,
                    params_partial_name,
                    variant,
                    fields_merge_partial,
                ));

                tokens
            });

    quote! {
        match (self, params_partial) {
            #variant_merge_partial_arms

            _ => {} // Merging different variants is not supported.
        }
    }
}

fn fields_merge_partial(fields: &Fields) -> proc_macro2::TokenStream {
    fields
        .iter()
        .filter(|field| !is_phantom_data(&field.ty))
        .enumerate()
        .map(|(field_index, field)| {
            if let Some(field_ident) = field.ident.as_ref() {
                let field_name_partial = field_name_partial(field_ident);
                quote! {
                    if let Some(#field_name_partial) = #field_name_partial {
                        *#field_ident = #field_name_partial;
                    }
                }
            } else {
                let field_ident = tuple_ident_from_field_index(field_index);
                let field_name_partial = field_name_partial(&field_ident);
                quote! {
                    if let Some(#field_name_partial) = #field_name_partial {
                        *#field_ident = #field_name_partial;
                    }
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
}
