/// Takes a `Presentable` type and presents it to the user.
///
/// # Design
///
/// ```text
///              .--- Presentable::present(&self, presenter);
///              :
/// .--------.   :    .-----------.      .---------------.
/// | Caller |   :    |  Peace's  |      | Implementor's |
/// |        | -----> | Presenter | ---> | Presenter     |
/// '--------'        '-----------'      '---------------'
///                        :
///                        :
///            // Peace wraps the implementor's `Presenter`
///            // in a tracking `Presenter`, which allows:
///            //
///            // * Implementors to perceive using a `Presenter`
///            //   in `Presentable` implementations.
///            // * Peace to gatekeep how much detail is passed
///            //   through, by tracking depth of information.
/// ```
#[async_trait::async_trait(?Send)]
pub trait Presenter<'output> {
    /// Presents a `&str` as an item name.
    async fn name(&mut self, name: &str) -> crate::Result;

    /// Presents a `&str` as plain text.
    async fn text(&mut self, text: &str) -> crate::Result;

    /// Presents a `&str` as inline code.
    async fn code_inline(&mut self, text: &str) -> crate::Result;

    /// Presents a list.
    async fn list(&mut self) -> crate::PresentableList;
}
