use serde::{Deserialize, Serialize};

/// The type of resource locaction.
///
/// This affects how the [`ResourceLocation`] is rendered.
///
/// [`ResourceLocation`]: crate::ResourceLocation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ResourceLocationType {
    /// Rendered with dashed lines.
    ///
    /// Suitable for concepts like:
    ///
    /// * Cloud provider
    /// * Network / subnet
    Grouping,
    /// Rendered with solid lines.
    ///
    /// Suitable for concepts like:
    ///
    /// * Localhost
    /// * Server
    Host,
    /// Rendered with solid lines.
    ///
    /// Suitable for concepts like:
    ///
    /// * File path
    /// * URL
    Path,
}
