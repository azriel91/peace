use crate::AnySpecDataType;

/// Runtime logic of how to look up values for each field in this struct.
///
/// This trait is automatically implemented by `#[derive(Params)]` on an
/// `Item::Params`, as well as in the `peace_params` crate for standard
/// library types.
pub trait AnySpecRt {
    /// Whether this `Spec` is usable to resolve values.
    ///
    /// This is only `false` for `*Spec::Stored`.
    ///
    /// After merging, `*Spec::Stored` will be replaced with whatever the
    /// `other` `*Spec` is. If after merging, the `*Spec` is `Stored`, then the
    /// `*Spec` will not be usable as there *wasn't* anything stored in the
    /// first place.
    fn is_usable(&self) -> bool;
    /// Deep merges the provided `AnySpecRt` with `self`, where `self` takes
    /// priority, except for `Self::Stored`.
    ///
    /// This means where `self` is `Self::Value`, `Self::InMemory`,
    /// `Self::MappingFn`, and `Self::FieldWise`, these would take priority over
    /// any stored item variants.
    ///
    /// For `Self::FieldWise`, a recursive merge would happen per field
    /// `ValueSpec`.
    ///
    /// # Design
    ///
    /// This can't be `Self` or `&Self` because that makes the trait non-object
    /// safe. Adding a `where: Self: Sized` bound prevents the method from being
    /// called from `cmd_ctx_builder`.
    fn merge(&mut self, other: &dyn AnySpecDataType);
}

impl<T> AnySpecRt for Box<T>
where
    T: AnySpecRt,
{
    fn is_usable(&self) -> bool {
        self.as_ref().is_usable()
    }

    fn merge(&mut self, other: &dyn AnySpecDataType)
    where
        Self: Sized,
    {
        self.as_mut().merge(other)
    }
}
