//! Type states for [`Resources`].
//!
//! This allows compile time checking that [`Resources`] is in the correct state
//! before a particular `FnSpec` or `OpSpec` is executed with it.
//!
//! [`Resources`]: crate::Resources

/// [`Resources`] is created but not setup.
///
/// [`Resources`]: crate::Resources
#[derive(Debug)]
pub struct Empty;

/// `FullSpec::setup` has been run over [`Resources`].
#[derive(Debug)]
pub struct SetUp;

/// [`Resources`] contains [`States`].
///
/// Implies [`SetUp`].
///
/// [`States`]: crate::States
#[derive(Debug)]
pub struct WithStates;

/// [`Resources`] contains [`StatesDesired`].
///
/// Implies [`SetUp`].
///
/// [`Resources`]: crate::Resources
/// [`StatesDesired`]: crate::StatesDesired
#[derive(Debug)]
pub struct WithStatesDesired;

/// [`Resources`] contains [`States`] and [`StatesDesired`].
///
/// Implies [`SetUp`].
///
/// [`Resources`]: crate::Resources
/// [`States`]: crate::States
/// [`StatesDesired`]: crate::StatesDesired
#[derive(Debug)]
pub struct WithStatesNowAndDesired;

/// Sealed marker trait indicating [`Resources`] has been set up for a
/// `FullSpecGraph`.
///
/// [`Resources`]: crate::Resources
pub trait HasBeenSetUp: private::Sealed {}
impl HasBeenSetUp for SetUp {}
impl HasBeenSetUp for WithStates {}
impl HasBeenSetUp for WithStatesDesired {}
impl HasBeenSetUp for WithStatesNowAndDesired {}

/// Sealed marker trait indicating [`Resources`] contains [`States`].
///
/// [`Resources`]: crate::Resources
/// [`States`]: crate::States
pub trait HasStates: private::Sealed + HasBeenSetUp {}
impl HasStates for WithStates {}
impl HasStates for WithStatesNowAndDesired {}

/// Sealed marker trait indicating [`Resources`] contains [`StatesDesired`].
///
/// [`Resources`]: crate::Resources
/// [`StatesDesired`]: crate::StatesDesired
pub trait HasStatesDesired: private::Sealed + HasBeenSetUp {}
impl HasStatesDesired for WithStatesDesired {}
impl HasStatesDesired for WithStatesNowAndDesired {}

mod private {
    pub trait Sealed {}

    impl Sealed for super::SetUp {}
    impl Sealed for super::WithStates {}
    impl Sealed for super::WithStatesDesired {}
    impl Sealed for super::WithStatesNowAndDesired {}
}
