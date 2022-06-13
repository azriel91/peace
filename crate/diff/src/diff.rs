use crate::Differ;

/// A trait to diff and apply diffs between two structs.
///
/// The derive macro can be used on structs when all fields of the struct
/// implement Diff Implementations are provided for bools, numeric types, Option
/// types, and HashMaps
pub trait Diff: Sized {
    /// The type associated with the structs' difference
    type Repr;

    /// Produces a diff between two structs
    fn diff(&self, other: &Self) -> Self::Repr;

    /// Produces a diff between two structs, using an external diffing
    /// implementation
    fn diff_custom<D: Differ<Self>>(&self, other: &Self, visitor: &D) -> D::Repr {
        visitor.diff(self, other)
    }

    /// Applies the diff directly to the struct
    fn apply(&mut self, diff: &Self::Repr);

    /// Applies the diff directly to the struct, using an external diffing
    /// implementation
    fn apply_custom<D: Differ<Self>>(&mut self, diff: &D::Repr, visitor: &D) {
        visitor.apply(self, diff)
    }

    /// Applies the diff to the struct and produces a new struct
    fn apply_new(&self, diff: &Self::Repr) -> Self {
        let mut new = Self::identity();
        new.apply(&new.diff(self));
        new.apply(diff);
        new
    }

    /// Applies the diff to the struct and produces a new struct, using an
    /// external diffing implementation
    fn apply_new_custom<D: Differ<Self>>(&self, diff: &D::Repr, visitor: &D) -> Self {
        let mut new = Self::identity();
        new.apply_custom(&new.diff_custom(self, visitor), visitor);
        new.apply_custom(diff, visitor);
        new
    }

    /// The identity element of the struct
    ///
    /// ```rust
    /// use peace_diff::Diff;
    ///
    /// let s = 42;
    /// let i = <i32 as Diff>::identity();
    ///
    /// assert_eq!(i.apply_new(&i.diff(&s)), s);
    /// ```
    ///
    /// or mathematically speaking, `i + (s - i) = s`
    fn identity() -> Self;
}
