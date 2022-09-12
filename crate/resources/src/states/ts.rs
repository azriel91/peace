//! Type states for [`States`].
//!
//! These distinguish between the purposes of each `States` map.
//!
//! [`States`]: crate::states::States

use serde::{Deserialize, Serialize};

/// Current states of items.
#[derive(Debug, Deserialize, Serialize)]
pub struct Current;

/// Desired states of items.
#[derive(Debug, Deserialize, Serialize)]
pub struct Desired;

/// States of items after running the `EnsureCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Ensured;

/// States of items after dry-running `EnsureCmd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct EnsuredDry;
