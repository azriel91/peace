/// Information about a function invocation.
#[derive(Debug, PartialEq, Eq)]
pub struct FnInvocation {
    /// Simple name of the function invoked.
    name: &'static str,
    /// Debug values of arguments to the function.
    ///
    /// A `None` means that that arguments to the function is not tracked,
    /// possibly because it does not implement `Debug`.
    args: Vec<Option<String>>,
}

impl FnInvocation {
    /// Returns a new `FnInvocation`.
    pub fn new(name: &'static str, args: Vec<Option<String>>) -> Self {
        Self { name, args }
    }

    // Currently we only use the `PartialEq` implementation to read the values.
    // /// Returns the simple name of the invoked function.
    // pub fn name(&self) -> &str {
    //     self.name
    // }

    // /// Returns the argument debug strings.
    // pub fn args(&self) -> &[Option<String>] {
    //     self.args.as_ref()
    // }
}
