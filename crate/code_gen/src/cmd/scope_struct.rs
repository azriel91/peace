use syn::{
    parse::{Parse, ParseStream},
    ItemStruct,
};

use crate::cmd::Scope;

/// The scope struct declaration and its scope.
#[derive(Clone)]
pub struct ScopeStruct {
    /// The proc macro input struct declaration.
    item_struct: ItemStruct,
    /// Scope of the struct.
    scope: Scope,
}

impl ScopeStruct {
    pub fn item_struct(&self) -> &ItemStruct {
        &self.item_struct
    }

    pub fn item_struct_mut(&mut self) -> &mut ItemStruct {
        &mut self.item_struct
    }

    pub fn scope(&self) -> Scope {
        self.scope
    }
}

const SCOPE_INVALID: &str = "expected struct to be one of:\n\
    \n\
    * `pub struct MultiProfileNoFlowBuilder;`\n\
    * `pub struct MultiProfileSingleFlowBuilder;`\n\
    * `pub struct NoProfileNoFlowBuilder;`\n\
    * `pub struct SingleProfileNoFlowBuilder;`\n\
    * `pub struct SingleProfileSingleFlowBuilder;`\n\
    \n\
    ";

impl Parse for ScopeStruct {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let item_struct = input.parse::<ItemStruct>()?;
        let struct_ident = &item_struct.ident;

        if !matches!(item_struct.fields, syn::Fields::Unit) {
            return Err(input.error(format!("expected `{struct_ident}` to be a unit struct.")));
        }

        let scope = if struct_ident == "MultiProfileNoFlowBuilder" {
            Scope::MultiProfileNoFlow
        } else if struct_ident == "MultiProfileSingleFlowBuilder" {
            Scope::MultiProfileSingleFlow
        } else if struct_ident == "NoProfileNoFlowBuilder" {
            Scope::NoProfileNoFlow
        } else if struct_ident == "SingleProfileNoFlowBuilder" {
            Scope::SingleProfileNoFlow
        } else if struct_ident == "SingleProfileSingleFlowBuilder" {
            Scope::SingleProfileSingleFlow
        } else {
            return Err(input.error(SCOPE_INVALID));
        };
        if !input.is_empty() {
            return Err(input.error(SCOPE_INVALID));
        }

        Ok(ScopeStruct { item_struct, scope })
    }
}
