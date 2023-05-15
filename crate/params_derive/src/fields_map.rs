use syn::{Attribute, DeriveInput, Field, Fields, Path, Type};

use crate::util::{field_spec_ty, is_phantom_data, is_serde_bound_attr};

pub fn fields_to_optional(fields: &mut Fields) {
    fields_map(fields, |field| {
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
/// If the type is marked with `#[params(external)]`, then it is wrapped as
/// `ValueSpec<MyTypeWrapper>`.
pub fn fields_to_value_spec(
    parent_ast: Option<&DeriveInput>,
    fields: &mut Fields,
    peace_params_path: &Path,
) {
    fields_map(fields, |field| {
        let field_ty = &field.ty;
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            // For external types, we don't know if they implement `ValueSpec`, so we treat
            // fields with this attribute as `ValueSpecFieldless`. Have tried to hold a
            // `Box<dyn ValueSpecRt>`, so it could delegate to either the `ValueSpec` or
            // `ValueSpecFieldless`. However, it makes it hard to deserialize from a
            // serialized `ValueSpec` because we would have to generate a concrete type with
            // the `field_ty`, which may make it impossible to handle upgrades / evolving
            // params types.
            //
            // In #119, we tried using `ValueSpec` for recursive value spec resolution, but
            // it proved too difficult.
            syn::parse2(field_spec_ty(parent_ast, peace_params_path, field))
                .expect("Failed to parse field to value spec.")
        }
    })
}

fn fields_map<F>(fields: &mut Fields, f: F)
where
    F: Fn(&Field) -> Type,
{
    match fields {
        Fields::Named(fields_named) => {
            fields_named.named.iter_mut().for_each(|field| {
                field.ty = f(&field);

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
                field.ty = f(&field);

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
