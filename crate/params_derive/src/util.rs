use proc_macro2::Span;
use syn::{
    meta::ParseNestedMeta, punctuated::Punctuated, AngleBracketedGenericArguments, Attribute,
    DeriveInput, Field, Fields, GenericArgument, GenericParam, Generics, Ident, LitInt, Path,
    PathArguments, PathSegment, ReturnType, Type, TypeGenerics, TypePath, Variant, WhereClause,
    WherePredicate,
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

/// Returns whether the type is annotated with `#[value_spec(fieldless)]`,
/// which means its spec should be fieldless.
///
/// This attribute must be:
///
/// * attached to std library types defined outside the `peace_params` crate.
/// * attached to each `Params`' field defined outside the item crate.
pub fn is_fieldless_type(ast: &DeriveInput) -> bool {
    is_known_fieldless_std_lib_spec(&ast.ident) || is_tagged_fieldless(&ast.attrs)
}

/// Returns if the given `Type`'s spec should be fieldless.
///
/// This applies to std library types, as well as non-`Path` types.
fn is_known_fieldless_std_lib_spec(ty_name: &Ident) -> bool {
    STD_LIB_TYPES
        .iter()
        .any(|std_lib_type| ty_name == std_lib_type)
}

/// Returns whether any of the attributes contains `#[value_spec(fieldless)]`.
///
/// This attribute should be:
///
/// * attached to std library types defined outside the `peace_params` crate, if
///   it isn't already covered by `STD_LIB_TYPES`.
/// * attached to each field in `Params` that is defined outside the item crate.
pub fn is_tagged_fieldless(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("value_spec") {
            let mut is_external = false;
            let _ = attr.parse_nested_meta(|parse_nested_meta| {
                is_external = parse_nested_meta.path.is_ident("fieldless");
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

/// Returns the value spec type for a value, e.g. `ParamsSpec<MyValue>` or
/// `ParamsSpecFieldless<String>`.
pub fn value_spec_ty(
    ast: &DeriveInput,
    ty_generics: &TypeGenerics,
    peace_params_path: &Path,
    impl_mode: ImplMode,
) -> proc_macro2::TokenStream {
    let value_name = &ast.ident;
    if is_fieldless_type(ast) || impl_mode == ImplMode::Fieldless {
        quote!(#peace_params_path::ParamsSpecFieldless<#value_name #ty_generics>)
    } else {
        quote!(#peace_params_path::ParamsSpec<#value_name #ty_generics>)
    }
}

/// Returns the value spec type for a value, e.g. `ParamsSpec::<MyValue>` or
/// `ParamsSpecFieldless::<String>`.
pub fn value_spec_ty_path(
    ast: &DeriveInput,
    ty_generics: &TypeGenerics,
    peace_params_path: &Path,
    impl_mode: ImplMode,
) -> proc_macro2::TokenStream {
    let value_name = &ast.ident;
    if is_fieldless_type(ast) || impl_mode == ImplMode::Fieldless {
        quote!(#peace_params_path::ParamsSpecFieldless::<#value_name #ty_generics>)
    } else {
        quote!(#peace_params_path::ParamsSpec::<#value_name #ty_generics>)
    }
}

/// Returns the type of a value spec field, e.g. `ParamsSpecFieldless<MyValue>`.
pub fn field_spec_ty(peace_params_path: &Path, field_ty: &Type) -> proc_macro2::TokenStream {
    quote!(#peace_params_path::ValueSpec<#field_ty>)
}

/// Returns the type of a value spec field, e.g.
/// `ParamsSpecFieldless::<MyValue>`.
pub fn field_spec_ty_path(peace_params_path: &Path, field_ty: &Type) -> proc_macro2::TokenStream {
    quote!(#peace_params_path::ValueSpec::<#field_ty>)
}

/// Returns the type of a value spec field, e.g.
/// `ParamsSpecFieldless::<MyValue>(#field_name)` or
/// `ParamsSpecFieldless::<MyValue>(FieldTypeWrapper(#field_name))`.
pub fn field_spec_ty_deconstruct(
    peace_params_path: &Path,
    field_name: &Ident,
) -> proc_macro2::TokenStream {
    quote!(#peace_params_path::ValueSpec::Value { value: #field_name })
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

/// Returns a comma separated list of deconstructed fields, deconstructed as
/// `field: field_other`.
///
/// Tuple fields are returned as `_n_other`, and marker fields are returned as
/// `::std::marker::PhantomData`.
pub fn fields_deconstruct_rename_other(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    fields_deconstruct_retain_map(
        fields,
        false,
        Some(|field_name| {
            let field_name_other = format_ident!("{}_other", field_name);
            quote!(#field_name_other)
        }),
    )
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

/// Returns the intersection of parent type arguments and variant field types.
///
/// This is the part between the angle brackets -- `T1, T2, ..`.
///
/// Note: This doesn't copy across any `WherePredicate`s. See
/// `variant_generics_where_clause` below.
pub fn variant_generics_intersect(
    parent_generics: &Generics,
    variant: &Variant,
) -> Vec<GenericParam> {
    let field_generics_maybe = variant
        .fields
        .iter()
        .flat_map(|field| field_generics_maybe(&field.ty));

    field_generics_maybe
        .filter(|field_generic| {
            parent_generics
                .params
                .iter()
                .any(|parent_generic_param| field_generic == parent_generic_param)
        })
        .collect::<Vec<GenericParam>>()
}

/// Returns the where clause to apply to an enum variant
pub fn variant_generics_where_clause(
    parent_generics: &Generics,
    variant_generics: &[GenericParam],
) -> Option<WhereClause> {
    let filtered_predicates = parent_generics.where_clause.as_ref().map(|where_clause| {
        where_clause
            .predicates
            .iter()
            .filter(|where_predicate| match where_predicate {
                WherePredicate::Lifetime(predicate_lifetime) => {
                    variant_generics
                        .iter()
                        .any(|variant_generic| match variant_generic {
                            GenericParam::Lifetime(variant_generic_lifetime) => {
                                variant_generic_lifetime.lifetime == predicate_lifetime.lifetime
                            }
                            GenericParam::Type(_) | GenericParam::Const(_) => false,
                        })
                }
                WherePredicate::Type(predicate_type) => {
                    variant_generics
                        .iter()
                        .any(|variant_generic| match variant_generic {
                            GenericParam::Type(variant_generic_type) => {
                                if let Type::Path(type_path) = &predicate_type.bounded_ty {
                                    type_path.path.is_ident(&variant_generic_type.ident)
                                } else {
                                    false
                                }
                            }
                            GenericParam::Lifetime(_) | GenericParam::Const(_) => false,
                        })
                }
                _ => false,
            })
    });

    filtered_predicates.map(|filtered_predicates| {
        parse_quote! {
            where
                #(#filtered_predicates,)*
        }
    })
}

/// Returns the simple types found in this type.
///
/// * If the type has no type parameters, then itself it returned.
/// * If the type has type parameters, then its type parameters are returned.
/// * This is applied recursively.
fn field_generics_maybe(ty: &Type) -> Vec<GenericParam> {
    match ty {
        Type::Array(type_array) => field_generics_maybe(&type_array.elem),
        Type::BareFn(bare_fn) => {
            let output_types = match &bare_fn.output {
                ReturnType::Default => Vec::new(),
                ReturnType::Type(_r_arrow, return_type) => field_generics_maybe(return_type),
            };

            bare_fn
                .inputs
                .iter()
                .flat_map(|fn_arg| field_generics_maybe(&fn_arg.ty))
                .chain(output_types)
                .collect::<Vec<GenericParam>>()
        }
        Type::Group(type_group) => field_generics_maybe(&type_group.elem),
        Type::ImplTrait(_) => {
            unreachable!("Cannot have impl trait in field type position.")
        }
        Type::Infer(_) => unreachable!("Cannot have inferred type `_` in field type position."),
        Type::Macro(_) => Vec::new(),
        Type::Never(_) => Vec::new(),
        Type::Paren(type_paren) => field_generics_maybe(&type_paren.elem),
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                if let PathArguments::AngleBracketed(inner_args) = &segment.arguments {
                    if inner_args.args.is_empty() {
                        if type_path.path.segments.len() == 1 {
                            // Return this type if there's no leading double colon, because a
                            // leading double colon means it cannot be a type parameter.
                            vec![parse_quote!(#ty)]
                        } else {
                            Vec::new()
                        }
                    } else {
                        // Recurse
                        inner_args
                            .args
                            .iter()
                            .flat_map(|generic_arg| {
                                match generic_arg {
                                    GenericArgument::Type(generic_ty) => {
                                        field_generics_maybe(generic_ty)
                                    }

                                    GenericArgument::Lifetime(lifetime) => {
                                        vec![parse_quote!(#lifetime)]
                                    }
                                    // GenericArgument::Const(_) |
                                    // GenericArgument::AssocType(_) |
                                    // GenericArgument::AssocConst(_) |
                                    // GenericArgument::Constraint(_) |
                                    _ => Vec::new(),
                                }
                            })
                            .collect::<Vec<GenericParam>>()
                    }
                } else if type_path.path.segments.len() == 1 {
                    // Return this type if there's no leading double colon, because a
                    // leading double colon means it cannot be a type parameter.
                    vec![parse_quote!(#ty)]
                } else {
                    Vec::new()
                }
            } else {
                unreachable!("Field type must have at least one segment");
            }
        }
        Type::Ptr(type_ptr) => field_generics_maybe(&type_ptr.elem),
        Type::Reference(type_reference) => field_generics_maybe(&type_reference.elem),
        Type::Slice(type_slice) => field_generics_maybe(&type_slice.elem),
        // trait objects with associated type params are not a likely use case
        Type::TraitObject(_) => Vec::new(),
        Type::Tuple(type_tuple) => type_tuple
            .elems
            .iter()
            .flat_map(field_generics_maybe)
            .collect::<Vec<GenericParam>>(),
        // Type::Verbatim(_) |
        _ => Vec::new(),
    }
}

/// Returns `let field_name = #expr;` where `expr` is determined by the provided
/// function.
///
/// `PhantomData` fields are skipped.
pub fn fields_vars_map<'f>(
    fields: &'f Fields,
    fn_expr: impl Fn(&Field, &Ident) -> proc_macro2::TokenStream + 'f,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'f {
    fields_stmt_map(fields, move |field, field_name, _field_index| {
        let expr = fn_expr(field, field_name);
        quote!(let #field_name = #expr;)
    })
}

/// Returns `#stmt` for each field, where `stmt` is determined by the provided
/// function.
///
/// * The `&Ident` passed into the closure is `_0` for tuple fields.
/// * The `LitInt` passed into the closure is `0` for the tuple index.
///
/// `PhantomData` fields are skipped.
pub fn fields_stmt_map<'f>(
    fields: &'f Fields,
    fn_stmt: impl Fn(&Field, &Ident, LitInt) -> proc_macro2::TokenStream + 'f,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'f {
    fields
        .iter()
        .enumerate()
        .filter(|(_field_index, field)| !is_phantom_data(&field.ty))
        .map(move |(field_index, field)| {
            let field_lit_int = tuple_index_from_field_index(field_index);
            if let Some(field_ident) = field.ident.as_ref() {
                fn_stmt(field, field_ident, field_lit_int)
            } else {
                let field_ident = tuple_ident_from_field_index(field_index);
                fn_stmt(field, &field_ident, field_lit_int)
            }
        })
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
    if let Some((type_path, inner_args)) = type_path_simple_and_args(field_ty) {
        let type_name = &type_path.ident;
        if type_name == "Option" {
            if let Some(inner_args) = inner_args {
                if inner_args.args.len() == 1 {
                    if let Some(GenericArgument::Type(inner_type)) = inner_args.args.first() {
                        if let Some((inner_type_path, _inner_args)) =
                            type_path_simple_and_args(inner_type)
                        {
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

pub fn type_path_simple_and_args(
    ty: &Type,
) -> Option<(&PathSegment, Option<&AngleBracketedGenericArguments>)> {
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

/// Whether to implement a fieldwise or fieldless spec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImplMode {
    /// Fields of the value type are known and accessible.
    Fieldwise,
    /// Fields of the value type are unknown or inaccessible.
    Fieldless,
}
