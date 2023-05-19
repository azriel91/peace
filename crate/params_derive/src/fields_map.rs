use syn::{Attribute, Field, Fields, FieldsNamed, FieldsUnnamed, Path, Type};

use crate::util::{field_spec_ty, is_phantom_data, is_serde_bound_attr};

/// Maps fields into different fields, wrapping them with the appropriate braces
/// / parenthesis if necessary.
pub fn fields_map<F>(fields: &Fields, f: F) -> Fields
where
    F: Fn(&Field) -> proc_macro2::TokenStream,
{
    let fields_mapped = fields.iter().map(|field| (field, f(field)));
    match fields {
        Fields::Named(_fields_named) => {
            let fields_mapped = fields_mapped.map(|(field, expr)| {
                let field_name = &field.ident;
                quote!(#field_name: #expr)
            });
            let fields_named: FieldsNamed = parse_quote!({
                #(#fields_mapped,)*
            });
            Fields::from(fields_named)
        }
        Fields::Unnamed(_fields_unnamed) => {
            let fields_mapped = fields_mapped.map(|(_field, expr)| expr);
            let fields_unnamed: FieldsUnnamed = parse_quote!((#(#fields_mapped,)*));
            Fields::from(fields_unnamed)
        }
        Fields::Unit => fields.clone(),
    }
}

pub fn fields_to_optional(fields: &mut Fields) {
    fields_each_map(fields, |field| {
        let field_ty = &field.ty;
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            parse_quote!(Option<#field_ty>)
        }
    })
}

/// Maps each field from `MyType` to `ValueSpec<MyType>`.
///
/// If the type is marked with `#[value_spec(fieldless)]`, then it is wrapped
/// as `ValueSpec<MyTypeWrapper>`.
pub fn fields_to_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    fields_each_map(fields, |field| {
        let field_ty = &field.ty;
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            // For external types, we don't know if they implement `ValueSpec`, so we treat
            // fields with this attribute as `ParamsSpecFieldless`. Have tried to hold a
            // `Box<dyn ValueSpecRt>`, so it could delegate to either the `ValueSpec` or
            // `ParamsSpecFieldless`. However, it makes it hard to deserialize from a
            // serialized `ValueSpec` because we would have to generate a concrete type with
            // the `field_ty`, which may make it impossible to handle upgrades / evolving
            // params types.
            //
            // In #119, we tried using `ValueSpec` for recursive value spec resolution, but
            // it proved too difficult.
            syn::parse2(field_spec_ty(peace_params_path, field_ty))
                .expect("Failed to parse field to value spec.")
        }
    })
}

/// Maps each field from `MyType` to `Option<ValueSpec<MyType>>`.
///
/// If the type is marked with `#[value_spec(fieldless)]`, then it is wrapped
/// as `Option<ValueSpec<MyTypeWrapper>>`.
///
/// Fieldless types -- stdlib types or types annotated with
/// `#[value_spec(fieldless)]` -- are wrapped as:
///
/// ```rust,ignore
/// `Option<ParamsSpecFieldless<MyTypeWrapper>>`.
/// ```
pub fn fields_to_optional_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    fields_each_map(fields, |field| {
        field_to_optional_value_spec(field, peace_params_path)
    })
}

/// Maps a field to `Option<ValueSpec<#field_ty>>`, `PhantomData` is returned as
/// is.
pub fn field_to_optional_value_spec(field: &Field, peace_params_path: &Path) -> Type {
    let field_ty = &field.ty;
    if is_phantom_data(field_ty) {
        field_ty.clone()
    } else {
        let field_spec_ty = field_spec_ty(peace_params_path, field_ty);
        parse_quote!(Option<#field_spec_ty>)
    }
}

fn fields_each_map<F>(fields: &mut Fields, f: F)
where
    F: Fn(&Field) -> Type,
{
    match fields {
        Fields::Named(fields_named) => {
            fields_named.named.iter_mut().for_each(|field| {
                field.ty = f(field);

                // Don't copy across most attributes.
                // The only attribute we copy across is `#[serde(bound = "..")]`
                field.attrs = field
                    .attrs
                    .drain(..)
                    .filter(is_serde_bound_attr)
                    .collect::<Vec<Attribute>>();
            });
        }
        Fields::Unnamed(fields_unnamed) => {
            fields_unnamed.unnamed.iter_mut().for_each(|field| {
                field.ty = f(field);

                // Don't copy across most attributes.
                // The only attribute we copy across is `#[serde(bound = "..")]`
                field.attrs = field
                    .attrs
                    .drain(..)
                    .filter(is_serde_bound_attr)
                    .collect::<Vec<Attribute>>();
            });
        }
        Fields::Unit => {}
    }
}
