use peace::{
    fmt::{presentable::CodeInline, Presentable},
    rt_model::output::{CliColorizeOpt, CliMdPresenter},
};

use crate::fmt::presentable::cli_output;

#[tokio::test]
async fn present() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut cli_output = cli_output(&mut buffer, CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    CodeInline::new("code_inline".into())
        .present(&mut presenter)
        .await?;

    let output = String::from_utf8(buffer)?;
    assert_eq!("\u{1b}[38;5;75m`code_inline`\u{1b}[0m", output);
    assert_eq!("`code_inline`", console::strip_ansi_codes(&output));
    Ok(())
}

#[test]
fn debug() {
    assert_eq!(
        "CodeInline(\"abc\")",
        format!("{:?}", CodeInline::new("abc".into()))
    )
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        "abc\n\
        ",
        serde_yaml::to_string(&CodeInline::new("abc".into()))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(CodeInline::new("abc".into()), serde_yaml::from_str("abc")?,);
    Ok(())
}
