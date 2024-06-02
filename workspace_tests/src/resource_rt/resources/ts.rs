mod debug {
    use peace::resource_rt::resources::ts::{Empty, SetUp};

    #[test]
    fn empty() {
        assert_eq!("Empty", format!("{Empty:?}"));
    }
    #[test]
    fn set_up() {
        assert_eq!("SetUp", format!("{SetUp:?}"));
    }
}
