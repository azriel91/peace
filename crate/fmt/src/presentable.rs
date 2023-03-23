pub use self::{
    bold::Bold, code_inline::CodeInline, heading::Heading, heading_level::HeadingLevel,
    list_numbered::ListNumbered,
};

use serde::Serialize;

use crate::Presenter;

mod bold;
mod code_inline;
mod heading;
mod heading_level;
mod list_numbered;
mod tuple_impl;

/// A type that is presentable to a user.
///
/// This is analogous in concept to `std::fmt::Display`, and in implementation
/// to `std::fmt::Debug`, with the difference that instead of formatting an
/// unstyled string, implementations register how they are presented with a
/// [`Presenter`].
///
/// # Implementors
///
/// Currently it is not possible to store `Box<dyn Presentable>`, because of the
/// following:
///
/// * `Presentable` implies `Serialize`.
/// * `Presentable::present<'_, PR>` and `Serialize::serialize<S>` are generic
///   trait methods.
/// * This means different concrete implementations of `Presentable`/`Serialize`
///   will have different vtables (with different sizes), and Rust returns the
///   following compilation error:
///
///     ```text
///     error[E0038]: the trait `Presentable` cannot be made into an object
///     ```
///
///     See <https://doc.rust-lang.org/error_codes/E0038.html>.
///
/// It is possible to store `Vec<Box<T>>` for any `T: Presentable` and invoke
/// `boxed.present()`.
///
/// # Examples
///
/// Presenting a list item with a name and value:
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
///
/// # Design
///
/// `Presentable` implies `Serialize` because it is beneficial for anything that
/// is presented to the user, to be able to be stored, so that it can be
/// re-presented to them at a later time. However, it currently doesn't imply
/// `DeserializeOwned`, which may mean the serialization half may not be
/// worthwhile, and `Presentable` wrapper types may just wrap borrowed data.
///
/// Previously, this was implemented as `Presentable: Serialize +
/// OwnedDeserialize`, with `OwnedDeserialize` being the following trait:
///
/// ```rust
/// use serde::de::DeserializeOwned;
///
/// /// Marker trait to allow `str` to implement `Presentable`.
/// ///
/// /// 1. `str` is not an owned type, so it doesn't `impl DeserializeOwned`.
/// /// 2. We don't want to relax the constraints such that `Presentable` doesn't
/// /// imply `DeserializeOwned`.
/// pub trait OwnedDeserialize {}
///
/// impl<T> OwnedDeserialize for T
/// where
///     T: ToOwned + ?Sized,
///     <T as ToOwned>::Owned: DeserializeOwned,
/// {
/// }
/// ```
///
/// However, because stateful deserialized types such as `TypeMap` don't
/// implement `DeserializeOwned`, so any types based on that such as `States`
/// would not be able to implement `Presentable` with this bound.
#[async_trait::async_trait(?Send)]
pub trait Presentable: Serialize {
    /// Presents this data type to the user.
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>;
}

#[async_trait::async_trait(?Send)]
impl<T> Presentable for &T
where
    T: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        // The `*` is important -- without it the `present` will stack overflow.
        (*self).present(presenter).await
    }
}

#[async_trait::async_trait(?Send)]
impl Presentable for &str {
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

/// **Note:** `T` must be `Sized`, as `Vec`s store sized types.
#[async_trait::async_trait(?Send)]
impl<T> Presentable for Vec<T>
where
    T: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.list_numbered(self.iter()).await
    }
}

/// **Note:** `T` must be `Sized`, as arrays store sized types.
#[async_trait::async_trait(?Send)]
impl<T> Presentable for [T]
where
    T: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter.list_numbered(self.iter()).await
    }
}
