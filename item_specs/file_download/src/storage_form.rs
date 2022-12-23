use serde::{Deserialize, Serialize};

/// Form to store the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum StorageForm {
    /// Download and store the response text as-is.
    ///
    /// This must only be used if the response is valid UTF-8.
    Text,
    /// Base64 encode the response bytes.
    ///
    /// This must be used if the response is binary.
    Base64,
}
