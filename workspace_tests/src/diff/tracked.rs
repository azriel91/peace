use peace::diff::{Equality, MaybeEq, Tracked};

// for coverage
#[test]
fn clone() {
    assert_eq!(
        Tracked::<Value>::Known(Value(0)),
        Tracked::<Value>::Known(Value(0)).clone()
    )
}

#[test]
fn debug() {
    let tracked = Tracked::<Value>::Known(Value(0));
    assert_eq!("Known(Value(0))", format!("{tracked:?}"))
}

mod maybe_eq {
    use peace::diff::{Equality, MaybeEq, Tracked};

    use super::Value;

    #[test]
    fn tracked_unknown_returns_equality_unknown() {
        assert_eq!(
            Equality::Unknown,
            Tracked::<Value>::Unknown.maybe_eq(&Tracked::<Value>::None)
        );
        assert_eq!(
            Equality::Unknown,
            Tracked::<Value>::Unknown.maybe_eq(&Tracked::Unknown)
        );
        assert_eq!(
            Equality::Unknown,
            Tracked::Unknown.maybe_eq(&Tracked::Known(Value(1)))
        );
    }

    #[test]
    fn tracked_nones_are_equal() {
        assert_eq!(
            Equality::Equal,
            Tracked::<Value>::None.maybe_eq(&Tracked::<Value>::None)
        );
    }

    #[test]
    fn tracked_knowns_delegate_to_known_value() {
        assert_eq!(
            Equality::Equal,
            Tracked::Known(Value(1)).maybe_eq(&Tracked::Known(Value(1)))
        );
        assert_eq!(
            Equality::NotEqual,
            Tracked::Known(Value(1)).maybe_eq(&Tracked::Known(Value(2)))
        );
    }

    #[test]
    fn tracked_known_does_not_equal_tracked_none() {
        assert_eq!(
            Equality::NotEqual,
            Tracked::Known(Value(1)).maybe_eq(&Tracked::None)
        );
        assert_eq!(
            Equality::NotEqual,
            Tracked::None.maybe_eq(&Tracked::Known(Value(2)))
        );
    }
}

mod partial_eq {
    use peace::diff::Tracked;

    use super::Value;

    #[test]
    fn tracked_unknown_returns_equality_unknown() {
        assert_eq!(false, Tracked::<Value>::Unknown == Tracked::<Value>::None);
        assert_eq!(false, Tracked::<Value>::Unknown == Tracked::Unknown);
        assert_eq!(false, Tracked::Unknown == Tracked::Known(Value(1)));
    }

    #[test]
    fn tracked_nones_are_equal() {
        assert_eq!(true, Tracked::<Value>::None == Tracked::<Value>::None);
    }

    #[test]
    fn tracked_knowns_delegate_to_known_value() {
        assert_eq!(true, Tracked::Known(Value(1)) == Tracked::Known(Value(1)));
        assert_eq!(false, Tracked::Known(Value(1)) == Tracked::Known(Value(2)));
    }

    #[test]
    fn tracked_known_does_not_equal_tracked_none() {
        assert_eq!(false, Tracked::Known(Value(1)) == Tracked::None);
        assert_eq!(false, Tracked::None == Tracked::Known(Value(2)));
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value(u8);

impl MaybeEq for Value {
    fn maybe_eq(&self, other: &Self) -> Equality {
        Equality::from(self == other)
    }
}
