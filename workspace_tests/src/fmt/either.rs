use peace::{
    fmt::{
        presentable::{Bold, CodeInline},
        Either, Presentable, PresentableExt,
    },
    rt_model::output::{CliColorizeOpt, CliMdPresenter},
};

use crate::fmt::cli_output;

#[tokio::test]
async fn either_left_present() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut cli_output = cli_output(&mut buffer, CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    let presentable: Either<Bold<_>, CodeInline> = Bold::new("abc").left_presentable();
    presentable.present(&mut presenter).await?;

    let output = String::from_utf8(buffer)?;

    assert!(matches!(&presentable, Either::Left(_)));
    assert_eq!("**abc**", output);

    Ok(())
}

#[tokio::test]
async fn either_right_present() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut cli_output = cli_output(&mut buffer, CliColorizeOpt::Never);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    let presentable: Either<CodeInline, Bold<_>> = Bold::new("abc").right_presentable();
    presentable.present(&mut presenter).await?;

    let output = String::from_utf8(buffer)?;

    assert!(matches!(&presentable, Either::Right(_)));
    assert_eq!("**abc**", output);

    Ok(())
}

#[test]
fn clone() {
    let either: Either<Bold<_>, CodeInline> = Bold::new("abc").left_presentable();

    let clone = Clone::clone(&either);

    assert_eq!(either, clone);
}

#[test]
fn debug() {
    let either: Either<Bold<_>, CodeInline> = Bold::new("abc").left_presentable();

    assert_eq!("Left(Bold(\"abc\"))", format!("{either:?}"));
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    let either: Either<Bold<_>, CodeInline> = Bold::new("abc").left_presentable();

    assert_eq!("!Left abc\n", serde_yaml::to_string(&either)?);

    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    let either = CodeInline::new("abc".into()).right_presentable();

    assert_eq!(
        either,
        serde_yaml::from_str::<Either<Bold<&'static str>, CodeInline>>("!Right abc")?
    );

    Ok(())
}
