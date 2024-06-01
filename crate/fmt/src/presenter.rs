use crate::{presentable::HeadingLevel, Presentable};

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
///
/// ## List Presentation
///
/// For output to be formatted aesthetically, certain formatters require the
/// presented width of a list's entries to be known.
///
/// However, the width of a `Presentable` entry's presentation can only be known
/// when it is `present`ed, which requires rendering to an in-memory buffer,
/// tracking the character count, then copying from that buffer to the actual
/// output.
///
/// Currently Peace does not support this staged approach, and instead streams
/// each entry to the presenter as it is added. The benefit of this approach is
/// the presentation can be rendered without needing intermediate data to be
/// held in memory for each entry.
#[async_trait::async_trait(?Send)]
pub trait Presenter<'output> {
    /// Presents the given presentable as a heading.
    ///
    /// # Purposes
    ///
    /// * Headings.
    async fn heading<P>(&mut self, level: HeadingLevel, presentable: &P) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized;

    /// Error returned during a presentation failure.
    type Error: std::error::Error;

    /// Presents text as a step id.
    ///
    /// # Purposes
    ///
    /// * An ID with no spaces, e.g. "my_step"
    async fn id(&mut self, id: &str) -> Result<(), Self::Error>;

    /// Presents text as a step name.
    ///
    /// # Purposes
    ///
    /// * A display name with spaces, e.g. "My Step"
    async fn name(&mut self, name: &str) -> Result<(), Self::Error>;

    /// Presents text as plain text.
    async fn text(&mut self, text: &str) -> Result<(), Self::Error>;

    /// Presents the given presentable bolded.
    ///
    /// # Purposes
    ///
    /// * Emphasis.
    async fn bold<P>(&mut self, presentable: &P) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized;

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

    /// Presents a numbered list.
    ///
    /// # Purposes
    ///
    /// * A list of items.
    async fn list_numbered<'f, P, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized + 'f,
        I: IntoIterator<Item = &'f P>;

    /// Presents a numbered list, computing the `Presentable` with the provided
    /// function.
    ///
    /// # Purposes
    ///
    /// * A list of items.
    async fn list_numbered_with<'f, P, I, T, F>(
        &mut self,
        iter: I,
        f: F,
    ) -> Result<(), Self::Error>
    where
        P: Presentable,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> P;

    /// Presents an aligned numbered list.
    ///
    /// Each item in the list has two parts: the "name", and the "description".
    ///
    /// The list will be rendered with all items' descriptions aligned to the
    /// item with the longest name. i.e.
    ///
    /// ```md
    /// 1. Short name:     Description 1.
    /// 2. Longer name:    Description 2.
    /// 3. Very long name: Another description.
    /// ```
    ///
    /// # Purposes
    ///
    /// * A list of items.
    async fn list_numbered_aligned<'f, P0, P1, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = &'f (P0, P1)>;

    /// Presents an aligned numbered list, computing the `Presentable` with the
    /// provided function.
    ///
    /// Each item in the list has two parts: the "name", and the "description".
    ///
    /// The list will be rendered with all items' descriptions aligned to the
    /// item with the longest name. i.e.
    ///
    /// ```md
    /// 1. Short name:     Description 1.
    /// 2. Longer name:    Description 2.
    /// 3. Very long name: Another description.
    /// ```
    ///
    /// # Purposes
    ///
    /// * A list of items.
    async fn list_numbered_aligned_with<'f, P0, P1, I, T, F>(
        &mut self,
        iter: I,
        f: F,
    ) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> &'f (P0, P1);

    /// Presents a bulleted list.
    ///
    /// # Purposes
    ///
    /// * A list of items.
    async fn list_bulleted<'f, P, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized + 'f,
        I: IntoIterator<Item = &'f P>;

    /// Presents a bulleted list, computing the `Presentable` with the provided
    /// function.
    ///
    /// # Purposes
    ///
    /// * A list of items.
    async fn list_bulleted_with<'f, P, I, T, F>(
        &mut self,
        iter: I,
        f: F,
    ) -> Result<(), Self::Error>
    where
        P: Presentable,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> P;

    /// Presents an aligned bulleted list.
    ///
    /// Each item in the list has two parts: the "name", and the "description".
    ///
    /// The list will be rendered with all items' descriptions aligned to the
    /// item with the longest name. i.e.
    ///
    /// ```md
    /// * Short name:     Description 1.
    /// * Longer name:    Description 2.
    /// * Very long name: Another description.
    /// ```
    ///
    /// # Purposes
    ///
    /// * A list of items.
    async fn list_bulleted_aligned<'f, P0, P1, I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = &'f (P0, P1)>;

    /// Presents an aligned bulleted list, computing the `Presentable` with the
    /// provided function.
    ///
    /// Each item in the list has two parts: the "name", and the "description".
    ///
    /// The list will be rendered with all items' descriptions aligned to the
    /// item with the longest name. i.e.
    ///
    /// ```md
    /// * Short name:     Description 1.
    /// * Longer name:    Description 2.
    /// * Very long name: Another description.
    /// ```
    ///
    /// # Purposes
    ///
    /// * A list of items.
    async fn list_bulleted_aligned_with<'f, P0, P1, I, T, F>(
        &mut self,
        iter: I,
        f: F,
    ) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> &'f (P0, P1);
}
