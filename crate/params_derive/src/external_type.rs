use proc_macro2::Span;
use syn::{DeriveInput, Fields, FieldsUnnamed, Ident, Path, Type};

use crate::{
    type_gen_external::{type_gen_external, External},
    util::{field_wrapper_generics, is_tagged_fieldless, type_path_simple_name},
    TypeGen,
};

/// Consolidates all the logic to wrap external types together.
///
/// This should make it easier to trace wherever external type logic is used, by
/// searching for `ExternalType::`.
pub struct ExternalType;

impl ExternalType {
    /// Implements external wrapper types for Params' fields that are defined by
    /// external crates.
    pub fn external_wrapper_types(
        ast: &DeriveInput,
        peace_params_path: &Path,
    ) -> proc_macro2::TokenStream {
        match &ast.data {
            syn::Data::Struct(data_struct) => {
                let fields_iter = data_struct.fields.iter();
                let (_, external_wrapper_types) =
                    external_wrapper_types_impl!(ast, fields_iter, peace_params_path);
                external_wrapper_types
            }
            syn::Data::Enum(data_enum) => {
                let fields_iter = data_enum
                    .variants
                    .iter()
                    .flat_map(|variant| variant.fields.iter());
                let (_, external_wrapper_types) =
                    external_wrapper_types_impl!(ast, fields_iter, peace_params_path);
                external_wrapper_types
            }
            syn::Data::Union(data_union) => {
                let fields_iter = data_union.fields.named.iter();
                let (_, external_wrapper_types) =
                    external_wrapper_types_impl!(ast, fields_iter, peace_params_path);
                external_wrapper_types
            }
        }
    }

    /// Generates `ThingWrapper` and `ThingWrapperPartial`.
    ///
    /// This used when `Thing` is an external type to both `peace` and the item
    /// spec crate, but is used as a `Value` within an `ItemSpec::Params`.
    ///
    /// # Parameters
    ///
    /// * `type_name`: Name of the type to generate.
    /// * `wrapper_type`: `Type` of the wrapper to generate, generated by
    ///   `ExternalType::wrapper_type`.
    /// * `wrapped_ty`: `Type` of the field that is external, e.g. `Thing`.
    pub fn wrapper_and_related_types_gen(
        parent_ast: Option<&DeriveInput>,
        peace_params_path: &Path,
        wrapper_type: &Type,
        wrapped_ty: &Type,
    ) -> proc_macro2::TokenStream {
        let wrapper_partial_type = Self::wrapper_partial_type(parent_ast, wrapped_ty);
        let Some(wrapper_partial_name) = type_path_simple_name(&wrapper_partial_type) else {
            unreachable!("Type must be present at this stage.");
        };
        let Some(wrapper_name) = type_path_simple_name(wrapper_type) else {
            unreachable!("Type must be present at this stage.");
        };

        let mut tokens = Self::wrapper_gen(
            peace_params_path,
            wrapper_type,
            wrapped_ty,
            wrapper_partial_name,
        );
        tokens.extend(Self::wrapper_partial_gen(
            wrapper_name,
            &wrapper_partial_type,
            wrapped_ty,
        ));

        tokens
    }

    /// Generates `struct ThingWrapper(Thing)` for a given `Thing`.
    ///
    /// This used when `Thing` is an external type to both `peace` and the item
    /// spec crate, but is used as a `Value` within an `ItemSpec::Params`.
    ///
    /// # Parameters
    ///
    /// * `peace_params_path`: One of `peace::params`, `peace_params`, or
    ///   `crate::params`.
    /// * `wrapper_type`: `Type` of the wrapper to generate, generated by
    ///   `ExternalType::wrapper_type`.
    /// * `wrapped_ty`: `Type` of the field that is external, e.g. `Thing`.
    fn wrapper_gen(
        peace_params_path: &Path,
        wrapper_type: &Type,
        wrapped_ty: &Type,
        wrapper_partial_name: &Ident,
    ) -> proc_macro2::TokenStream {
        // TODO: we need to copy the type bounds from the params type onto any field
        // that uses that type parameter.
        let ast: DeriveInput = parse_quote!(pub struct #wrapper_type;);
        let wrapper_name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        let fields_unnamed: FieldsUnnamed = parse_quote!((#wrapped_ty));
        let fields = Fields::from(fields_unnamed);

        let struct_constructor = TypeGen::struct_constructor(wrapper_name, &fields);
        let struct_fields_clone = TypeGen::struct_fields_clone(wrapper_name, &fields);
        let struct_fields_debug = TypeGen::struct_fields_debug(wrapper_name, &fields);

        quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            #[serde(bound = "")]
            pub struct #wrapper_name #ty_generics #fields;

            impl #impl_generics #wrapper_name #ty_generics
            #where_clause
            {
                #struct_constructor
            }

            impl #impl_generics ::std::clone::Clone
            for #wrapper_name #ty_generics
            #where_clause
            {
                fn clone(&self) -> Self {
                    #struct_fields_clone
                }
            }

            impl #impl_generics ::std::fmt::Debug
            for #wrapper_name #ty_generics
            #where_clause
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    #struct_fields_debug
                }
            }

            // impl From<ThingWrapper> for Thing
            impl #impl_generics ::std::convert::From<#wrapper_name #ty_generics>
            for #wrapped_ty
            #where_clause
            {
                fn from(wrapper: #wrapper_name #ty_generics) -> Self {
                    wrapper.0
                }
            }

            impl #impl_generics #peace_params_path::ValueFieldless
            for #wrapper_name #ty_generics
            #where_clause
            {
                type Spec = #peace_params_path::ValueSpecFieldless<#wrapper_name #ty_generics>;
                type Partial = #wrapper_partial_name #ty_generics;
            }
        }
    }

    /// Generates `ThingWrapper` and `ThingWrapperPartial`.
    ///
    /// This used when `Thing` is an external type to both `peace` and the item
    /// spec crate, but is used as a `Value` within an `ItemSpec::Params`.
    ///
    /// # Parameters
    ///
    /// * `wrapper_partial_type`: `Type` of the wrapper to generate, generated
    ///   by `ExternalType::wrapper_partial_type`.
    /// * `wrapped_ty`: `Type` of the field that is external, e.g. `Thing`.
    fn wrapper_partial_gen(
        wrapper_name: &Ident,
        wrapper_partial_type: &Type,
        wrapped_ty: &Type,
    ) -> proc_macro2::TokenStream {
        let Some(wrapper_partial_name) = type_path_simple_name(wrapper_partial_type) else {
            unreachable!("Type must be present at this stage.");
        };

        let ast: DeriveInput = parse_quote!(pub struct #wrapper_partial_type;);
        let generics_split = ast.generics.split_for_impl();

        type_gen_external(
            &ast,
            &generics_split,
            External::Wrapper {
                value_ty: wrapped_ty,
                wrapper_name,
            },
            wrapper_partial_name,
            &[parse_quote!(#[derive(serde::Serialize, serde::Deserialize)])],
        )
    }

    /// Returns the wrapper type name.
    ///
    /// In practice, `Thing<T>` will generate `ThingWrapper<T>`, but this may
    /// change in the future, e.g. to avoid name collisions.
    pub fn wrapper_type(parent_ast: Option<&DeriveInput>, ty: &Type) -> Type {
        match ty {
            Type::Path(type_path) => {
                let Some(field_type_segment) = type_path.path.segments.last() else {
                    unreachable!("Field type must have at least one segment.");
                };
                let field_type_name = &field_type_segment.ident;
                let field_generics = &field_type_segment.arguments;
                let wrapper_type_name = {
                    let prefix = parent_ast
                        .map(|parent_ast| format!("{}", parent_ast.ident))
                        .unwrap_or_else(|| String::from(""));
                    let mut wrapper_type_name = format!("{prefix}{field_type_name}Wrapper");
                    if let Some(first_char) = wrapper_type_name.get_mut(0..1) {
                        first_char.make_ascii_uppercase()
                    }
                    Ident::new(&wrapper_type_name, Span::call_site())
                };

                let field_wrapper_generics = field_wrapper_generics(parent_ast, field_generics);

                parse_quote!(#wrapper_type_name #field_wrapper_generics)
            }

            // Type::Array(_)
            // | Type::BareFn(_)
            // | Type::Group(_)
            // | Type::ImplTrait(_)
            // | Type::Infer(_)
            // | Type::Macro(_)
            // | Type::Never(_)
            // | Type::Paren(_)
            // | Type::Ptr(_)
            // | Type::Reference(_)
            // | Type::Slice(_)
            // | Type::TraitObject(_)
            // | Type::Tuple(_)
            // | Type::Verbatim(_)
            _ => panic!(
                "Unsupported type to generate wrapper: {ty}",
                ty = quote!(#ty)
            ),
        }
    }

    /// Returns the wrapper partial type name.
    ///
    /// In practice, `Thing<T>` will generate `ThingWrapperPartial<T>`, but this
    /// may change in the future, e.g. to avoid name collisions.
    pub fn wrapper_partial_type(parent_ast: Option<&DeriveInput>, ty: &Type) -> Type {
        match ty {
            Type::Path(type_path) => {
                let Some(field_type_segment) = type_path.path.segments.last() else {
                    unreachable!("Field type must have at least one segment.");
                };
                let field_type_name = &field_type_segment.ident;
                let field_generics = &field_type_segment.arguments;
                let wrapper_partial_type_name = {
                    let prefix = parent_ast
                        .map(|parent_ast| format!("{}", parent_ast.ident))
                        .unwrap_or_else(|| String::from(""));
                    let mut wrapper_partial_type_name =
                        format!("{prefix}{field_type_name}WrapperPartial");
                    if let Some(first_char) = wrapper_partial_type_name.get_mut(0..1) {
                        first_char.make_ascii_uppercase()
                    }
                    Ident::new(&wrapper_partial_type_name, Span::call_site())
                };

                let field_wrapper_generics = field_wrapper_generics(parent_ast, field_generics);

                parse_quote!(#wrapper_partial_type_name #field_wrapper_generics)
            }

            // Type::Array(_)
            // | Type::BareFn(_)
            // | Type::Group(_)
            // | Type::ImplTrait(_)
            // | Type::Infer(_)
            // | Type::Macro(_)
            // | Type::Never(_)
            // | Type::Paren(_)
            // | Type::Ptr(_)
            // | Type::Reference(_)
            // | Type::Slice(_)
            // | Type::TraitObject(_)
            // | Type::Tuple(_)
            // | Type::Verbatim(_)
            _ => panic!(
                "Unsupported type to generate wrapper: {ty}",
                ty = quote!(#ty)
            ),
        }
    }
}

macro_rules! external_wrapper_types_impl {
    ($parent_ast:ident, $fields_iter:ident, $peace_params_path:ident) => {
        $fields_iter
            .filter_map(|field| {
                // We don't want to generate wrapper types for std external fields.
                //
                // Else we'd use `is_external_field`
                if is_tagged_fieldless(&field.attrs) {
                    let field_ty = &field.ty;
                    let wrapper_type = ExternalType::wrapper_type(Some($parent_ast), field_ty);
                    Some((wrapper_type, field_ty))
                } else {
                    None
                }
            })
            .fold(
                (
                    std::collections::HashSet::new(),
                    proc_macro2::TokenStream::new(),
                ),
                |(mut generated_types, mut tokens), (wrapper_type, field_ty)| {
                    if !generated_types.contains(field_ty) {
                        tokens.extend(ExternalType::wrapper_and_related_types_gen(
                            Some($parent_ast),
                            $peace_params_path,
                            &wrapper_type,
                            field_ty,
                        ));

                        generated_types.insert(field_ty.clone());
                    }
                    (generated_types, tokens)
                },
            )
    };
}

use external_wrapper_types_impl;
