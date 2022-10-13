/// How to format command output -- human readable or machine parsable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human readable text.
    Text,
    /// The YAML Ain't Markup Language™ ([YAML]) format.
    ///
    /// [YAML]: https://yaml.org/
    Yaml,
    /// The JavaScript Object Notation ([JSON]) format
    ///
    /// [JSON]: https://www.json.org/
    #[cfg(feature = "output_json")]
    Json,
}
