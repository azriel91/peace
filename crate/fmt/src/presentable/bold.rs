use serde::{Deserialize, Serialize};

use crate::{Presentable, Presenter};

/// Presents the given presentable as bolded.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Bold<P>(P);

impl<P> Bold<P> {
    /// Returns a new `Bold` wrapper.
    pub fn new(presentable: P) -> Self {
        Self(presentable)
    }
}

#[async_trait::async_trait(?Send)]
impl<P> Presentable for Bold<P>
where
    P: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.bold(&self.0).await
    }
}
