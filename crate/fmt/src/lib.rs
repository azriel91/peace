//! Presentation and formatting support for the peace automation framework.
//!
//! See [Output Presentation].
//!
//! [Output Presentation]: https://peace.mk/book/technical_concepts/output/presentation.html

pub use crate::{
    error::Error, presentable::Presentable, presentable_list::PresentableList,
    presenter::Presenter, result::Result,
};

mod error;
mod presentable;
mod presentable_list;
mod presenter;
mod result;
