use serde::{Deserialize, Serialize};
use url::Url;

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
/// 1. `ResourceLocation::Group("cloud")`
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
///
/// # Design
///
/// When designing this, another design that was considered is using an enum
/// like the following:
///
/// ```rust,ignore
/// #[derive(Debug)]
/// enum ResourceLocation {
///     Host(ResourceLocationHost),
///     Url(Url),
/// }
///
/// struct ResourceLocationHost {
///     host: Host<String>,
///     port: Option<u16>,
/// }
///
/// impl ResourceLocation {
///     fn from_url(url: &Url) -> Self {
///         Self::Url(url.clone())
///     }
/// }
///
/// impl From<&Url> for ResourceLocationHost {
///     type Error = ();
///
///     fn from(url: &Url) -> Result<Self, ()> {
///         url.host()
///             .map(|host| {
///                 let host = host.to_owned();
///                 let port = url.map(Url::port_or_known_default);
///                 Self { host, port }
///             })
///             .ok_or(())
///     }
/// }
/// ```
///
/// However, the purpose of `ResourceLocation` is primarily for rendering, and
/// providing accurate variants for each kind of resource location causes
/// additional burden on:
///
/// * framework maintainers to maintain those variants
/// * item implementors to select the correct variant for accuracy
/// * item implementors to select a variant consistent with other item
///   implementors
///
/// A less accurate model with a limited number of [`ResourceLocationType`]s
/// balances the modelling accuracy, rendering, and maintenance burden.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ResourceLocation {
    /// The name of the resource location.
    pub name: String,
    /// The type of the resource location.
    pub r#type: ResourceLocationType,
}

impl ResourceLocation {
    /// The string used for an unknown host.
    pub const HOST_UNKNOWN: &'static str = "unknown";
    /// The string used for localhost.
    pub const LOCALHOST: &'static str = "localhost";

    /// Returns a new `ResourceLocation`.
    ///
    /// See also:
    ///
    /// * [`ResourceLocation::group`]
    /// * [`ResourceLocation::host`]
    /// * [`ResourceLocation::localhost`]
    /// * [`ResourceLocation::path`]
    pub fn new(name: String, r#type: ResourceLocationType) -> Self {
        Self { name, r#type }
    }

    /// Returns `ResourceLocation::new(name, ResourceLocationType::Group)`.
    pub fn group(name: String) -> Self {
        Self {
            name,
            r#type: ResourceLocationType::Group,
        }
    }

    /// Returns `ResourceLocation::new(name, ResourceLocationType::Host)`.
    pub fn host(name: String) -> Self {
        Self {
            name,
            r#type: ResourceLocationType::Host,
        }
    }

    /// Returns `ResourceLocation::new("unknown".to_string(),
    /// ResourceLocationType::Host)`.
    pub fn host_unknown() -> Self {
        Self {
            name: Self::HOST_UNKNOWN.to_string(),
            r#type: ResourceLocationType::Host,
        }
    }

    /// Returns `ResourceLocation::new(name, ResourceLocationType::Host)`.
    ///
    /// This is "lossy" in the sense that if the URL doesn't have a [`Host`],
    /// this will return localhost, as URLs without a host may be unix sockets,
    /// or data URLs.
    ///
    /// [`Host`]: url::Host
    pub fn host_from_url(url: &Url) -> Self {
        url.host_str()
            .map(|host_str| Self {
                name: host_str.to_string(),
                r#type: ResourceLocationType::Host,
            })
            .unwrap_or_else(Self::localhost)
    }

    /// Returns `ResourceLocation::host("localhost".to_string())`.
    pub fn localhost() -> Self {
        Self {
            name: Self::LOCALHOST.to_string(),
            r#type: ResourceLocationType::Host,
        }
    }

    /// Returns `ResourceLocation::new(name, ResourceLocationType::Path)`.
    pub fn path(name: String) -> Self {
        Self {
            name,
            r#type: ResourceLocationType::Path,
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
