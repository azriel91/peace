/// Describes how to store peace automation data.
///
/// See <https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API>.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WebStorageSpec {
    /// Use browser local storage to store peace data.
    ///
    /// Persists even when the browser is closed and reopened.
    ///
    /// * Stores data with no expiration date, and gets cleared only through
    ///   JavaScript, or clearing the Browser cache / Locally Stored Data.
    /// * Storage limit is the maximum amongst the two.
    LocalStorage,
    /// Use session storage to store peace data.
    ///
    /// Maintains a separate storage area for each given origin that's available
    /// for the duration of the page session (as long as the browser is open,
    /// including page reloads and restores)
    ///
    /// * Stores data only for a session, meaning that the data is stored until
    ///   the browser (or tab) is closed.
    /// * Data is never transferred to the server.
    /// * Storage limit is larger than a cookie (at most 5MB).
    SessionStorage,
}
