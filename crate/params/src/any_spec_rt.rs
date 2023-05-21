use crate::AnySpecDataType;

/// Runtime logic of how to look up values for each field in this struct.
///
/// This trait is automatically implemented by `#[derive(Params)]` on an
/// `ItemSpec::Params`, as well as in the `peace_params` crate for standard
/// library types.
pub trait AnySpecRt {
    /// Whether this `Spec` is usable to resolve values.
    ///
    /// This is only `false` for `*Spec::MappingFn`s that have been
    /// deserialized, as mapping functions cannot be deserialized back into
    /// logic without embedding a script interpreter or compiler.
    fn is_usable(&self) -> bool;
    /// Deep merges the provided `AnySpecRt` with `self`, where `self` takes
    /// priority, except for `Self::Stored`.
    ///
    /// This means where `self` is `Self::Value`, `Self::InMemory`,
    /// `Self::MappingFn`, and `Self::FieldWise`, these would take priority over
    /// any stored item spec variants.
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
