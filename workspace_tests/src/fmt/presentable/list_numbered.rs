use peace::{
    fmt::{presentable::ListNumbered, Presentable},
    rt_model::output::{CliColorizeOpt, CliMdPresenter},
};

use crate::fmt::presentable::cli_output;

#[tokio::test]
async fn present() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut cli_output = cli_output(&mut buffer, CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    let list_numbered =
        ListNumbered::new((1..=11).map(|n| format!("Item {n}")).collect::<Vec<_>>());
    list_numbered.present(&mut presenter).await?;

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        " \u{1b}[38;5;15m1.\u{1b}[0m Item 1
 \u{1b}[38;5;15m2.\u{1b}[0m Item 2
 \u{1b}[38;5;15m3.\u{1b}[0m Item 3
 \u{1b}[38;5;15m4.\u{1b}[0m Item 4
 \u{1b}[38;5;15m5.\u{1b}[0m Item 5
 \u{1b}[38;5;15m6.\u{1b}[0m Item 6
 \u{1b}[38;5;15m7.\u{1b}[0m Item 7
 \u{1b}[38;5;15m8.\u{1b}[0m Item 8
 \u{1b}[38;5;15m9.\u{1b}[0m Item 9
\u{1b}[38;5;15m10.\u{1b}[0m Item 10
\u{1b}[38;5;15m11.\u{1b}[0m Item 11
",
        output
    );
    assert_eq!(
        r#" 1. Item 1
 2. Item 2
 3. Item 3
 4. Item 4
 5. Item 5
 6. Item 6
 7. Item 7
 8. Item 8
 9. Item 9
10. Item 10
11. Item 11
"#,
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[test]
fn debug() {
    assert_eq!(
        r#"ListNumbered(["abc"])"#,
        format!("{:?}", ListNumbered::new(vec!["abc"]))
    )
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        "- abc\n",
        serde_yaml::to_string(&ListNumbered::new(vec!["abc"]))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        ListNumbered::new(vec![(String::from("abc"))]),
        serde_yaml::from_str("- abc")?,
    );
    Ok(())
}

#[test]
fn from() {
    assert_eq!(
        ListNumbered::new(vec!["abc"]),
        ListNumbered::from(vec!["abc"])
    );
}
