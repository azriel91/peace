use proc_macro2::Span;
use syn::{
    AngleBracketedGenericArguments, Attribute, Fields, GenericArgument, Ident, LitInt, Path,
    PathArguments, PathSegment, Type, TypePath, Variant,
};

/// Returns whether the type or field is defined outside the crate.
pub fn is_external(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("params") {
            let mut is_external = false;
            let _ = attr.parse_nested_meta(|parse_nested_meta| {
                is_external = parse_nested_meta.path.is_ident("external");
                Ok(())
            });

            is_external
        } else {
            false
        }
    })
}

/// Returns whether the attribute is a `#[serde(bound = "..")]` attribute.
pub fn is_serde_bound_attr(attr: &Attribute) -> bool {
    if attr.path().is_ident("serde") {
        let mut is_bound = false;
        let _ = attr.parse_nested_meta(|parse_nested_meta| {
            is_bound = parse_nested_meta.path.is_ident("bound");
            Ok(())
        });

        is_bound
    } else {
        false
    }
}

/// Returns whether the given field is a `PhantomData`.
pub fn is_phantom_data(field_ty: &Type) -> bool {
    matches!(&field_ty, Type::Path(TypePath { path, .. })
        if matches!(path.segments.last(), Some(segment) if segment.ident == "PhantomData"))
}

/// Returns tuple idents as `_n` where `n` is the index of the field.
pub fn tuple_ident_from_field_index(field_index: usize) -> Ident {
    Ident::new(&format!("_{field_index}"), Span::call_site())
}

/// Returns tuple idents as `_n` where `n` is the index of the field.
pub fn tuple_index_from_field_index(field_index: usize) -> LitInt {
    // Need to convert this to a `LitInt`,
    // because `quote` outputs a usize index as `0usize` instead of `0`
    LitInt::new(&format!("{field_index}"), Span::call_site())
}

/// Returns `MyType` for a given `path::to::MyType<T>`.
///
/// If the type is not a `TypePath`, then this returns `None`.
pub fn type_path_simple_name(ty: &Type) -> Option<&Ident> {
    let Type::Path(TypePath { path: Path {
        segments, ..
    }, .. }) = ty else {
        return None;
    };
    segments.last().map(|segment| &segment.ident)
}

/// Returns `MyType` for a given `path::to::MyType<T>`.
///
/// If the type is not a `TypePath`, then this returns `None`.
pub fn type_path_name_and_generics(ty: &Type) -> Option<(&Ident, &PathArguments)> {
    let Type::Path(TypePath { path: Path {
        segments, ..
    }, .. }) = ty else {
        return None;
    };
    segments
        .last()
        .map(|segment| (&segment.ident, &segment.arguments))
}

/// Returns a comma separated list of deconstructed fields.
///
/// Tuple fields are returned as `_n`, and marker fields are returned as
/// `::std::marker::PhantomData`.
pub fn fields_deconstruct(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    fields_deconstruct_retain(fields, false)
}

/// Returns a comma separated list of deconstructed fields, deconstructed as
/// `field: Some(field)`.
///
/// Tuple fields are returned as `Some(_n)`, and marker fields are returned as
/// `::std::marker::PhantomData`.
pub fn fields_deconstruct_some(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    fields_deconstruct_retain_map(fields, false, Some(|field_name| quote!(Some(#field_name))))
}

/// Returns a comma separated list of deconstructed fields, deconstructed as
/// `field: None`.
///
/// Tuple fields are returned as `None`, and marker fields are returned as
/// `::std::marker::PhantomData`.
pub fn fields_deconstruct_none(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    fields_deconstruct_retain_map(fields, false, Some(|_field_name| quote!(None)))
}

pub fn fields_deconstruct_retain(
    fields: &Fields,
    retain_phantom_data: bool,
) -> Vec<proc_macro2::TokenStream> {
    fields_deconstruct_retain_map(fields, retain_phantom_data, None)
}

fn fields_deconstruct_retain_map(
    fields: &Fields,
    retain_phantom_data: bool,
    fn_ident_map: Option<fn(&Ident) -> proc_macro2::TokenStream>,
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
                if let Some(fn_ident_map) = fn_ident_map {
                    let field_further_deconstructed = fn_ident_map(field_ident);
                    quote!(#field_ident: #field_further_deconstructed)
                } else {
                    quote!(#field_ident)
                }
            } else {
                let field_ident = tuple_ident_from_field_index(field_index);
                if let Some(fn_ident_map) = fn_ident_map {
                    fn_ident_map(&field_ident)
                } else {
                    quote!(#field_ident)
                }
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

/// Returns the reference `&Field` for any `Field`.
///
/// This includes special handling for the following types:
///
/// * `Option`: returns `Option<&Field>`.
/// * `PathBuf`: returns `&Path`.
/// * `Vec<T>`: returns `&[T]`.
/// * `String`: returns `&str`.
pub fn field_ty_to_ref_ty(field_ty: &Type, field_var: &proc_macro2::TokenStream) -> RefTypeAndExpr {
    if let Some((type_path, inner_args)) = type_path_simple(field_ty) {
        let type_name = &type_path.ident;
        if type_name == "Option" {
            if let Some(inner_args) = inner_args {
                if inner_args.args.len() == 1 {
                    if let Some(GenericArgument::Type(inner_type)) = inner_args.args.first() {
                        if let Some((inner_type_path, _inner_args)) = type_path_simple(inner_type) {
                            let inner_type_name = &inner_type_path.ident;
                            if inner_type_name == "String" {
                                return RefTypeAndExpr {
                                    ref_type: parse_quote!(Option<&str>),
                                    ref_mut_type: parse_quote!(Option<&mut str>),
                                    ref_expr: parse_quote!(self.#field_var.as_deref()),
                                    ref_mut_expr: parse_quote!(self.#field_var.as_deref_mut()),
                                };
                            } else if inner_type_name == "PathBuf" {
                                // It's more useful to return a `&mut PathBuf` instead of a `&mut
                                // Path`, as `PathBuf` has `.push()`.
                                return RefTypeAndExpr {
                                    ref_type: parse_quote!(Option<&Path>),
                                    ref_mut_type: parse_quote!(Option<&mut PathBuf>),
                                    ref_expr: parse_quote!(self.#field_var.as_deref()),
                                    ref_mut_expr: parse_quote!(self.#field_var.as_mut()),
                                };
                            }
                        }

                        return RefTypeAndExpr {
                            ref_type: parse_quote!(Option<&#inner_type>),
                            ref_mut_type: parse_quote!(Option<&mut #inner_type>),
                            ref_expr: parse_quote!(self.#field_var.as_ref()),
                            ref_mut_expr: parse_quote!(self.#field_var.as_mut()),
                        };
                    }
                }
            }
        } else if type_name == "PathBuf" {
            return RefTypeAndExpr {
                ref_type: parse_quote!(&::std::path::Path),
                ref_mut_type: parse_quote!(&mut ::std::path::PathBuf),
                ref_expr: parse_quote!(self.#field_var.as_str()),
                ref_mut_expr: parse_quote!(self.#field_var.as_mut_str()),
            };
        } else if type_name == "String" {
            return RefTypeAndExpr {
                ref_type: parse_quote!(&str),
                ref_mut_type: parse_quote!(&mut str),
                ref_expr: parse_quote!(self.#field_var.as_str()),
                ref_mut_expr: parse_quote!(self.#field_var.as_mut_str()),
            };
        }
    } else if let Type::Reference(_) = field_ty {
        return RefTypeAndExpr {
            ref_type: parse_quote!(#field_ty), // no `&` prefix
            ref_mut_type: parse_quote!(&mut #field_ty),
            ref_expr: parse_quote!(&self.#field_var),
            ref_mut_expr: parse_quote!(&mut self.#field_var),
        };
    }

    // By default, return `&#field_ty`.
    RefTypeAndExpr {
        ref_type: parse_quote!(&#field_ty),
        ref_mut_type: parse_quote!(&mut #field_ty),
        ref_expr: parse_quote!(&self.#field_var),
        ref_mut_expr: parse_quote!(&mut self.#field_var),
    }
}

fn type_path_simple(ty: &Type) -> Option<(&PathSegment, Option<&AngleBracketedGenericArguments>)> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let arguments = if let PathArguments::AngleBracketed(inner_args) = &segment.arguments {
                Some(inner_args)
            } else {
                None
            };

            return Some((segment, arguments));
        }
    }

    None
}

/// Type of reference to return, and the expression to obtain it from the
/// original object.
pub struct RefTypeAndExpr {
    /// e.g. `&Field`, `&str`
    pub ref_type: Type,
    /// e.g. `&mut Field`, `&mut str`
    pub ref_mut_type: Type,
    /// e.g. `&field`, `field.as_ref()`, `field.as_deref()`
    pub ref_expr: proc_macro2::TokenStream,
    /// e.g. `&mut field`, `field.as_mut()`
    pub ref_mut_expr: proc_macro2::TokenStream,
}
