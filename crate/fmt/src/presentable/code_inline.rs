use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Presentable, Presenter};

/// Presents the given string as inline code.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CodeInline<'s>(Cow<'s, str>);

impl<'s> CodeInline<'s> {
    /// Returns a new `Code` wrapper.
    pub fn new(s: Cow<'s, str>) -> Self {
        Self(s)
    }
}

#[async_trait::async_trait(?Send)]
impl Presentable for CodeInline<'_> {
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.code_inline(&self.0).await
    }
}
