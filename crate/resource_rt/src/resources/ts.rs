//! Type states for [`Resources`].
//!
//! This allows compile time checking that [`Resources`] is in the correct state
//! before a particular `TryFnSpec`, `ApplyFns`, or `CleanOpSpec`
//! is executed with it.
//!
//! [`Resources`]: crate::Resources

/// [`Resources`] is created but not setup.
///
/// [`Resources`]: crate::Resources
#[derive(Debug)]
pub struct Empty;

/// `Item::setup` has been run over [`Resources`].
#[derive(Debug)]
pub struct SetUp;
