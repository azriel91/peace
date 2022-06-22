extern crate proc_macro;

use proc_macro2::{Span, TokenStream as Tokens};
use quote::{format_ident, quote};
use std::iter::Peekable;
use syn::{
    parenthesized, parse,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Comma, Where},
    Attribute, Data, DataEnum, DeriveInput, Error, Field, Fields, GenericParam, Generics, Ident,
    ImplGenerics, Index, Result as SynResult, TypeGenerics, TypeParam, Visibility, WhereClause,
    WherePredicate,
};

use crate::proc_macro::TokenStream;

#[proc_macro_derive(Diff, attributes(diff))]
pub fn diff_derive(input: TokenStream) -> TokenStream {
    match derive_or_error(input) {
        Err(err) => err.to_compile_error().into(),
        Ok(result) => result,
    }
}

fn derive_or_error(input: TokenStream) -> SynResult<TokenStream> {
    let input: DeriveInput = syn::parse(input)?;
    let vis = input.vis;
    let ident = input.ident;

    let attrs = parse_struct_attributes(&input.attrs, vis, &ident)?;

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    // Create a `T: Diff + PartialEq` predicate for each type param, and add it to
    // the where clause
    let where_clause_owned = type_where_clause(&input.generics, where_clause);
    let where_clause_with_diff_predicates = where_clause_owned.as_ref().or(where_clause);
    let generics = (
        impl_generics,
        type_generics,
        where_clause_with_diff_predicates,
    );

    let tokens = match input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => derive_named(attrs, ident, &fields.named, generics)?,
            Fields::Unnamed(fields) => derive_unnamed(attrs, ident, &fields.unnamed, generics)?,
            Fields::Unit => derive_unit(ident),
        },
        Data::Enum(data_enum) => derive_enum(attrs, ident, &data_enum, generics),
        _ => todo!(),
    }
    .into();
    Ok(tokens)
}

fn type_where_clause(
    generics: &Generics,
    where_clause: Option<&WhereClause>,
) -> Option<WhereClause> {
    let mut where_predicates = type_where_predicates(generics);
    if where_predicates.peek().is_some() {
        let mut where_clause = match where_clause {
            Some(where_clause) => where_clause.to_owned(),
            None => WhereClause {
                where_token: Where::default(),
                predicates: Punctuated::default(),
            },
        };

        where_predicates.for_each(|predicate| where_clause.predicates.push(predicate));
        Some(where_clause)
    } else {
        None
    }
}

fn type_where_predicates(
    generics: &Generics,
) -> Peekable<impl Iterator<Item = WherePredicate> + '_> {
    let where_predicates = generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(TypeParam { ident, bounds, .. }) = param {
                let bounds = if bounds.is_empty() {
                    quote!(Diff)
                } else {
                    quote!(#bounds + Diff)
                };
                let where_predicate: WherePredicate = syn::parse2(quote! { #ident: #bounds })
                    .expect("Failed to parse where predicate in diff_derive");
                Some(where_predicate)
            } else {
                None
            }
        })
        .peekable();
    where_predicates
}

fn derive_named(
    attrs: StructAttributes,
    ident: Ident,
    fields: &Punctuated<Field, Comma>,
    generics: (ImplGenerics<'_>, TypeGenerics<'_>, Option<&'_ WhereClause>),
) -> SynResult<Tokens> {
    let attr = attrs.attrs;
    let diff_ident = attrs.name;
    let visibility = attrs.visibility;
    let (impl_generics, type_generics, where_clause) = generics;

    let field_attrs = fields
        .iter()
        .map(|field| {
            parse_named_field_attributes(&field.attrs, &field.vis, field.ident.as_ref().unwrap())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let types = fields.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let names = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let diff_names = field_attrs.iter().map(|f| &f.name).collect::<Vec<_>>();
    let attrs = field_attrs.iter().map(|f| &f.attrs).collect::<Vec<_>>();
    let visbs = field_attrs
        .iter()
        .map(|f| &f.visibility)
        .collect::<Vec<_>>();

    Ok(quote! {
        #(#attr)*
        #visibility struct #diff_ident #type_generics #where_clause {
            #(
                #(#attrs)*
                #visbs #diff_names: <#types as Diff>::Repr
            ),*
        }

        impl #impl_generics Diff for #ident #type_generics #where_clause {
            type Repr = #diff_ident #type_generics;

            fn diff(&self, other: &Self) -> Self::Repr {
                #diff_ident {
                    #(#diff_names: self.#names.diff(&other.#names)),*
                }
            }

            fn apply(&mut self, diff: &Self::Repr) {
                #(self.#names.apply(&diff.#diff_names);)*
            }

            fn identity() -> Self {
                Self {
                    #(#names: <#types as Diff>::identity()),*
                }
            }
        }
    })
}

fn derive_unnamed(
    attrs: StructAttributes,
    ident: Ident,
    fields: &Punctuated<Field, Comma>,
    generics: (ImplGenerics<'_>, TypeGenerics<'_>, Option<&'_ WhereClause>),
) -> SynResult<Tokens> {
    let attr = attrs.attrs;
    let diff_ident = attrs.name;
    let visibility = attrs.visibility;
    let (impl_generics, type_generics, where_clause) = generics;

    let field_attrs = fields
        .iter()
        .map(|field| parse_unnamed_field_attributes(&field.attrs, &field.vis))
        .collect::<Result<Vec<_>, _>>()?;

    let (numbers, types): (Vec<_>, Vec<_>) = fields
        .iter()
        .map(|field| &field.ty)
        .enumerate()
        .map(|(a, b)| (Index::from(a), b))
        .unzip();
    let attrs = field_attrs.iter().map(|f| &f.attrs).collect::<Vec<_>>();
    let visbs = field_attrs
        .iter()
        .map(|f| &f.visibility)
        .collect::<Vec<_>>();

    Ok(quote! {
        #(#attr)*
        #visibility struct #diff_ident #type_generics (
            #(
                #(#attrs)*
                #visbs <#types as Diff>::Repr
            ),*
        ) #where_clause ;

        impl #impl_generics Diff for #ident #type_generics #where_clause {
            type Repr = #diff_ident #type_generics;

            fn diff(&self, other: &Self) -> Self::Repr {
                #diff_ident (
                    #(self.#numbers.diff(&other.#numbers)),*
                )
            }

            fn apply(&mut self, diff: &Self::Repr) {
                #(self.#numbers.apply(&diff.#numbers);)*
            }

            fn identity() -> Self {
                Self (
                    #(<#types as Diff>::identity()),*
                )
            }
        }
    })
}

fn derive_unit(ident: Ident) -> Tokens {
    quote! {
        impl Diff for #ident {
            type Repr = ();

            fn diff(&self, other: &Self) -> Self::Repr {
                ()
            }

            fn apply(&mut self, diff: &Self::Repr) {
                ()
            }

            fn identity() -> Self {
                Self
            }
        }
    }
}

fn derive_enum(
    attrs: StructAttributes,
    ident: Ident,
    data_enum: &DataEnum,
    generics: (ImplGenerics<'_>, TypeGenerics<'_>, Option<&'_ WhereClause>),
) -> Tokens {
    let attr = attrs.attrs;
    let diff_ident = attrs.name;
    let visibility = attrs.visibility;
    let (impl_generics, type_generics, where_clause) = generics;

    let first = data_enum.variants.first().unwrap();
    let first_ident = &first.ident;
    let first_names = first
        .fields
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();
    let first_types = first
        .fields
        .iter()
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

    let variants_type_decl = data_enum
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            match &variant.fields {
                Fields::Named(fields) => {
                    let names = fields
                        .named
                        .iter()
                        .map(|field| &field.ident)
                        .collect::<Vec<_>>();
                    let types = fields
                        .named
                        .iter()
                        .map(|field| &field.ty)
                        .collect::<Vec<_>>();

                    quote! { #ident{#(#names: <#types as Diff>::Repr),*} }
                }
                Fields::Unnamed(fields) => {
                    let types = fields
                        .unnamed
                        .iter()
                        .map(|field| &field.ty)
                        .collect::<Vec<_>>();
                    quote! { #ident(#(<#types as Diff>::Repr),*) }
                }
                Fields::Unit => quote! {
                    #ident
                },
            }
        })
        .collect::<Vec<Tokens>>();

    let variants_diff_arms = data_enum
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            match &variant.fields {
                Fields::Named(fields) => {
                    let t = fields.named.iter().map(|x| &x.ty).collect::<Vec<_>>();
                    let i = fields.named.iter().map(|x| &x.ident).collect::<Vec<_>>();
                    let a = fields
                        .named
                        .iter()
                        .map(|x| {
                            syn::Ident::new(
                                &format!("a_{}", x.ident.as_ref().unwrap()),
                                Span::call_site(),
                            )
                        })
                        .collect::<Vec<_>>();
                    let b = fields
                        .named
                        .iter()
                        .map(|x| {
                            syn::Ident::new(
                                &format!("b_{}", x.ident.as_ref().unwrap()),
                                Span::call_site(),
                            )
                        })
                        .collect::<Vec<_>>();
                    quote! {
                        (Self::#ident{#(#i: #a),*}, Self::#ident{#(#i: #b),*}) =>
                            if #(#a == #b)&&* {
                                Self::Repr::NoChange
                            } else {
                                #diff_ident::#ident{#(#i: #a.diff(#b)),*}
                            },
                        (_, Self::#ident{#(#i: #b),*}) =>
                            #diff_ident::#ident{#(#i: <#t as Diff>::identity().diff(#b)),*}
                    }
                }
                Fields::Unnamed(fields) => {
                    let t = fields.unnamed.iter().map(|x| &x.ty).collect::<Vec<_>>();
                    let a = (0..fields.unnamed.len())
                        .map(|x| syn::Ident::new(&format!("a{}", x), Span::call_site()))
                        .collect::<Vec<_>>();
                    let b = (0..fields.unnamed.len())
                        .map(|x| syn::Ident::new(&format!("b{}", x), Span::call_site()))
                        .collect::<Vec<_>>();
                    quote! {
                        (Self::#ident(#(#a),*), Self::#ident(#(#b),*)) =>
                            if #(#a == #b)&&* {
                                Self::Repr::NoChange
                            } else {
                                #diff_ident::#ident(#(#a.diff(#b)),*)
                            },
                        (_, Self::#ident(#(#b),*)) =>
                            #diff_ident::#ident(#(<#t as Diff>::identity().diff(#b)),*)
                    }
                }
                Fields::Unit => quote! {
                    (Self::#ident, Self::#ident) => Self::Repr::NoChange,
                    (_, Self::#ident) => Self::Repr::#ident
                },
            }
        })
        .collect::<Vec<Tokens>>();

    let variants_apply_arms = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let t = fields.named.iter().map(|x|&x.ty).collect::<Vec<_>>();
                let i = fields.named.iter().map(|x|&x.ident).collect::<Vec<_>>();
                let a = fields.named.iter()
                    .map(|x| syn::Ident::new(&format!("a_{}", x.ident.as_ref().unwrap()), Span::call_site()))
                    .collect::<Vec<_>>();
                let b = fields.named.iter()
                    .map(|x| syn::Ident::new(&format!("b_{}", x.ident.as_ref().unwrap()), Span::call_site()))
                    .collect::<Vec<_>>();
                quote! {
                    Self::Repr::#ident{#(#i: #b),*} => {
                        if let Self::#ident{#(#i: #a),*} = self {
                            #(#a.apply(#b));*;
                        } else {
                            *self = Self::#ident{#(#i: <#t as Diff>::identity().apply_new(#b)),*};
                        }
                    }
                }
            },
            Fields::Unnamed(fields) => {
                let t = fields.unnamed.iter().map(|x|&x.ty).collect::<Vec<_>>();
                let a = (0..fields.unnamed.len())
                    .map(|x| syn::Ident::new(&format!("a{}", x), Span::call_site()))
                    .collect::<Vec<_>>();
                let b = (0..fields.unnamed.len())
                    .map(|x| syn::Ident::new(&format!("b{}", x), Span::call_site()))
                    .collect::<Vec<_>>();
                quote! {
                    Self::Repr::#ident(#(#b),*) => {
                        if let Self::#ident(#(#a),*) = self {
                            #(#a.apply(#b));*;
                        } else {
                            *self = Self::#ident(#(<#t as Diff>::identity().apply_new(#b)),*);
                        }
                    }
                }
            },
            Fields::Unit => quote! {
                Self::Repr::#ident => *self = Self::#ident
            },
        }
    }).collect::<Vec<Tokens>>();

    let identity = match &first.fields {
        Fields::Named(_) => quote! {
            Self::#first_ident { #(#first_names: <#first_types as Diff>::identity()),* }
        },
        Fields::Unnamed(_) => quote! {
            Self::#first_ident ( #(<#first_types as Diff>::identity()),* )
        },
        Fields::Unit => quote! {
            Self::#first_ident
        },
    };

    quote! {
        #(#attr)*
        #visibility enum #diff_ident #type_generics #where_clause {
            NoChange,
            #(#variants_type_decl),*,
        }

        impl #impl_generics Diff for #ident #type_generics #where_clause {
            type Repr = #diff_ident #type_generics;

            fn diff(&self, other: &Self) -> Self::Repr {
                match (self, other) {
                    #(#variants_diff_arms),*,
                }
            }

            fn apply(&mut self, diff: &Self::Repr) {
                match diff {
                    Self::Repr::NoChange => {},
                    #(#variants_apply_arms),*,
                }
            }

            fn identity() -> Self {
                #identity
            }
        }
    }
}

#[derive(Default)]
struct StructAttributesRaw {
    name: Option<Ident>,
    visibility: Option<Visibility>,
    attrs: OuterAttributes,
}

/// Contains top-level attributes for structs and enums
struct StructAttributes {
    name: Ident,
    visibility: Visibility,
    attrs: Vec<Attribute>,
}

#[derive(Default)]
struct NamedFieldAttributesRaw {
    name: Option<Ident>,
    visibility: Option<Visibility>,
    attrs: OuterAttributes,
}

/// Customizable attributes for fields
struct NamedFieldAttributes {
    name: Ident,
    visibility: Visibility,
    attrs: Vec<Attribute>,
}

#[derive(Default)]
struct UnnamedFieldAttributesRaw {
    visibility: Option<Visibility>,
    attrs: OuterAttributes,
}

/// Customizable attributes for fields
struct UnnamedFieldAttributes {
    visibility: Visibility,
    attrs: Vec<Attribute>,
}

/// A named attribute with unspecified tokens inside parentheses
struct ParenAttr {
    name: Ident,
    tokens: Tokens,
}

impl Parse for ParenAttr {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let name = input.parse()?;
        let content;
        parenthesized!(content in input);
        Ok(ParenAttr {
            name,
            tokens: content.parse::<Tokens>()?,
        })
    }
}

#[derive(Default)]
struct OuterAttributes(Vec<Attribute>);

impl Parse for OuterAttributes {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        Ok(Self(input.call(Attribute::parse_outer)?))
    }
}

fn parse_struct_attributes(
    attrs: &[Attribute],
    vis: Visibility,
    ident: &Ident,
) -> SynResult<StructAttributes> {
    let mut raw = StructAttributesRaw::default();
    attrs
        .iter()
        .filter(|a| a.path.is_ident("diff"))
        .try_for_each(|attr| {
            let attr_named: ParenAttr = attr.parse_args()?;
            let name = attr_named.name.to_string();

            match name.as_ref() {
                "attr" => {
                    raw.attrs = parse(attr_named.tokens.into())?
                }
                "name" => {
                    raw.name = Some(parse(attr_named.tokens.into())?)
                }
                "visibility" => {
                    raw.visibility = Some(parse(attr_named.tokens.into())?)
                }
                _ => {
                    return Err(
                        Error::new(attr_named.name.span(),
                        format!("Attribute name {} was not expected. Possible attribute names: attr, name, visibility", name)
                    ))
                },
            }

            Ok(())
        })?;
    Ok(StructAttributes {
        name: raw.name.unwrap_or(format_ident!("{}Diff", ident)),
        visibility: raw.visibility.unwrap_or(vis),
        attrs: raw.attrs.0,
    })
}

fn parse_named_field_attributes(
    attrs: &[Attribute],
    vis: &Visibility,
    ident: &Ident,
) -> SynResult<NamedFieldAttributes> {
    let mut raw = NamedFieldAttributesRaw::default();
    attrs
        .iter()
        .filter(|a| a.path.is_ident("diff"))
        .try_for_each(|attr| {
            let attr_named: ParenAttr = attr.parse_args()?;
            let name = attr_named.name.to_string();

            match name.as_ref() {
                "attr" => {
                    raw.attrs = parse(attr_named.tokens.into())?
                }
                "name" => {
                    raw.name = Some(parse(attr_named.tokens.into())?)
                }
                "visibility" => {
                    raw.visibility = Some(parse(attr_named.tokens.into())?)
                }
                _ => {
                    return Err(
                        Error::new(attr_named.name.span(),
                        format!("Attribute name {} was not expected. Possible attribute names: attr, name, visibility", name)
                    ))
                },
            }

            Ok(())
        })?;
    Ok(NamedFieldAttributes {
        name: raw.name.unwrap_or_else(|| ident.clone()),
        visibility: raw.visibility.unwrap_or_else(|| vis.clone()),
        attrs: raw.attrs.0,
    })
}

fn parse_unnamed_field_attributes(
    attrs: &[Attribute],
    vis: &Visibility,
) -> SynResult<UnnamedFieldAttributes> {
    let mut raw = UnnamedFieldAttributesRaw::default();
    attrs
        .iter()
        .filter(|a| a.path.is_ident("diff"))
        .try_for_each(|attr| {
            let attr_named: ParenAttr = attr.parse_args()?;
            let name = attr_named.name.to_string();

            match name.as_ref() {
                "attr" => {
                    raw.attrs = parse(attr_named.tokens.into())?
                }
                "visibility" => {
                    raw.visibility = Some(parse(attr_named.tokens.into())?)
                }
                _ => {
                    return Err(
                        Error::new(attr_named.name.span(),
                        format!("Attribute name {} was not expected. Possible attribute names: attr, visibility", name)
                    ))
                },
            }

            Ok(())
        })?;
    Ok(UnnamedFieldAttributes {
        visibility: raw.visibility.unwrap_or_else(|| vis.clone()),
        attrs: raw.attrs.0,
    })
}
