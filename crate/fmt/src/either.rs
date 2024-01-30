use serde::{Deserialize, Serialize};

use crate::{Presentable, Presenter};

/// Combines two different `Presentable`s into a single type.
///
/// This is useful when conditionally choosing between two distinct
/// `Presentable` types:
///
/// ```rust,ignore
/// use peace_fmt::{Either, Presentable};
///
/// let cond = true;
///
/// let presentable = if cond {
///     Either::Left(Bold::new(String::from("a")));
/// } else {
///     Either::Right(CodeInline::new("b".into()));
/// };
///
/// presentln!(output, &presentable);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Either<A, B> {
    /// First branch of the type.
    Left(A),
    /// Second branch of the type.
    Right(B),
}

#[async_trait::async_trait(?Send)]
impl<A, B> Presentable for Either<A, B>
where
    A: Presentable,
    B: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        match self {
            Self::Left(a) => a.present(presenter).await,
            Self::Right(b) => b.present(presenter).await,
        }
    }
}
