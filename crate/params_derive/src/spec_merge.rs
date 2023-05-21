use syn::{punctuated::Punctuated, DeriveInput, Fields, Ident, Path, Variant};

use crate::util::{
    fields_deconstruct, fields_deconstruct_rename_other, fields_stmt_map, variant_match_arm,
};

pub fn spec_merge(
    ast: &DeriveInput,
    params_field_wise_name: &Ident,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    let merge_logic = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_spec_merge(params_field_wise_name, fields, peace_params_path)
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_spec_merge(params_field_wise_name, variants, peace_params_path)
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_spec_merge(params_field_wise_name, &fields, peace_params_path)
        }
    };

    quote! {
        fn merge(&mut self, other_boxed: &#peace_params_path::AnySpecRtBoxed)
        where
            Self: Sized,
        {
            let other: Option<&Self> = other_boxed.downcast_ref();
            let Some(other) = other else {
                let self_ty_name = tynm::type_name::<Self>();
                panic!("Failed to downcast value into `{self_ty_name}`. Value: `{other_boxed:#?}`.");
            };

            #merge_logic
        }
    }
}

/// Deep merges spec fields within this struct.
pub fn struct_fields_spec_merge(
    params_field_wise_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    let fields_spec_merge = fields_spec_merge(fields, peace_params_path);
    let fields_deconstructed = fields_deconstruct(fields);
    let fields_deconstructed_other = fields_deconstruct_rename_other(fields);

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
            // let #params_field_wise_name {
            //     field_1: field_1_other,
            //     field_2: field_2_other,
            //     marker: PhantomData,
            // } = other;
            //
            // AnySpecRt::merge(field_1, field_1_other);
            // AnySpecRt::merge(field_2, field_2_other);
            // ```

            quote! {
                let #params_field_wise_name {
                    #(#fields_deconstructed),*
                } = self;

                let #params_field_wise_name {
                    #(#fields_deconstructed_other),*
                } = other;

                #fields_spec_merge
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // let #params_name(_0, _1, PhantomData,) = self;
            // let #params_name(_0_other, _1_other, PhantomData,) = other;
            //
            // AnySpecRt::merge(_0, _0_other);
            // AnySpecRt::merge(_1, _1_other);
            // ```

            quote! {
                let #params_field_wise_name(#(#fields_deconstructed),*) = self;
                let #params_field_wise_name(#(#fields_deconstructed_other),*) = other;

                #fields_spec_merge

                spec_merge
            }
        }
        Fields::Unit => quote!(),
    }
}

/// Deep merges spec fields within this enum.
pub fn variants_spec_merge(
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
    //         AnySpecRt::merge(_0, _0_other);
    //         AnySpecRt::merge(_1, _1_other);
    //     }
    //     ValueSpec::Variant3 {
    //         field_1,
    //         field_2,
    //         marker: PhantomData,
    //     } => {
    //         AnySpecRt::merge(field_1, field_1_other);
    //         AnySpecRt::merge(field_2, field_2_other);
    //     }
    // }
    // ```

    let variant_resolve_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let fields = &variant.fields;
                let fields_deconstructed = fields_deconstruct(fields);

                let variant_fields_spec_merge = {
                    let fields_spec_merge = fields_spec_merge(fields, peace_params_path);

                    quote! {
                        let mut spec_merge = true;
                        #fields_spec_merge

                        spec_merge
                    }
                };
                tokens.extend(variant_match_arm(
                    params_field_wise_name,
                    variant,
                    &fields_deconstructed,
                    variant_fields_spec_merge,
                ));

                tokens
            });

    quote! {
        match self {
            #variant_resolve_arms
        }
    }
}

fn fields_spec_merge(fields: &Fields, peace_params_path: &Path) -> proc_macro2::TokenStream {
    fields_stmt_map(fields, move |_field, field_name, _field_index| {
        let field_name_other = format_ident!("{}_other", field_name);
        quote! {
            #peace_params_path::AnySpecRt::merge(#field_name, #field_name_other);
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
