use syn::{Attribute, Fields, Path, Type};

use crate::{
    util::{is_external, is_phantom_data, is_serde_bound_attr},
    ExternalType,
};

pub fn fields_to_optional(fields: &mut Fields) {
    fields_map(fields, |field_ty| {
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
pub fn fields_to_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    // This has to come before `fields_map` because the attributes are cleared in
    // it.
    fields.iter_mut().for_each(|field| {
        if is_external(&field.attrs) {
            field.ty = ExternalType::wrapper_type(&field.ty);
        }
    });

    fields_map(fields, |field_ty| {
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            parse_quote!(#peace_params_path::ValueSpecFieldless<#field_ty>)
        }
    })
}

fn fields_map<F>(fields: &mut Fields, f: F)
where
    F: Fn(&Type) -> Type,
{
    match fields {
        Fields::Named(fields_named) => {
            fields_named.named.iter_mut().for_each(|field| {
                // Don't copy across most attributes.
                // The only attribute we copy across is `#[serde(bound = "..")]`
                field.attrs = field
                    .attrs
                    .drain(..)
                    .filter(is_serde_bound_attr)
                    .collect::<Vec<Attribute>>();
                field.attrs.clear();

                let field_ty = &field.ty;
                field.ty = f(field_ty);
            });
        }
        Fields::Unnamed(fields_unnamed) => {
            fields_unnamed.unnamed.iter_mut().for_each(|field| {
                // Don't copy across most attributes.
                // The only attribute we copy across is `#[serde(bound = "..")]`
                field.attrs = field
                    .attrs
                    .drain(..)
                    .filter(is_serde_bound_attr)
                    .collect::<Vec<Attribute>>();
                field.attrs.clear();

                let field_ty = &field.ty;
                field.ty = f(field_ty);
            });
        }
        Fields::Unit => {}
    }
}
