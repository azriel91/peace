use syn::{punctuated::Punctuated, DeriveInput, Fields, Ident, Path, Variant};

use crate::util::{fields_deconstruct, fields_stmt_map, variant_match_arm};

pub fn is_usable_body(
    ast: &DeriveInput,
    params_field_wise_name: &Ident,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_is_usable(params_field_wise_name, fields, peace_params_path)
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_is_usable(params_field_wise_name, variants, peace_params_path)
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_is_usable(params_field_wise_name, &fields, peace_params_path)
        }
    }
}

/// Returns whether the fields within this struct all return `true` for
/// `is_usable`.
pub fn struct_fields_is_usable(
    params_field_wise_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    let fields_is_usable = fields_is_usable(fields, peace_params_path);
    let fields_deconstructed = fields_deconstruct(fields);

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // let #params_field_wise_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // } = self;
            //
            // let mut is_usable = true;
            // is_usable &= ValueSpecRt::is_usable(field_1);
            // is_usable &= ValueSpecRt::is_usable(field_2);
            //
            // is_usable
            // ```

            quote! {
                let #params_field_wise_name {
                    #(#fields_deconstructed),*
                } = self;

                let mut is_usable = true;
                #fields_is_usable

                is_usable
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // let #params_name(_0, _1, PhantomData,) = self;
            //
            // let mut is_usable = true;
            // is_usable &= ValueSpecRt::is_usable(_0);
            // is_usable &= ValueSpecRt::is_usable(_1);
            //
            // is_usable
            // ```

            quote! {
                let #params_field_wise_name(#(#fields_deconstructed),*) = self;

                let mut is_usable = true;
                #fields_is_usable

                is_usable
            }
        }
        Fields::Unit => quote!(true),
    }
}

/// Returns whether the fields within this enum all return `true` for
/// `is_usable`.
pub fn variants_is_usable(
    params_field_wise_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // ```rust
    // match self {
    //     ValueSpec::Variant1 => true,
    //     ValueSpec::Variant2(_0, _1, PhantomData) => {
    //         let mut is_usable = true;
    //         is_usable &= ValueSpecRt::is_usable(_0);
    //         is_usable &= ValueSpecRt::is_usable(_1);
    //         is_usable
    //     }
    //     ValueSpec::Variant3 {
    //         field_1,
    //         field_2,
    //         marker: PhantomData,
    //     } => {
    //         let mut is_usable = true;
    //         is_usable &= ValueSpecRt::is_usable(field_1);
    //         is_usable &= ValueSpecRt::is_usable(field_2);
    //         is_usable
    //     }
    // }
    // ```

    let variant_resolve_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let fields = &variant.fields;
                let fields_deconstructed = fields_deconstruct(fields);

                let variant_fields_is_usable = {
                    let fields_is_usable = fields_is_usable(fields, peace_params_path);

                    quote! {
                        let mut is_usable = true;
                        #fields_is_usable

                        is_usable
                    }
                };
                tokens.extend(variant_match_arm(
                    params_field_wise_name,
                    variant,
                    &fields_deconstructed,
                    variant_fields_is_usable,
                ));

                tokens
            });

    quote! {
        match self {
            #variant_resolve_arms
        }
    }
}

fn fields_is_usable(fields: &Fields, peace_params_path: &Path) -> proc_macro2::TokenStream {
    fields_stmt_map(fields, move |_field, field_name, _field_index| {
        quote! {
            is_usable &= #peace_params_path::ValueSpecRt::is_usable(#field_name);
        }
    })
    .fold(
        proc_macro2::TokenStream::new(),
        |mut tokens, next_tokens| {
            tokens.extend(next_tokens);
            tokens
        },
    )
}
