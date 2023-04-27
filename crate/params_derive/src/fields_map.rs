use syn::{Fields, Path, Type};

use crate::util::is_phantom_data;

pub fn fields_to_optional(fields: &mut Fields) {
    fields_map(fields, |field_ty| {
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            parse_quote!(Option<#field_ty>)
        }
    })
}

pub fn fields_to_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    fields_map(fields, |field_ty| {
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            parse_quote!(#peace_params_path::ValueSpec<#field_ty>)
        }
    })
}

pub fn fields_to_optional_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    fields_map(fields, |field_ty| {
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            parse_quote!(Option<#peace_params_path::ValueSpec<#field_ty>>)
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
                // Don't copy across attributes, e.g. `#[serde(default)].
                field.attrs.clear();

                let field_ty = &mut field.ty;
                field.ty = f(field_ty);
            });
        }
        Fields::Unnamed(fields_unnamed) => {
            fields_unnamed.unnamed.iter_mut().for_each(|field| {
                // Don't copy across attributes, e.g. `#[serde(default)].
                field.attrs.clear();

                let field_ty = &field.ty;
                field.ty = f(field_ty);
            });
        }
        Fields::Unit => {}
    }
}
