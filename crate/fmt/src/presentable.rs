use serde::{de::DeserializeOwned, Serialize};

use crate::Presenter;

/// A type that is presentable to a user.
///
/// This is analogous to `std::fmt::Display`, with the difference that instead
/// of formatting an unstyled string, implementations register how they are
/// presented with a [`Presenter`].
///
/// # Implementors
///
/// ```rust
/// # use peace_fmt::{self as fmt, Presentable};
/// // use peace::fmt::{self, Presentable};
///
/// struct Item {
///     name: String,
///     desc: String,
/// }
///
/// impl Presentable for Item {
///     fn present(&self, presenter: &mut dyn fmt::Presenter) -> fmt::Result {
///         presenter.name(&self.name)?;
///         presenter.text(&self.desc)?;
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait(?Send)]
pub trait Presentable: Serialize + DeserializeOwned {
    /// Presents this data type to the user.
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>;
}
