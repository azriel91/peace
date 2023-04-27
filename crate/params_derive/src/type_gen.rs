use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, Attribute, DeriveInput, Fields, Ident, ImplGenerics, LitInt,
    TypeGenerics, Variant, WhereClause,
};

use crate::util::{is_phantom_data, tuple_ident_from_field_index};

/// Generates a type based off the `Params` type.
///
/// # Parameters
///
/// * `ast`: The `Params` type.
/// * `generics_split`: Generics of the `Params` type.
/// * `type_name`: Name of the type to generate.
/// * `fields_map`: Transformation function to apply to the type of the fields.
/// * `attrs`: Attributes to attach to the generated type.
pub fn type_gen<F>(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    type_name: &Ident,
    fields_map: F,
    attrs: &[Attribute],
) -> proc_macro2::TokenStream
where
    F: Fn(&mut Fields),
{
    let (impl_generics, ty_generics, _where_clause) = generics_split;

    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let mut fields = data_struct.fields.clone();
            fields_map(&mut fields);
            let semi_colon_maybe = if matches!(&fields, Fields::Unnamed(_) | Fields::Unit) {
                quote!(;)
            } else {
                quote!()
            };

            let struct_fields_clone = struct_fields_clone(type_name, &fields);
            let struct_fields_debug = struct_fields_debug(type_name, &fields);

            quote! {
                #(#attrs)*
                pub struct #type_name #ty_generics #fields #semi_colon_maybe

                impl #impl_generics ::std::clone::Clone for #type_name #ty_generics {
                    fn clone(&self) -> Self {
                        #struct_fields_clone
                    }
                }

                impl #impl_generics ::std::fmt::Debug for #type_name #ty_generics {
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

            let variants_clone = variants_clone(&variants);
            let variants_debug = variants_debug(&variants);

            quote! {
                #(#attrs)*
                pub enum #type_name #ty_generics {
                    #variants
                }

                impl #impl_generics ::std::clone::Clone for #type_name #ty_generics {
                    fn clone(&self) -> Self {
                        #variants_clone
                    }
                }

                impl #impl_generics ::std::fmt::Debug for #type_name #ty_generics {
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
                #(#attrs)*
                pub union #type_name #ty_generics #fields
            }
        }
    }
}

fn struct_fields_clone(type_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
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
                    // Need to convert this to a `LitInt`,
                    // because `quote` outputs the index as `0usize` instead of `0`
                    let field_index = LitInt::new(&format!("{field_index}"), Span::call_site());

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

fn variants_clone(variants: &Punctuated<Variant, Token![,]>) -> proc_macro2::TokenStream {
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

    let variant_clone_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let variant_name = &variant.ident;
                let variant_fields = &variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(field_index, field)| {
                        if is_phantom_data(&field.ty) {
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
                    .collect::<Vec<proc_macro2::TokenStream>>();
                let variant_fields_clone = variant_fields_clone(&variant.ident, &variant.fields);

                match &variant.fields {
                    Fields::Named(_fields_named) => {
                        tokens.extend(quote! {
                            Self::#variant_name { #(#variant_fields),* } => {
                                #variant_fields_clone
                            }
                        });
                    }
                    Fields::Unnamed(_) => {
                        tokens.extend(quote! {
                            Self::#variant_name(#(#variant_fields),*) => {
                                #variant_fields_clone
                            }
                        });
                    }
                    Fields::Unit => {
                        tokens.extend(quote! {
                            Self::#variant_name => {
                                #variant_fields_clone
                            }
                        });
                    }
                }

                tokens
            });

    quote! {
        match self {
            #variant_clone_arms
        }
    }
}

fn variant_fields_clone(variant_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
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

fn variants_debug(variants: &Punctuated<Variant, Token![,]>) -> proc_macro2::TokenStream {
    // Generates:
    //
    // match self {
    //     Self::Variant1 => f.debug_struct("Variant1").finish(),
    //     Self::Variant2 => f.debug_tuple("Variant2").finish(),
    //     Self::Variant3 { .. } => f.debug_struct("Variant3").field(..).finish(),
    // }

    let variant_debug_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let variant_name = &variant.ident;
                let variant_fields = &variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(field_index, field)| {
                        field
                            .ident
                            .clone()
                            .unwrap_or_else(|| tuple_ident_from_field_index(field_index))
                    })
                    .collect::<Vec<Ident>>();
                let variant_fields_debug = variant_fields_debug(&variant.ident, &variant.fields);

                match &variant.fields {
                    Fields::Named(_fields_named) => {
                        tokens.extend(quote! {
                            Self::#variant_name { #(#variant_fields),* } => {
                                #variant_fields_debug
                            }
                        });
                    }
                    Fields::Unnamed(_) => {
                        tokens.extend(quote! {
                            Self::#variant_name(#(#variant_fields),*) => {
                                #variant_fields_debug
                            }
                        });
                    }
                    Fields::Unit => {
                        tokens.extend(quote! {
                            Self::#variant_name => {
                                #variant_fields_debug
                            }
                        });
                    }
                }

                tokens
            });

    quote! {
        match self {
            #variant_debug_arms
        }
    }
}

fn struct_fields_debug(type_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let type_name = &type_name.to_string();
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // let mut debug_struct = f.debug_struct(#type_name);
            // debug_struct.field("field_0", &self.field_0);
            // debug_struct.field("field_1", &self.field_1);
            // debug_struct.finish()

            let tokens = quote! {
                let mut debug_struct = f.debug_struct(#type_name);
            };

            let mut tokens = fields_named.named.iter().fold(tokens, |mut tokens, field| {
                if let Some(field_name) = field.ident.as_ref() {
                    let field_name_str = &field_name.to_string();
                    tokens.extend(quote! {
                        debug_struct.field(#field_name_str, &self.#field_name);
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
            // debug_tuple.field(&self.0);
            // debug_tuple.field(&self.1);
            // debug_tuple.finish()

            let tokens = quote! {
                let mut debug_tuple = f.debug_tuple(#type_name);
            };

            let mut tokens =
                (0..fields_unnamed.unnamed.len()).fold(tokens, |mut tokens, field_index| {
                    // Need to convert this to a `LitInt`,
                    // because `quote` outputs the index as `0usize` instead of `0`
                    let field_index = LitInt::new(&format!("{field_index}"), Span::call_site());
                    tokens.extend(quote! {
                        debug_tuple.field(&self.#field_index);
                    });
                    tokens
                });

            tokens.extend(quote!(debug_tuple.finish()));

            tokens
        }
        Fields::Unit => quote!(f.debug_struct(#type_name).finish()),
    }
}

fn variant_fields_debug(variant_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let variant_name = &variant_name.to_string();
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // let mut debug_struct = f.debug_struct(#variant_name);
            // debug_struct.field("field_0", &field_0);
            // debug_struct.field("field_1", &field_1);
            // debug_struct.finish()

            let tokens = quote! {
                let mut debug_struct = f.debug_struct(#variant_name);
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
            // let mut debug_tuple = f.debug_tuple(#variant_name);
            // debug_tuple.field(&_0);
            // debug_tuple.field(&_1);
            // debug_tuple.finish()

            let tokens = quote! {
                let mut debug_tuple = f.debug_tuple(#variant_name);
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
        Fields::Unit => quote!(f.debug_struct(#variant_name).finish()),
    }
}
