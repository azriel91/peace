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
    type Error: std::error::Error;

    /// Presents text as an item id.
    ///
    /// # Purposes
    ///
    /// * An ID with no spaces, e.g. "my_item"
    async fn id(&mut self, id: &str) -> Result<(), Self::Error>;

    /// Presents text as an item name.
    ///
    /// # Purposes
    ///
    /// * A display name with spaces, e.g. "My Item"
    async fn name(&mut self, name: &str) -> Result<(), Self::Error>;

    /// Presents text as plain text.
    async fn text(&mut self, text: &str) -> Result<(), Self::Error>;

    /// Presents text as inline code.
    ///
    /// # Purposes
    ///
    /// * Short bit of code, e.g. "my::module", "MyStruct", "function_name".
    async fn code_inline(&mut self, text: &str) -> Result<(), Self::Error>;

    /// Presents text as a tag.
    ///
    /// # Purposes
    ///
    /// * A profile, e.g. "development", "production".
    /// * A value used to categorize data, e.g. "stale".
    async fn tag(&mut self, tag: &str) -> Result<(), Self::Error>;

    /// Presents a list.
    fn list<'f>(&'f mut self) -> crate::PresentableList<'output, 'f, Self>
    where
        Self: Sized;
}
