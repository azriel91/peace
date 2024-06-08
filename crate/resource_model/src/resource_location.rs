use serde::{Deserialize, Serialize};

use crate::ResourceLocationType;

/// One layer of where a resource is located.
///
/// These will be merged into the same node based on their variant and name.
///
/// For example, if two different items provide the following
/// `ResourceLocation`s:
///
/// Item 1:
///
/// 1. `ResourceLocation::Grouping("cloud")`
/// 2. `ResourceLocation::Host("app.domain.com")`
/// 3. `ResourceLocation::Path("/path/to/a_file")`
///
/// Item 2:
///
/// 1. `ResourceLocation::Host("app.domain.com")`
/// 2. `ResourceLocation::Path("/path/to/another_file")`
///
/// Then the resultant node hierarchy will be:
///
/// ```yaml
/// cloud:
///   app.domain.com:
///     "/path/to/a_file": {}
///     "/path/to/another_file": {}
/// ```
///
/// # Implementors
///
/// Item implementors should endeavour to use the same name for each
/// `ResourceLocation`, as that is how the Peace framework determines if two
/// `ResourceLocation`s are the same.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ResourceLocation {
    /// The name of the resource location.
    pub name: String,
    /// The type of the resource location.
    pub r#type: ResourceLocationType,
}

impl ResourceLocation {
    /// The string used for localhost.
    pub const LOCALHOST: &'static str = "localhost";

    /// Returns a new `ResourceLocation`.
    pub fn new(name: String, r#type: ResourceLocationType) -> Self {
        Self { name, r#type }
    }

    /// Returns `ResourceLocation { name: "localhost".to_string(), r#type:
    /// ResourceLocationType::Host }`.
    pub fn localhost() -> Self {
        Self {
            name: Self::LOCALHOST.to_string(),
            r#type: ResourceLocationType::Host,
        }
    }

    /// Returns the name of the resource location.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the resource location.
    pub fn r#type(&self) -> ResourceLocationType {
        self.r#type
    }
}
