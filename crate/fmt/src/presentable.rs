use serde::{Deserialize, Serialize};

use crate::{OwnedDeserialize, Presenter};

/// A type that is presentable to a user.
///
/// This is analogous to `std::fmt::Display`, with the difference that instead
/// of formatting an unstyled string, implementations register how they are
/// presented with a [`Presenter`].
///
/// # Implementors
///
/// ```rust
/// # use peace_fmt::{Presentable, Presenter};
/// # use serde::{Deserialize, Serialize};
/// // use peace::fmt::{Presentable, Presenter};
///
/// #[derive(Clone, Deserialize, Serialize)]
/// struct Item {
///     name: String,
///     desc: String,
/// }
///
/// #[async_trait::async_trait(?Send)]
/// impl Presentable for Item {
///     async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
///     where
///         PR: Presenter<'output>,
///     {
///         presenter.name(&self.name).await?;
///         presenter.text(": ").await?;
///         presenter.text(&self.desc).await?;
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait(?Send)]
pub trait Presentable: Serialize + OwnedDeserialize {
    /// Presents this data type to the user.
    async fn present<'output, 't, PR>(&'t self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
        't: 'output;
}

#[async_trait::async_trait(?Send)]
impl Presentable for str {
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.text(self).await
    }
}

#[async_trait::async_trait(?Send)]
impl Presentable for String {
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.text(self).await
    }
}

#[async_trait::async_trait(?Send)]
impl<T> Presentable for Vec<T>
where
    for<'de> T: Clone + Presentable + Deserialize<'de>,
{
    async fn present<'output, 't, PR>(&'t self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
        't: 'output,
    {
        presenter.list().entries(self.iter()).await.finish()
    }
}

#[async_trait::async_trait(?Send)]
impl<T> Presentable for [T]
where
    for<'de> T: Clone + Presentable + Deserialize<'de>,
{
    async fn present<'output, 't, PR>(&'t self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
        't: 'output,
    {
        presenter.list().entries(self.iter()).await.finish()
    }
}
