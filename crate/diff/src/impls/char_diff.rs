use crate::Diff;

impl Diff for char {
    type Repr = Option<char>;

    fn diff(&self, other: &Self) -> Self::Repr {
        if self != other { Some(*other) } else { None }
    }

    fn apply(&mut self, diff: &Self::Repr) {
        if let Some(diff) = diff {
            *self = *diff
        }
    }

    fn identity() -> Self {
        '\x00'
    }
}
