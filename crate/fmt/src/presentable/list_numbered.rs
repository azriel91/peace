use serde::{Deserialize, Serialize};

use crate::{Presentable, Presenter};

/// Presents the given iterator as a numbered list.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListNumbered<P>(Vec<P>);

impl<P> ListNumbered<P> {
    /// Returns a new `ListNumbered` wrapper.
    pub fn new(presentables: Vec<P>) -> Self {
        Self(presentables)
    }
}

#[async_trait::async_trait(?Send)]
impl<P> Presentable for ListNumbered<P>
where
    P: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.list_numbered(&self.0).await
    }
}
