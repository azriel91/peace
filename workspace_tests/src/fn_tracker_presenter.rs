use peace::fmt::{async_trait, Presentable, Presenter};

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
        P: Presentable + 'f,
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

    async fn list_bulleted<'f, P, I>(&mut self, _iter: I) -> Result<(), Self::Error>
    where
        P: Presentable + 'f,
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
}
