use peace::fmt::presentable::CodeInline;

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
