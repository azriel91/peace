use serde::de::DeserializeOwned;

/// Marker trait to allow `str` to implement `Presentable`.
///
/// 1. `str` is not an owned type, so it doesn't `impl DeserializeOwned`.
/// 2. We don't want to relax the constraints such that `Presentable` doesn't
/// imply `DeserializeOwned`.
pub trait OwnedDeserialize {}

impl<T> OwnedDeserialize for T
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: DeserializeOwned,
{
}
