use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    punctuated::Punctuated, Attribute, DeriveInput, Fields, Ident, ImplGenerics, TypeGenerics,
    Variant, Visibility, WhereClause,
};

use crate::util::{
    field_ty_to_ref_ty, fields_deconstruct, fields_deconstruct_retain, is_phantom_data,
    is_serde_bound_attr, tuple_ident_from_field_index, tuple_index_from_field_index,
    variant_match_arm, RefTypeAndExpr,
};

pub struct TypeGen;

impl TypeGen {
    /// Generates a type based off the `Params` / `Value` type.
    ///
    /// # Parameters
    ///
    /// * `ast`: The `Params` type.
    /// * `generics_split`: Generics of the `Params` type.
    /// * `type_name`: Name of the type to generate.
    /// * `fields_map`: Transformation function to apply to the type of the
    ///   fields.
    /// * `attrs_to_add`: Attributes to attach to the generated type.
    pub fn gen_from_value_type<F>(
        ast: &DeriveInput,
        generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
        type_name: &Ident,
        fields_map: F,
        attrs_to_add: &[Attribute],
        generated_type_is_serializable: bool,
    ) -> proc_macro2::TokenStream
    where
        F: Fn(&mut Fields),
    {
        let (impl_generics, ty_generics, where_clause) = generics_split;
        let serde_bound_empty = parse_quote!(#[serde(bound = "")]);
        let serde_bound_attrs = if generated_type_is_serializable {
            let mut serde_bound_attrs = ast
                .attrs
                .iter()
                .filter(|attr| is_serde_bound_attr(attr))
                .collect::<Vec<&Attribute>>();
            if serde_bound_attrs.is_empty() {
                serde_bound_attrs.push(&serde_bound_empty);
            }
            serde_bound_attrs
        } else {
            Vec::new()
        };

        match &ast.data {
            syn::Data::Struct(data_struct) => {
                let mut fields = data_struct.fields.clone();
                fields_map(&mut fields);
                let struct_definition = if matches!(&fields, Fields::Unnamed(_) | Fields::Unit) {
                    quote! {
                        pub struct #type_name #ty_generics #fields
                        #where_clause;
                    }
                } else {
                    quote! {
                        pub struct #type_name #ty_generics
                        #where_clause
                        #fields
                    }
                };

                let struct_constructor = Self::struct_constructor(type_name, &fields);
                let struct_fields_clone = Self::struct_fields_clone(type_name, &fields);
                let struct_fields_debug = Self::struct_fields_debug(type_name, &fields);
                let struct_getters_and_mut_getters = Self::struct_getters_and_mut_getters(&fields);

                quote! {
                    #(#attrs_to_add)*
                    #(#serde_bound_attrs)*
                    #struct_definition

                    impl #impl_generics #type_name #ty_generics
                    #where_clause
                    {
                        #struct_constructor

                        #struct_getters_and_mut_getters
                    }

                    impl #impl_generics ::std::clone::Clone
                    for #type_name #ty_generics
                    #where_clause
                    {
                        fn clone(&self) -> Self {
                            #struct_fields_clone
                        }
                    }

                    impl #impl_generics ::std::fmt::Debug
                    for #type_name #ty_generics
                    #where_clause
                    {
                        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            #struct_fields_debug
                        }
                    }
                }
            }
            syn::Data::Enum(data_enum) => {
                let mut variants = data_enum.variants.clone();
                variants.iter_mut().for_each(|variant| {
                    fields_map(&mut variant.fields);
                });

                let variants_clone = Self::variants_clone(&variants);
                let variants_debug = Self::variants_debug(&variants);

                quote! {
                    #(#attrs_to_add)*
                    #(#serde_bound_attrs)*
                    pub enum #type_name #ty_generics
                    #where_clause
                    {
                        #variants
                    }

                    impl #impl_generics ::std::clone::Clone
                    for #type_name #ty_generics
                    #where_clause
                    {
                        fn clone(&self) -> Self {
                            #variants_clone
                        }
                    }

                    impl #impl_generics ::std::fmt::Debug
                    for #type_name #ty_generics
                    #where_clause
                    {
                        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            #variants_debug
                        }
                    }
                }
            }
            syn::Data::Union(data_union) => {
                let mut fields = Fields::from(data_union.fields.clone());
                fields_map(&mut fields);

                quote! {
                    #(#attrs_to_add)*
                    #(#serde_bound_attrs)*
                    pub union #type_name #ty_generics
                    #where_clause
                    #fields
                }
            }
        }
    }

    /// Returns tokens for a constructor `Struct::new(..)` if any fields are
    /// non-pub, or there are phantom data fields.
    pub fn struct_constructor(
        type_name: &Ident,
        fields: &Fields,
    ) -> Option<proc_macro2::TokenStream> {
        let constructor_needed = fields.iter().any(|field| {
            is_phantom_data(&field.ty)
                || matches!(field.vis, Visibility::Restricted(_) | Visibility::Inherited)
        });

        if constructor_needed {
            let constructor_doc = format!("Returns a new `{type_name}`.");
            let fields_deconstructed = fields_deconstruct(fields);
            let fields_as_params = fields
                .iter()
                .enumerate()
                .filter(|(_field_index, field)| !is_phantom_data(&field.ty))
                .map(|(field_index, field)| {
                    let field_name = if let Some(field_ident) = field.ident.as_ref() {
                        quote!(#field_ident)
                    } else {
                        let field_ident = tuple_ident_from_field_index(field_index);
                        quote!(#field_ident)
                    };
                    let field_ty = &field.ty;
                    quote!(#field_name: #field_ty)
                })
                .collect::<Vec<proc_macro2::TokenStream>>();

            let constructor = match fields {
                Fields::Named(_fields_named) => quote! {
                    Self {
                        #(#fields_deconstructed),*
                    }
                },
                Fields::Unnamed(_fields_unnamed) => quote! {
                    Self(#(#fields_deconstructed),*)
                },
                Fields::Unit => unreachable!("Guarded by `constructor_needed`."),
            };

            Some(quote! {
                #[doc = #constructor_doc]
                pub fn new(#(#fields_as_params),*) -> Self {
                    #constructor
                }
            })
        } else {
            None
        }
    }

    /// Returns `field(&self) -> &Field` and `field_mut(&mut self) -> &mut
    /// Field` for any fields that are non-pub.
    ///
    /// This includes special handling for the following types:
    ///
    /// * `Option`: returns `Option<&Field>`.
    /// * `PathBuf`: returns `&Path`.
    /// * `Vec<T>`: returns `&[T]`.
    /// * `String`: returns `&str`.
    pub fn struct_getters_and_mut_getters(fields: &Fields) -> proc_macro2::TokenStream {
        let fields_as_params = fields
            .iter()
            .enumerate()
            .filter(|(_field_index, field)| {
                !is_phantom_data(&field.ty)
                    && matches!(field.vis, Visibility::Restricted(_) | Visibility::Inherited)
            })
            .map(|(field_index, field)| {
                let (self_field_name, field_name) = if let Some(field_ident) = field.ident.as_ref()
                {
                    let self_field_name = field_ident.to_token_stream();
                    let field_name = self_field_name.clone();
                    (self_field_name, field_name)
                } else {
                    let self_field_name =
                        tuple_index_from_field_index(field_index).to_token_stream();
                    let field_name = tuple_ident_from_field_index(field_index).to_token_stream();
                    (self_field_name, field_name)
                };
                let field_name_mut = Ident::new(&format!("{field_name}_mut"), Span::call_site());

                let RefTypeAndExpr {
                    ref_type,
                    ref_mut_type,
                    ref_expr,
                    ref_mut_expr,
                } = field_ty_to_ref_ty(&field.ty, &self_field_name);
                quote! {
                    pub fn #field_name(&self) -> #ref_type {
                        #ref_expr
                    }

                    pub fn #field_name_mut(&mut self) -> #ref_mut_type {
                        #ref_mut_expr
                    }
                }
            })
            .collect::<Vec<proc_macro2::TokenStream>>();

        quote! {
            #(#fields_as_params)*
        }
    }

    pub fn struct_fields_clone(type_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
        match fields {
            Fields::Named(fields_named) => {
                // Generates:
                //
                // #type_name {
                //     field_1: self.field_1.clone(),
                //     field_2: self.field_2.clone(),
                //     marker: PhantomData,
                // }

                let fields_clone = fields_named.named.iter().fold(
                    proc_macro2::TokenStream::new(),
                    |mut tokens, field| {
                        if let Some(field_name) = field.ident.as_ref() {
                            if is_phantom_data(&field.ty) {
                                tokens.extend(quote! {
                                    #field_name: std::marker::PhantomData,
                                });
                            } else {
                                tokens.extend(quote! {
                                    #field_name: self.#field_name.clone(),
                                });
                            }
                        }
                        tokens
                    },
                );

                quote! {
                    #type_name {
                        #fields_clone
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                // Generates:
                //
                // #type_name(self.0.clone(), self.1.clone(), PhantomData)
                let fields_clone = fields_unnamed.unnamed.iter().enumerate().fold(
                    proc_macro2::TokenStream::new(),
                    |mut tokens, (field_index, field)| {
                        let field_index = tuple_index_from_field_index(field_index);

                        if is_phantom_data(&field.ty) {
                            tokens.extend(quote!(std::marker::PhantomData,));
                        } else {
                            tokens.extend(quote!(self.#field_index.clone(),));
                        }

                        tokens
                    },
                );

                quote! {
                    #type_name(#fields_clone)
                }
            }
            Fields::Unit => quote!(#type_name),
        }
    }

    pub fn variants_clone(variants: &Punctuated<Variant, Token![,]>) -> proc_macro2::TokenStream {
        // Generates:
        //
        // match self {
        //     Self::Variant1 => Self::Variant1,
        //     Self::Variant2(_0, _1, PhantomData) => {
        //         Self::Variant2(_0.clone(), _1.clone(), PhantomData)
        //     }
        //     Self::Variant3 { field_1, field_2, marker: PhantomData } => {
        //         Self::Variant3 {
        //             field_1: field_1.clone(),
        //             field_2: field_2.clone(),
        //             marker: PhantomData,
        //         }
        //     }
        // }
        let self_ident = Ident::new("Self", Span::call_site());

        let variant_clone_arms =
            variants
                .iter()
                .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                    let variant_fields = fields_deconstruct(&variant.fields);
                    let variant_fields_clone =
                        Self::variant_fields_clone(&variant.ident, &variant.fields);

                    tokens.extend(variant_match_arm(
                        &self_ident,
                        variant,
                        &variant_fields,
                        variant_fields_clone,
                    ));

                    tokens
                });

        quote! {
            match self {
                #variant_clone_arms
            }
        }
    }

    pub fn variant_fields_clone(variant_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
        match fields {
            Fields::Named(fields_named) => {
                // Generates:
                //
                // Self {
                //     field_1: field_1.clone(),
                //     field_2: field_2.clone(),
                //     marker: PhantomData,
                // }

                let fields_clone = fields_named.named.iter().fold(
                    proc_macro2::TokenStream::new(),
                    |mut tokens, field| {
                        if let Some(field_name) = field.ident.as_ref() {
                            if is_phantom_data(&field.ty) {
                                tokens.extend(quote! {
                                    #field_name: std::marker::PhantomData,
                                });
                            } else {
                                tokens.extend(quote! {
                                    #field_name: #field_name.clone(),
                                });
                            }
                        }
                        tokens
                    },
                );

                quote! {
                    Self::#variant_name {
                        #fields_clone
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                // Generates:
                //
                // Self(_0.clone(), _1.clone(), PhantomData)
                let fields_clone = fields_unnamed
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
                                tokens.extend(quote!(#field_index.clone(),));
                            }

                            tokens
                        },
                    );

                quote! {
                    Self::#variant_name(#fields_clone)
                }
            }
            Fields::Unit => quote!(Self::#variant_name),
        }
    }

    pub fn variants_debug(variants: &Punctuated<Variant, Token![,]>) -> proc_macro2::TokenStream {
        // Generates:
        //
        // match self {
        //     Self::Variant1 => f.debug_struct("Variant1").finish(),
        //     Self::Variant2 => f.debug_tuple("Variant2").finish(),
        //     Self::Variant3 { .. } => f.debug_struct("Variant3").field(..).finish(),
        // }
        let self_ident = Ident::new("Self", Span::call_site());

        let variant_debug_arms =
            variants
                .iter()
                .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                    // This differs from `crate::util::fields_deconstruct` in that
                    // this retains `PhantomData` fields.
                    let variant_fields = fields_deconstruct_retain(&variant.fields, true);

                    let variant_fields_debug = Self::fields_debug(&variant.ident, &variant.fields);

                    tokens.extend(variant_match_arm(
                        &self_ident,
                        variant,
                        &variant_fields,
                        variant_fields_debug,
                    ));

                    tokens
                });

        quote! {
            match self {
                #variant_debug_arms
            }
        }
    }

    pub fn struct_fields_debug(type_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
        let fields_debug = Self::fields_debug(type_name, fields);
        let fields_deconstructed = fields_deconstruct_retain(fields, true);

        match fields {
            Fields::Named(_fields_named) => {
                // Generates:
                //
                // ```rust
                // let #type_name {
                //     field_1,
                //     field_2,
                //     marker: PhantomData,
                // } = self;
                //
                // #fields_debug
                // ```

                quote! {
                    let #type_name {
                        #(#fields_deconstructed),*
                    } = self;

                    #fields_debug
                }
            }
            Fields::Unnamed(_fields_unnamed) => {
                // Generates:
                //
                // ```rust
                // let #type_name(_0, _1, PhantomData,) = self;
                //
                // #fields_debug
                // ```

                quote! {
                    let #type_name(#(#fields_deconstructed),*) = self;

                    #fields_debug
                }
            }
            Fields::Unit => fields_debug,
        }
    }

    pub fn fields_debug(type_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
        let type_name = &type_name.to_string();
        match fields {
            Fields::Named(fields_named) => {
                // Generates:
                //
                // let mut debug_struct = f.debug_struct(#type_name);
                // debug_struct.field("field_0", &field_0);
                // debug_struct.field("field_1", &field_1);
                // debug_struct.finish()

                let tokens = quote! {
                    let mut debug_struct = f.debug_struct(#type_name);
                };

                let mut tokens = fields_named.named.iter().fold(tokens, |mut tokens, field| {
                    if let Some(field_name) = field.ident.as_ref() {
                        let field_name_str = &field_name.to_string();
                        tokens.extend(quote! {
                            debug_struct.field(#field_name_str, &#field_name);
                        });
                    }
                    tokens
                });

                tokens.extend(quote!(debug_struct.finish()));

                tokens
            }
            Fields::Unnamed(fields_unnamed) => {
                // Generates:
                //
                // let mut debug_tuple = f.debug_tuple(#type_name);
                // debug_tuple.field(&_0);
                // debug_tuple.field(&_1);
                // debug_tuple.finish()

                let tokens = quote! {
                    let mut debug_tuple = f.debug_tuple(#type_name);
                };

                let mut tokens = (0..fields_unnamed.unnamed.len())
                    .map(tuple_ident_from_field_index)
                    .fold(tokens, |mut tokens, field_index| {
                        tokens.extend(quote! {
                            debug_tuple.field(&#field_index);
                        });
                        tokens
                    });

                tokens.extend(quote!(debug_tuple.finish()));

                tokens
            }
            Fields::Unit => quote!(f.debug_struct(#type_name).finish()),
        }
    }
}
