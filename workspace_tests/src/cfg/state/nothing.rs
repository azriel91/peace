use peace::cfg::state::Nothing;

#[test]
fn display() {
    assert_eq!("", format!("{}", Nothing));
}

#[test]
fn clone() {
    assert_eq!(Nothing, (&Nothing).clone());
}

#[test]
fn debug() {
    assert_eq!("Nothing", format!("{:?}", Nothing));
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!("null\n", serde_yaml::to_string(&Nothing)?);
    Ok(())
}

#[test]
fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let external = serde_yaml::from_str::<Nothing>("null\n")?;
    assert_eq!(Nothing, external);

    Ok(())
}
