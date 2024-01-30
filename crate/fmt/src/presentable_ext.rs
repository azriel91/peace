use crate::{Either, Presentable};

/// Additional functionality for `Presentable` types.
pub trait PresentableExt {
    /// Wraps this `Presentable` in an `Either`, making it the left-hand variant
    /// of that `Either`.
    ///
    /// This can be used in combination with the `right_presentable` method to
    /// write `if` statements that evaluate to different `Presentable`s in
    /// different branches.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use peace_fmt::{Either, Presentable, PresentableExt};
    ///
    /// let cond = true;
    ///
    /// let presentable = if cond {
    ///     Bold::new(String::from("a")).left_presentable();
    /// } else {
    ///     CodeInline::new("b".into()).right_presentable();
    /// };
    ///
    /// presentln!(output, &presentable);
    /// ```
    fn left_presentable<B>(self) -> Either<Self, B>
    where
        B: Presentable,
        Self: Sized;

    /// Wraps this `Presentable` in an `Either`, making it the right-hand
    /// variant of that `Either`.
    ///
    /// This can be used in combination with the `left_presentable` method to
    /// write `if` statements that evaluate to different `Presentable`s in
    /// different branches.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use peace_fmt::{Either, Presentable, PresentableExt};
    ///
    /// let cond = true;
    ///
    /// let presentable = if cond {
    ///     Bold::new(String::from("a")).left_presentable();
    /// } else {
    ///     CodeInline::new("b".into()).right_presentable();
    /// };
    ///
    /// presentln!(output, &presentable);
    /// ```
    fn right_presentable<A>(self) -> Either<A, Self>
    where
        A: Presentable,
        Self: Sized;
}

impl<T> PresentableExt for T
where
    T: Presentable,
{
    fn left_presentable<B>(self) -> Either<Self, B>
    where
        B: Presentable,
        Self: Sized,
    {
        Either::Left(self)
    }

    fn right_presentable<A>(self) -> Either<A, Self>
    where
        A: Presentable,
        Self: Sized,
    {
        Either::Right(self)
    }
}
