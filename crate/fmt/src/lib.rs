//! Presentation and formatting support for the peace automation framework.
//!
//! See [Output Presentation].
//!
//! [Output Presentation]: https://peace.mk/book/technical_concepts/output/presentation.html

// Re-exports
pub use async_trait::async_trait;

pub use crate::{
    presentable::Presentable, presentable_list::PresentableList, presenter::Presenter,
};

mod presentable;
mod presentable_list;
mod presenter;
