use peace::cfg::state::External;

#[test]
fn display() {
    assert_eq!("u8 not yet determined", format!("{}", External::<u8>::Tbd));
    assert_eq!("u8: 123", format!("{}", External::<u8>::Value(123)));
}

#[test]
fn clone() {
    assert_eq!(External::<u8>::Tbd, (&External::<u8>::Tbd).clone());
}

#[test]
fn debug() {
    assert_eq!("Tbd", format!("{:?}", External::<u8>::Tbd));
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!("!Tbd null\n", serde_yaml::to_string(&External::<u8>::Tbd)?);
    assert_eq!(
        "!Value 123\n",
        serde_yaml::to_string(&External::<u8>::Value(123))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let external = serde_yaml::from_str::<External<u8>>("!Tbd null\n")?;
    assert_eq!(External::<u8>::Tbd, external);

    let external = serde_yaml::from_str::<External<u8>>("!Value 123\n")?;
    assert_eq!(External::<u8>::Value(123), external);
    Ok(())
}
