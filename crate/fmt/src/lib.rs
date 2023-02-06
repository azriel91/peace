//! Presentation and formatting support for the peace automation framework.
//!
//! See [Output Presentation].
//!
//! [Output Presentation]: https://peace.mk/book/technical_concepts/output/presentation.html

// Re-exports
pub use async_trait::async_trait;

pub use crate::{presentable::Presentable, presenter::Presenter};

mod presentable;
mod presenter;

/// Ergonomically present multiple [`Presentable`]s.
///
/// # Examples
///
/// ```rust,ignore
/// use peace_fmt::{present, Presentable};
///
/// present!(output, "a str", item, "\n");
/// ```
#[macro_export]
macro_rules! present {
    ($output:ident, [$($p:expr),+]) => {
        $($output.present($p).await?;)+
    };
}

/// Ergonomically present multiple [`Presentable`]s.
///
/// # Examples
///
/// ```rust,ignore
/// use peace_fmt::{present, Presentable};
///
/// presentln!(output, "a str", item, "\n");
/// ```
#[macro_export]
macro_rules! presentln {
    ($output:ident, [$($p:expr),+]) => {
        $($output.present($p).await?;)+
        $output.present("\n").await?;
    };
}
