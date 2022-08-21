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

/// `ItemSpec::setup` has been run over [`Resources`].
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
/// Implies [`SetUp`], [`WithStates`], and [`WithStatesDesired`].
///
/// [`Resources`]: crate::Resources
/// [`States`]: crate::States
/// [`StatesDesired`]: crate::StatesDesired
#[derive(Debug)]
pub struct WithStatesCurrentAndDesired;

/// [`Resources`] contains [`States`], [`StatesDesired`], and [`StateDiffs`].
///
/// Implies [`SetUp`] and [`WithStatesNowAndDesired`].
///
/// [`Resources`]: crate::Resources
/// [`States`]: crate::States
/// [`StatesDesired`]: crate::StatesDesired
/// [`StateDiffs`]: crate::StateDiffs
#[derive(Debug)]
pub struct WithStateDiffs;

/// [`Resources`] have been run through `EnsureCmd::exec_dry`.
///
/// Implies [`SetUp`], [`WithStatesNowAndDesired`], and [`WithStateDiffs`].
///
/// [`Resources`]: crate::Resources
/// [`States`]: crate::States
/// [`StatesDesired`]: crate::StatesDesired
/// [`StateDiffs`]: crate::StateDiffs
#[derive(Debug)]
pub struct EnsuredDry;

/// [`Resources`] have been run through `EnsureCmd::exec`.
///
/// This means `States` is now stale, and [`StatesEnsured`] holds the up to date
/// states.
///
/// Implies [`SetUp`], [`WithStatesNowAndDesired`], and [`WithStateDiffs`].
///
/// [`Resources`]: crate::Resources
/// [`States`]: crate::States
/// [`StatesDesired`]: crate::StatesDesired
/// [`StatesEnsured`]: crate::StatesEnsured
/// [`StateDiffs`]: crate::StateDiffs
#[derive(Debug)]
pub struct Ensured;
