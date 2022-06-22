use std::{ops::Deref, sync::Arc};

use crate::Diff;

impl<T> Diff for Arc<T>
where
    T: Diff + Clone,
{
    type Repr = T::Repr;

    fn diff(&self, other: &Self) -> Self::Repr {
        self.deref().diff(other.deref())
    }

    fn apply(&mut self, diff: &Self::Repr) {
        match Arc::get_mut(self) {
            Some(m) => m.apply(diff),
            None => {
                let mut x = (**self).clone();
                x.apply(diff);
                *self = Arc::new(x);
            }
        }
    }

    fn identity() -> Self {
        Arc::new(T::identity())
    }
}
