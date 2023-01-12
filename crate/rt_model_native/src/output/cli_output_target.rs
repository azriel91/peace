#[cfg(feature = "output_in_memory")]
use peace_rt_model_core::indicatif::InMemoryTerm;

/// Where to output progress updates to -- stdout or stderr.
///
/// This defaults to `stderr`.
///
/// # Implementation Note
///
/// `PartialEq` and `Eq` are implemented manually. For `InMemory`, equality is
/// defined by the [`contents`] of the in-memory terminal.
///
/// [`contents`]: https://docs.rs/indicatif/latest/indicatif/struct.InMemoryTerm.html#method.contents
#[derive(Clone, Debug, Default)]
pub enum CliOutputTarget {
    /// Standard error -- file descriptor 1.
    Stdout,
    /// Standard error -- file descriptor 2.
    #[default]
    Stderr,
    /// Render to an in-memory buffer.
    ///
    /// This variant can be constructed using the `in_memory` function:
    ///
    /// ```rust
    /// # #[cfg(feature = "output_in_memory")]
    /// # fn main() {
    /// use peace_rt_model_native::CliOutputTarget;
    ///
    /// let progress_target = CliOutputTarget::in_memory(50, 120);
    /// # }
    /// #
    /// # #[cfg(not(feature = "output_in_memory"))]
    /// # fn main() {}
    /// ```
    #[cfg(feature = "output_in_memory")]
    InMemory(InMemoryTerm),
}

impl PartialEq for CliOutputTarget {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Stdout, Self::Stdout) | (Self::Stderr, Self::Stderr) => true,
            #[cfg(feature = "output_in_memory")]
            (Self::InMemory(term_self), Self::InMemory(term_other)) => {
                term_self.contents() == term_other.contents()
            }
            _ => false,
        }
    }
}

impl Eq for CliOutputTarget {}

impl CliOutputTarget {
    /// Returns `CliOutputTarget::InMemory` with an empty buffer.
    ///
    /// # Parameters
    ///
    /// * `row_count`: Number of rows (lines) of the in-memory terminal.
    /// * `col_count`: Number of columns (characters) of the in-memory terminal.
    #[cfg(feature = "output_in_memory")]
    pub fn in_memory(row_count: u16, col_count: u16) -> Self {
        Self::InMemory(InMemoryTerm::new(row_count, col_count))
    }
}
