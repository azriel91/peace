use serde::{Deserialize, Serialize};

use crate::{presentable::HeadingLevel, Presentable, Presenter};

/// Presents the given string as a heading.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Heading<P> {
    /// Level of conceptual detail for a given topic.
    level: HeadingLevel,
    /// The heading level.
    presentable: P,
}

impl<P> Heading<P> {
    /// Returns a new `Heading` wrapper.
    pub fn new(level: HeadingLevel, presentable: P) -> Self {
        Self { level, presentable }
    }
}

#[async_trait::async_trait(?Send)]
impl<P> Presentable for Heading<P>
where
    P: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.heading(self.level, &self.presentable).await
    }
}
