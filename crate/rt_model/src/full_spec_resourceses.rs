use std::ops::{Deref, DerefMut};

use peace_data::Resources;
use rt_vec::{BorrowFail, Ref, RefMut, RtVec};

use crate::FullSpecRtId;

/// Resources for all `FullSpec`s. `RtVec<Resources>` newtype.
///
/// Pronounced as full spec *resoursees*, this is a double plural -- each
/// `Resources` map stores the resources for a `FullSpec`, and
#[derive(Debug, Default)]
pub struct FullSpecResourceses(RtVec<Resources>);

impl FullSpecResourceses {
    /// Returns a new `FullSpecResourceses`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `FullSpecResourceses` with the specified capacity.
    ///
    /// The `FullSpecResourceses` will be able to hold at least capacity
    /// elements without reallocating. If capacity is 0, the vec will not
    /// allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(RtVec::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> RtVec<Resources> {
        self.0
    }

    /// Returns a reference to the value corresponding to the index.
    ///
    /// See [`try_borrow`] for a non-panicking version of this function.
    ///
    /// # Panics
    ///
    /// * Panics if the value is already borrowed mutably.
    ///
    /// [`try_borrow`]: Self::try_borrow
    pub fn borrow(&self, full_spec_rt_id: FullSpecRtId) -> Ref<Resources> {
        self.0.borrow(full_spec_rt_id.index())
    }

    /// Returns a reference to the value if it exists and is not mutably
    /// borrowed.
    ///
    /// * Returns `BorrowFail::ValueNotFound` if `index` is out of bounds.
    /// * Returns `BorrowFail::BorrowConflictImm` if the value is already
    ///   borrowed mutably.
    pub fn try_borrow(&self, full_spec_rt_id: FullSpecRtId) -> Result<Ref<Resources>, BorrowFail> {
        self.0.try_borrow(full_spec_rt_id.index())
    }

    /// Returns a reference to the value if it exists and is not borrowed.
    ///
    /// See [`try_borrow_mut`] for a non-panicking version of this function.
    ///
    /// # Panics
    ///
    /// Panics if the value is already borrowed either immutably or mutably.
    ///
    /// [`try_borrow_mut`]: Self::try_borrow_mut
    pub fn borrow_mut(&self, full_spec_rt_id: FullSpecRtId) -> RefMut<Resources> {
        self.0.borrow_mut(full_spec_rt_id.index())
    }

    /// Returns a mutable reference to the value if it is successfully borrowed
    /// mutably.
    ///
    /// * Returns `BorrowFail::ValueNotFound` if `index` is out of bounds.
    /// * Returns `BorrowFail::BorrowConflictMut` if the value is already
    ///   borrowed.
    pub fn try_borrow_mut(
        &self,
        full_spec_rt_id: FullSpecRtId,
    ) -> Result<RefMut<Resources>, BorrowFail> {
        self.0.try_borrow_mut(full_spec_rt_id.index())
    }

    /// Returns a reference to the value corresponding to the index if it is not
    /// borrowed.
    ///
    /// Returns `None` if `index` is out of bounds.
    ///
    /// See [`try_borrow`] for a version of this that returns a `Result` with
    /// the reason why the value is not returned.
    ///
    /// # Panics
    ///
    /// Panics if the value is already borrowed mutably.
    ///
    /// [`try_borrow`]: Self::try_borrow
    pub fn get(&self, full_spec_rt_id: FullSpecRtId) -> Option<Ref<Resources>> {
        self.0.get(full_spec_rt_id.index())
    }

    /// Retrieves a value without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_mut(&mut self, full_spec_rt_id: FullSpecRtId) -> Option<&mut Resources> {
        self.0.get_mut(full_spec_rt_id.index())
    }
}

impl Deref for FullSpecResourceses {
    type Target = RtVec<Resources>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FullSpecResourceses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<RtVec<Resources>> for FullSpecResourceses {
    fn from(resourceses: RtVec<Resources>) -> Self {
        Self(resourceses)
    }
}
