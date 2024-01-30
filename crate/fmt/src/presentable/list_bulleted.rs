use serde::{Deserialize, Serialize};

use crate::{Presentable, Presenter};

/// Presents the given iterator as a bulleted list.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListBulleted<P>(Vec<P>);

impl<P> ListBulleted<P> {
    /// Returns a new `ListBulleted` wrapper.
    pub fn new(presentables: Vec<P>) -> Self {
        Self(presentables)
    }
}

#[async_trait::async_trait(?Send)]
impl<P> Presentable for ListBulleted<P>
where
    P: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.list_bulleted(&self.0).await
    }
}

impl<P> From<Vec<P>> for ListBulleted<P>
where
    P: Presentable,
{
    fn from(presentables: Vec<P>) -> Self {
        Self(presentables)
    }
}
