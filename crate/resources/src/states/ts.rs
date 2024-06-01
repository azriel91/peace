//! Type states for [`States`].
//!
//! These distinguish between the purposes of each `States` map.
//!
//! [`States`]: crate::states::States

use serde::{Deserialize, Serialize};

/// Clean / blank states of steps.
///
/// Not to be confused with [`Cleaned`].
#[derive(Debug, Deserialize, Serialize)]
pub struct Clean;

/// Stored current states of steps.
#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentStored;

/// Current states of steps.
#[derive(Debug, Deserialize, Serialize)]
pub struct Current;

/// Stored goal states of steps.
#[derive(Debug, Deserialize, Serialize)]
pub struct GoalStored;

/// Goal states of steps.
#[derive(Debug, Deserialize, Serialize)]
pub struct Goal;

/// States of steps after running the `EnsureCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Ensured;

/// States of steps after dry-running `EnsureCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct EnsuredDry;

/// States of steps after running the `CleanCmd`.
///
/// Not to be confused with [`Clean`].
#[derive(Debug, Deserialize, Serialize)]
pub struct Cleaned;

/// States of steps after dry-running `CleanCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct CleanedDry;

/// Previous states of steps.
///
/// This is intended as a record of `States` before an `ApplyCmd` (`EnsureCmd`
/// or `CleanCmd`) are run.
#[derive(Debug, Deserialize, Serialize)]
pub struct Previous;
