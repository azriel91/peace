/// How to format command output -- human readable or machine parsable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human readable text.
    Text,
    /// The [YAML] format.
    ///
    /// [YAML]: https://yaml.org/
    Yaml,
}
