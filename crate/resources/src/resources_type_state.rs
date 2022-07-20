//! Type states for [`Resources`].
//!
//! This allows compile time checking that `Resources` is in the correct state
//! before a particular `FnSpec` or `OpSpec` is executed with it.
//!
//! [`Resources`]: crate::Resources

/// [`Resources`] is created but not setup.
pub struct Empty;

/// `FullSpec::setup` has been run over [`Resources`].
pub struct SetUp;

/// [`Resources`] contains [`FullSpecStates`].
///
/// Implies [`SetUp`].
///
/// [`FullSpecStates`]: crate::FullSpecStates
pub struct WithStates;
