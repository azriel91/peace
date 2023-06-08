//! Type states for [`States`].
//!
//! These distinguish between the purposes of each `States` map.
//!
//! [`States`]: crate::states::States

use serde::{Deserialize, Serialize};

/// Stored current states of items.
#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentStored;

/// Current states of items.
#[derive(Debug, Deserialize, Serialize)]
pub struct Current;

/// Stored goal states of items.
#[derive(Debug, Deserialize, Serialize)]
pub struct GoalStored;

/// Goal states of items.
#[derive(Debug, Deserialize, Serialize)]
pub struct Goal;

/// States of items after running the `EnsureCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Ensured;

/// States of items after dry-running `EnsureCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct EnsuredDry;

/// States of items after running the `CleanCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Cleaned;

/// States of items after dry-running `CleanCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct CleanedDry;
