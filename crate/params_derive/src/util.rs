use proc_macro2::Span;
use syn::{Fields, Ident, Type, TypePath, Variant};

/// Returns whether the given field is a `PhantomData`.
pub fn is_phantom_data(field_ty: &Type) -> bool {
    matches!(&field_ty, Type::Path(TypePath { path, .. })
        if matches!(path.segments.last(), Some(segment) if segment.ident == "PhantomData"))
}

/// Returns tuple idents as `_n` where `n` is the index of the field.
pub fn tuple_ident_from_field_index(field_index: usize) -> Ident {
    Ident::new(&format!("_{field_index}"), Span::call_site())
}

/// Returns a comma separated list of deconstructed fields.
///
/// Tuple fields are returned as `_n`, and marker fields are returned as
/// `::std::marker::PhantomData`.
pub fn fields_deconstruct(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    fields_deconstruct_retain(fields, false)
}

pub fn fields_deconstruct_retain(
    fields: &Fields,
    retain_phantom_data: bool,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .enumerate()
        .map(|(field_index, field)| {
            if !retain_phantom_data && is_phantom_data(&field.ty) {
                if let Some(field_ident) = field.ident.as_ref() {
                    quote!(#field_ident: ::std::marker::PhantomData)
                } else {
                    quote!(::std::marker::PhantomData)
                }
            } else if let Some(field_ident) = field.ident.as_ref() {
                quote!(#field_ident)
            } else {
                let field_ident = tuple_ident_from_field_index(field_index);
                quote!(#field_ident)
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>()
}

/// Generates an enum variant match arm.
///
/// # Parameters
///
/// * `enum_name`: e.g. `MyParams`
/// * `variant`: Variant to generate the match arm for.
/// * `fields_deconstructed`: Deconstructed fields of the variant. See
///   [`fields_deconstruct`].
/// * `match_arm_body`: Tokens to insert as the match arm body.
pub fn variant_match_arm(
    enum_name: &Ident,
    variant: &Variant,
    fields_deconstructed: &[proc_macro2::TokenStream],
    match_arm_body: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let variant_name = &variant.ident;
    match &variant.fields {
        Fields::Named(_fields_named) => {
            quote! {
                #enum_name::#variant_name { #(#fields_deconstructed),* } => {
                    #match_arm_body
                }
            }
        }
        Fields::Unnamed(_) => {
            quote! {
                #enum_name::#variant_name(#(#fields_deconstructed),*) => {
                    #match_arm_body
                }
            }
        }
        Fields::Unit => {
            quote! {
                #enum_name::#variant_name => {
                    #match_arm_body
                }
            }
        }
    }
}
