use syn::{
    parse::{Parse, ParseStream},
    LitStr,
};

pub(crate) struct LitStrMaybe(pub Option<LitStr>);

impl std::ops::Deref for LitStrMaybe {
    type Target = Option<LitStr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for LitStrMaybe {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let value = if input.is_empty() {
            None
        } else {
            Some(input.parse::<LitStr>()?)
        };

        Ok(LitStrMaybe(value))
    }
}
