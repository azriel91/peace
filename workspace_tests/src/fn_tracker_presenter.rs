use peace::fmt::{async_trait, presentable::HeadingLevel, Presentable, Presenter};

use crate::{fn_name::fn_name_short, FnInvocation};

/// Tracks `Presenter` function calls.
///
/// Formats `Presentable` data as markdown on the CLI.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct FnTrackerPresenter {
    /// List of function invocations.
    fn_invocations: Vec<FnInvocation>,
}

impl FnTrackerPresenter {
    /// Returns a new `FnTrackerPresenter`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the recorded function invocations.
    pub fn fn_invocations(&self) -> &[FnInvocation] {
        self.fn_invocations.as_ref()
    }
}

#[async_trait(?Send)]
impl Presenter<'static> for FnTrackerPresenter {
    type Error = std::io::Error;

    async fn heading<P>(
        &mut self,
        heading_level: HeadingLevel,
        _presentable: &P,
    ) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized,
    {
        self.fn_invocations.push(FnInvocation::new(
            fn_name_short!(),
            vec![Some(format!("{heading_level:?}")), None],
        ));
        Ok(())
    }

    async fn id(&mut self, id: &str) -> Result<(), Self::Error> {
        self.fn_invocations.push(FnInvocation::new(
            fn_name_short!(),
            vec![Some(format!("{id:?}"))],
        ));
        Ok(())
    }

    async fn name(&mut self, name: &str) -> Result<(), Self::Error> {
        self.fn_invocations.push(FnInvocation::new(
            fn_name_short!(),
            vec![Some(format!("{name:?}"))],
        ));
        Ok(())
    }

    async fn text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.fn_invocations.push(FnInvocation::new(
            fn_name_short!(),
            vec![Some(format!("{text:?}"))],
        ));
        Ok(())
    }

    async fn bold<P>(&mut self, _presentable: &P) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized,
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None]));
        Ok(())
    }

    async fn tag(&mut self, tag: &str) -> Result<(), Self::Error> {
        self.fn_invocations.push(FnInvocation::new(
            fn_name_short!(),
            vec![Some(format!("{tag:?}"))],
        ));

        Ok(())
    }

    async fn code_inline(&mut self, code: &str) -> Result<(), Self::Error> {
        self.fn_invocations.push(FnInvocation::new(
            fn_name_short!(),
            vec![Some(format!("{code:?}"))],
        ));
        Ok(())
    }

    async fn list_numbered<'f, P, I>(&mut self, _iter: I) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized + 'f,
        I: IntoIterator<Item = &'f P>,
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None]));

        Ok(())
    }

    async fn list_numbered_with<'f, P, I, T, F>(
        &mut self,
        _iter: I,
        _f: F,
    ) -> Result<(), Self::Error>
    where
        P: Presentable,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> P,
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None, None]));

        Ok(())
    }

    async fn list_numbered_aligned<'f, P0, P1, I>(&mut self, _iter: I) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = &'f (P0, P1)>,
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None]));

        Ok(())
    }

    async fn list_numbered_aligned_with<'f, P0, P1, I, T, F>(
        &mut self,
        _iter: I,
        _f: F,
    ) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> &'f (P0, P1),
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None, None]));

        Ok(())
    }

    async fn list_bulleted<'f, P, I>(&mut self, _iter: I) -> Result<(), Self::Error>
    where
        P: Presentable + ?Sized + 'f,
        I: IntoIterator<Item = &'f P>,
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None]));

        Ok(())
    }

    async fn list_bulleted_with<'f, P, I, T, F>(
        &mut self,
        _iter: I,
        _f: F,
    ) -> Result<(), Self::Error>
    where
        P: Presentable,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> P,
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None, None]));

        Ok(())
    }

    async fn list_bulleted_aligned<'f, P0, P1, I>(&mut self, _iter: I) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = &'f (P0, P1)>,
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None]));

        Ok(())
    }

    async fn list_bulleted_aligned_with<'f, P0, P1, I, T, F>(
        &mut self,
        _iter: I,
        _f: F,
    ) -> Result<(), Self::Error>
    where
        P0: Presentable + 'f,
        P1: Presentable + 'f,
        I: IntoIterator<Item = T>,
        T: 'f,
        F: Fn(T) -> &'f (P0, P1),
    {
        self.fn_invocations
            .push(FnInvocation::new(fn_name_short!(), vec![None, None]));

        Ok(())
    }
}

#[tokio::test]
async fn coverage() -> Result<(), std::io::Error> {
    let mut fn_tracker_presenter = FnTrackerPresenter::new();
    fn_tracker_presenter
        .heading(HeadingLevel::Level1, "")
        .await?;
    fn_tracker_presenter.id("the_id").await?;
    fn_tracker_presenter.name("the_name").await?;
    fn_tracker_presenter.text("the_text").await?;
    fn_tracker_presenter.bold("").await?;
    fn_tracker_presenter.tag("the_tag").await?;
    fn_tracker_presenter.code_inline("the_code_inline").await?;
    fn_tracker_presenter
        .list_numbered(std::iter::once("abc"))
        .await?;
    fn_tracker_presenter
        .list_numbered_with(std::iter::once("abc"), std::convert::identity)
        .await?;
    fn_tracker_presenter
        .list_numbered_aligned(std::iter::once(&("abc", "def")))
        .await?;
    fn_tracker_presenter
        .list_numbered_aligned_with(std::iter::once(&("abc", "def")), std::convert::identity)
        .await?;
    fn_tracker_presenter
        .list_bulleted(std::iter::once("abc"))
        .await?;
    fn_tracker_presenter
        .list_bulleted_with(std::iter::once("abc"), std::convert::identity)
        .await?;
    fn_tracker_presenter
        .list_bulleted_aligned(std::iter::once(&("abc", "def")))
        .await?;
    fn_tracker_presenter
        .list_bulleted_aligned_with(std::iter::once(&("abc", "def")), std::convert::identity)
        .await?;

    [
        FnInvocation::new("heading", vec![Some(String::from("Level1")), None]),
        FnInvocation::new("id", vec![Some(String::from("\"the_id\""))]),
        FnInvocation::new("name", vec![Some(String::from("\"the_name\""))]),
        FnInvocation::new("text", vec![Some(String::from("\"the_text\""))]),
        FnInvocation::new("bold", vec![None]),
        FnInvocation::new("tag", vec![Some(String::from("\"the_tag\""))]),
        FnInvocation::new(
            "code_inline",
            vec![Some(String::from("\"the_code_inline\""))],
        ),
        FnInvocation::new("list_numbered", vec![None]),
        FnInvocation::new("list_numbered_with", vec![None, None]),
        FnInvocation::new("list_numbered_aligned", vec![None]),
        FnInvocation::new("list_numbered_aligned_with", vec![None, None]),
        FnInvocation::new("list_bulleted", vec![None]),
        FnInvocation::new("list_bulleted_with", vec![None, None]),
        FnInvocation::new("list_bulleted_aligned", vec![None]),
        FnInvocation::new("list_bulleted_aligned_with", vec![None, None]),
    ]
    .into_iter()
    .zip(fn_tracker_presenter.fn_invocations().iter())
    .for_each(|(expected, actual)| assert_eq!(&expected, actual));

    Ok(())
}
