use serde::{Deserialize, Serialize};

use crate::{Presentable, Presenter};

/// Presents the given iterator as an aligned bulleted list.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListBulletedAligned<P0, P1>(Vec<(P0, P1)>);

impl<P0, P1> ListBulletedAligned<P0, P1> {
    /// Returns a new `ListBulletedAligned` wrapper.
    pub fn new(presentables: Vec<(P0, P1)>) -> Self {
        Self(presentables)
    }
}

#[async_trait::async_trait(?Send)]
impl<P0, P1> Presentable for ListBulletedAligned<P0, P1>
where
    P0: Presentable,
    P1: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.list_bulleted_aligned(&self.0).await
    }
}

impl<P0, P1> From<Vec<(P0, P1)>> for ListBulletedAligned<P0, P1>
where
    P0: Presentable,
    P1: Presentable,
{
    fn from(presentables: Vec<(P0, P1)>) -> Self {
        Self(presentables)
    }
}
