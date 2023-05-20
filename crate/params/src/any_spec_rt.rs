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
}

impl<T> AnySpecRt for Box<T>
where
    T: AnySpecRt + ?Sized,
{
    fn is_usable(&self) -> bool {
        self.as_ref().is_usable()
    }
}
