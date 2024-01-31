use std::{fmt::Debug, hash::Hash};

use serde::{de::Deserialize, Serialize};

/// Unique identifier for an `Item`.
///
/// This is a flat enum, where each variant represents an item managed by the
/// automation software.
///
/// # Examples
///
/// The following
///
/// ```rust
/// use peace_core::ItemId;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
/// #[serde(rename_all = "snake_case")]
/// pub enum EnvmanItemId {
///     AppDownload,
///     AppExtract,
///     IamPolicy,
///     IamRole,
///     InstanceProfile,
///     S3Bucket,
///     S3Object,
/// }
/// ```
///
/// # Design Note
///
/// TODO: Experiment with upgrades.
///
/// For compatibility and migrating item IDs deployed with old versions of
/// the automation software, experiment with the following:
///
/// * developers should provide a `#[serde(from = "FromType")]` implementation,
///   where the `FromType` contains the `ItemId`s from previous automation
///   software versions.
/// * the `ItemId` implementation is a hierarchical enum, with a variant for
///   each version of the automation software's items.
pub trait ItemId:
    Clone + Copy + Debug + Hash + PartialEq + Eq + for<'de> Deserialize<'de> + Serialize + 'static
{
}

impl<T> ItemId for T where
    T: Clone
        + Copy
        + Debug
        + Hash
        + PartialEq
        + Eq
        + for<'de> Deserialize<'de>
        + Serialize
        + 'static
{
}
