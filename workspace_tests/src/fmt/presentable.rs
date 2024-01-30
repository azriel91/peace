mod bold;
mod code_inline;
mod heading;
mod list_bulleted;
mod list_bulleted_aligned;
mod list_numbered;
mod list_numbered_aligned;

use peace::fmt::Presentable;

use crate::{FnInvocation, FnTrackerPresenter};

#[tokio::test]
async fn ref_t_is_presentable_when_t_is_presentable() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = FnTrackerPresenter::new();

    let s: String = String::from("hello");
    <&String as Presentable>::present(&&s, &mut presenter).await?;

    assert_eq!(
        vec![FnInvocation::new(
            "text",
            vec![Some(r#""hello""#.to_string())]
        ),],
        presenter.fn_invocations()
    );

    Ok(())
}

#[tokio::test]
async fn ref_str_is_presentable() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = FnTrackerPresenter::new();

    let s = "hello";
    <&str as Presentable>::present(&s, &mut presenter).await?;

    assert_eq!(
        vec![FnInvocation::new(
            "text",
            vec![Some(r#""hello""#.to_string())]
        ),],
        presenter.fn_invocations()
    );

    Ok(())
}

#[tokio::test]
async fn string_is_presentable() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = FnTrackerPresenter::new();

    let s: String = String::from("hello");
    <String as Presentable>::present(&s, &mut presenter).await?;

    assert_eq!(
        vec![FnInvocation::new(
            "text",
            vec![Some(r#""hello""#.to_string())]
        ),],
        presenter.fn_invocations()
    );

    Ok(())
}

#[tokio::test]
async fn vec_t_is_presentable_when_t_is_presentable() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = FnTrackerPresenter::new();

    let list = vec![String::from("one"), String::from("two")];
    <Vec<String> as Presentable>::present(&list, &mut presenter).await?;

    assert_eq!(
        vec![FnInvocation::new("list_numbered", vec![None])],
        presenter.fn_invocations()
    );

    Ok(())
}

#[tokio::test]
async fn array_t_is_presentable_when_t_is_presentable() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = FnTrackerPresenter::new();

    let list = [String::from("one"), String::from("two")];
    <[String] as Presentable>::present(&list, &mut presenter).await?;

    assert_eq!(
        vec![FnInvocation::new("list_numbered", vec![None])],
        presenter.fn_invocations()
    );

    Ok(())
}
