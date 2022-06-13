/// A trait allowing a custom struct to handle the diffing implementation for a
/// type.
pub trait Differ<T> {
    type Repr;

    fn diff(&self, a: &T, b: &T) -> Self::Repr;

    fn apply(&self, a: &mut T, b: &Self::Repr);
}
