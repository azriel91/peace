use proc_macro2::Span;
use syn::{
    meta::ParseNestedMeta, punctuated::Punctuated, AngleBracketedGenericArguments, Attribute,
    DeriveInput, Field, Fields, GenericArgument, GenericParam, Generics, Ident, LitInt, Path,
    PathArguments, PathSegment, Type, TypePath, Variant, WherePredicate,
};

// Remember to update `params/src/std_impl.rs` when updating this.
//
// This can be replaced by `std::cell::OnceCell` when Rust 1.70.0 is released.
static STD_LIB_TYPES: &[&str] = &[
    "bool",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "usize",
    "isize",
    "String",
    "PathBuf",
    #[cfg(not(target_arch = "wasm32"))]
    "OsString",
    "Option",
    "Vec",
];

/// Returns whether the type is annotated with `#[params(external)]`, which
/// means its spec should be fieldless.
///
/// This attribute must be:
///
/// * attached to std library types defined outside the `peace_params` crate.
/// * attached to each `Params`' field defined outside the item spec crate.
pub fn is_external_type(ast: &DeriveInput) -> bool {
    is_known_fieldless_std_lib_spec(&ast.ident) || is_external(&ast.attrs)
}

/// Returns whether the field is annotated with `#[params(external)]`, which
/// means its spec should be fieldless.
///
/// This attribute must be:
///
/// * attached to std library types defined outside the `peace_params` crate.
/// * attached to each `Params`' field defined outside the item spec crate.
pub fn is_external_field(field: &Field) -> bool {
    is_known_fieldless_type_spec(&field.ty) || is_external(&field.attrs)
}

/// Returns if the given `Type`'s spec should be fieldless.
///
/// This applies to std library types, as well as non-`Path` types.
fn is_known_fieldless_type_spec(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => segments
            .last()
            .map(|path_segment| is_known_fieldless_std_lib_spec(&path_segment.ident))
            .unwrap_or(false),

        // If we cannot detect the type, we don't generate a fieldwise spec for it.
        //
        // Type::Array(_) |
        // Type::BareFn(_) |
        // Type::Group(_) |
        // Type::ImplTrait(_) |
        // Type::Infer(_) |
        // Type::Macro(_) |
        // Type::Never(_) |
        // Type::Paren(_) |
        // Type::Ptr(_) => |
        // Type::Reference(_) => |
        // Type::Slice(_) => |
        // Type::TraitObject(_) => |
        // Type::Tuple(_) => |
        // Type::Verbatim(_) => |
        _ => true,
    }
}

/// Returns if the given `Type`'s spec should be fieldless.
///
/// This applies to std library types, as well as non-`Path` types.
fn is_known_fieldless_std_lib_spec(ty_name: &Ident) -> bool {
    STD_LIB_TYPES
        .iter()
        .any(|std_lib_type| ty_name == std_lib_type)
}

/// Returns whether any of the attributes contains `#[params(external)]`.
///
/// This attribute should be:
///
/// * attached to std library types defined outside the `peace_params` crate, if
///   it isn't already covered by `STD_LIB_TYPES`.
/// * attached to each field in `Params` that is defined outside the item spec
///   crate.
fn is_external(attrs: &[Attribute]) -> bool {
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

/// Returns the field wrapper generics to use, which is the intersection of the
/// field type arguments and the parent type arguments.
pub fn field_wrapper_generics(
    parent_type_generics: Option<&Generics>,
    field_generics: &PathArguments,
) -> Option<proc_macro2::TokenStream> {
    parent_type_generics.and_then(|parent_type_generics| match field_generics {
        PathArguments::None => None,
        PathArguments::AngleBracketed(angle_bracketed) => {
            let field_generics = &angle_bracketed.args;
            let field_wrapper_generics = field_generics
                .iter()
                .filter(|field_generic| {
                    let field_argument_as_param: GenericParam = parse_quote!(#field_generic);

                    parent_type_generics
                        .params
                        .iter()
                        .any(|parent_generic| parent_generic == &field_argument_as_param)
                })
                .collect::<Vec<&GenericArgument>>();
            if field_wrapper_generics.is_empty() {
                None
            } else {
                Some(quote!(<#(#field_wrapper_generics,)*>))
            }
        }
        PathArguments::Parenthesized(_) => None,
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

/// Returns the `T: Serialize + DeserializeOwned` bounds to use for each type
/// parameter.
///
/// This will be either:
///
/// * whatever is provided in a user specified `#[serde(bound = "..")]`, or
/// * `T: Serialize + DeserializeOwned` if the bound has not been specified.
pub fn serde_bounds_for_type_params(ast: &DeriveInput) -> Vec<WherePredicate> {
    let mut serde_bounds_for_type_params = ast.attrs.iter().map(serde_bounds).fold(
        None::<Vec<WherePredicate>>,
        |mut where_predicates_all, where_predicates_for_bound| {
            if let Some(where_predicates_all) = where_predicates_all.as_mut() {
                if let Some(where_predicates_for_bound) = where_predicates_for_bound {
                    where_predicates_all.extend(where_predicates_for_bound);
                }
            } else if let Some(where_predicates_for_bound) = where_predicates_for_bound {
                where_predicates_all = Some(where_predicates_for_bound);
            }

            where_predicates_all
        },
    );

    if serde_bounds_for_type_params.is_none() {
        serde_bounds_for_type_params = Some(
            ast.generics
                .params
                .iter()
                .filter_map(|generic_param| match generic_param {
                    GenericParam::Lifetime(_) => None,
                    GenericParam::Type(type_param) => Some(type_param),
                    GenericParam::Const(_) => None,
                })
                .map(|type_param| {
                    parse_quote! {
                        #type_param: serde::Serialize + serde::de::DeserializeOwned
                    }
                })
                .collect::<Vec<WherePredicate>>(),
        );
    }

    serde_bounds_for_type_params.unwrap_or_default()
}

/// Returns the where predicate within a `#[serde(bound = "WherePredicate")]`
/// attribute.
///
/// See [`serde_derive::internals::attr::parse_lit_into_where`][parse_lit_into_where].
///
/// [parse_lit_into_where]: https://github.com/serde-rs/serde/blob/3e4a23cbd064f983e0029404e69b1210d232f94f/serde_derive/src/internals/attr.rs#L1469
pub fn serde_bounds(attr: &Attribute) -> Option<Vec<WherePredicate>> {
    let mut where_predicates = None::<Vec<WherePredicate>>;
    if attr.path().is_ident("serde") {
        let _ = attr.parse_nested_meta(|parse_nested_meta| {
            if parse_nested_meta.path.is_ident("bound") {
                let string = match get_lit_str(&parse_nested_meta)? {
                    Some(string) => string,
                    None => return Ok(()),
                };

                match string.parse_with(Punctuated::<WherePredicate, Token![,]>::parse_terminated) {
                    Ok(predicates) => {
                        if let Some(where_predicates) = where_predicates.as_mut() {
                            where_predicates.extend(predicates);
                        } else {
                            where_predicates =
                                Some(predicates.into_iter().collect::<Vec<WherePredicate>>());
                        }
                    }
                    Err(_error) => {}
                }
            }
            Ok(())
        });
    }

    where_predicates
}

fn get_lit_str(meta: &ParseNestedMeta) -> syn::Result<Option<syn::LitStr>> {
    let expr: syn::Expr = meta.value()?.parse()?;
    let mut value = &expr;
    while let syn::Expr::Group(e) = value {
        value = &e.expr;
    }
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit),
        ..
    }) = value
    {
        Ok(Some(lit.clone()))
    } else {
        Ok(None)
    }
}

/// Returns bounds for `T: Value + TryFrom<TPartial, T::Partial: From<T>`.
///
/// ```rust,ignore
/// T: Value<Spec = ValueSpecFieldless<T>> + TryFrom<<T as Value>::Partial>,
/// <T as ValueFieldless>::Partial: From<T>,
/// ```
pub fn t_value_and_try_from_partial_bounds<'f>(
    ast: &'f DeriveInput,
    peace_params_path: &'f Path,
) -> impl Iterator<Item = WherePredicate> + 'f {
    ast.generics
        .params
        .iter()
        .filter_map(|generic_param| match generic_param {
            GenericParam::Lifetime(_) => None,
            GenericParam::Type(type_param) => {
                if type_param.ident == "Id" {
                    None
                } else {
                    Some(type_param)
                }
            }
            GenericParam::Const(_) => None,
        })
        .flat_map(move |type_param| {
            let t_value_and_try_from_partial: WherePredicate = parse_quote! {
                #type_param:
                    #peace_params_path::ValueFieldless<Spec = #peace_params_path::ValueSpecFieldless<#type_param>>
                    + ::std::convert::TryFrom<<#type_param as #peace_params_path::ValueFieldless>::Partial>
            };
            let t_partial_from_t = parse_quote! {
                <#type_param as #peace_params_path::ValueFieldless>::Partial:
                    ::std::convert::From<#type_param>
            };
            [t_value_and_try_from_partial, t_partial_from_t]
        })
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
