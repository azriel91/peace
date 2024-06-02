use std::any::{Any, TypeId};

use peace::resources::{
    resources::ts::{Empty, SetUp},
    states::{StatesCurrent, StatesGoal},
    Resources,
};

mod ts;

#[test]
fn debug() {
    let mut resources = Resources::new();
    resources.insert(1u32);

    assert_eq!(
        r#"Resources { inner: {u32: 1}, marker: PhantomData<peace_resources_rt::resources::ts::Empty> }"#,
        format!("{resources:?}")
    );
}

#[test]
fn defaults_to_resources_empty() {
    let resources_default = Resources::default();

    assert_eq!(
        TypeId::of::<Resources::<Empty>>(),
        resources_default.type_id()
    );
}

#[test]
fn resources_set_up_from_resources_empty() {
    let resources_empty = Resources::new();

    let resources_set_up = Resources::<SetUp>::from(resources_empty);

    // no default resources
    assert!(!resources_set_up.contains::<StatesCurrent>());
    assert!(!resources_set_up.contains::<StatesGoal>());
}
