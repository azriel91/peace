#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use peace::data::{Data, DataAccess, DataAccessDyn, DataInit, Resources, TypeIds, R, W};

    #[test]
    fn data_named_fields_init() {
        let mut resources = Resources::new();

        DataNamedFields::init(&mut resources);

        assert_eq!(A(1), *resources.borrow::<A>());
        assert_eq!(B(2), *resources.borrow::<B>());
    }

    #[test]
    fn data_unnamed_fields_init() {
        let mut resources = Resources::new();

        DataUnnamedFields::init(&mut resources);

        assert_eq!(A(1), *resources.borrow::<A>());
        assert_eq!(B(2), *resources.borrow::<B>());
    }

    #[test]
    fn data_mut_fields_init() {
        let mut resources = Resources::new();

        DataMutFields::init(&mut resources);

        assert_eq!(A(1), *resources.borrow::<A>());
        assert_eq!(B(2), *resources.borrow::<B>());
    }

    #[test]
    fn r_init() {
        let mut resources = Resources::new();

        R::<'_, A>::init(&mut resources);

        assert_eq!(A(1), *resources.borrow::<A>());
    }

    #[test]
    fn w_init() {
        let mut resources = Resources::new();

        W::<'_, A>::init(&mut resources);

        assert_eq!(A(1), *resources.borrow::<A>());
    }

    #[test]
    fn data_named_fields_borrows() {
        let mut type_ids_expected = TypeIds::new();
        type_ids_expected.push(TypeId::of::<A>());
        type_ids_expected.push(TypeId::of::<B>());

        let type_ids_actual = <DataNamedFields<'_> as DataAccess>::borrows();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn data_unnamed_fields_borrows() {
        let mut type_ids_expected = TypeIds::new();
        type_ids_expected.push(TypeId::of::<A>());
        type_ids_expected.push(TypeId::of::<B>());

        let type_ids_actual = <DataUnnamedFields<'_> as DataAccess>::borrows();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn data_mix_fields_borrows() {
        let mut type_ids_expected = TypeIds::new();
        type_ids_expected.push(TypeId::of::<A>());

        let type_ids_actual = <DataMixFields<'_> as DataAccess>::borrows();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn r_borrows() {
        let mut type_ids_expected = TypeIds::new();
        type_ids_expected.push(TypeId::of::<A>());

        let type_ids_actual = <R<'_, A> as DataAccess>::borrows();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn w_borrows() {
        let type_ids_expected = TypeIds::new();

        let type_ids_actual = <W<'_, A> as DataAccess>::borrows();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn data_named_fields_borrow_muts() {
        let type_ids_expected = TypeIds::new();

        let type_ids_actual = <DataNamedFields<'_> as DataAccess>::borrow_muts();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn data_unnamed_fields_borrow_muts() {
        let type_ids_expected = TypeIds::new();

        let type_ids_actual = <DataUnnamedFields<'_> as DataAccess>::borrow_muts();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn data_mix_fields_borrow_muts() {
        let mut type_ids_expected = TypeIds::new();
        type_ids_expected.push(TypeId::of::<B>());

        let type_ids_actual = <DataMixFields<'_> as DataAccess>::borrow_muts();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn r_borrow_muts() {
        let type_ids_expected = TypeIds::new();

        let type_ids_actual = <R<'_, A> as DataAccess>::borrow_muts();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[test]
    fn w_borrow_muts() {
        let mut type_ids_expected = TypeIds::new();
        type_ids_expected.push(TypeId::of::<A>());

        let type_ids_actual = <W<'_, A> as DataAccess>::borrow_muts();

        assert_eq!(type_ids_expected, type_ids_actual);
    }

    #[derive(Debug, Data)]
    struct DataNamedFields<'op> {
        a_imm: R<'op, A>,
        b_imm: R<'op, B>,
    }

    #[derive(Debug, Data)]
    struct DataUnnamedFields<'op>(R<'op, A>, R<'op, B>);

    #[derive(Debug, Data)]
    struct DataMutFields<'op> {
        a_mut: W<'op, A>,
        b_mut: W<'op, B>,
    }

    #[derive(Debug, Data)]
    struct DataMixFields<'op> {
        a_imm: R<'op, A>,
        b_mut: W<'op, B>,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct A(u32);

    impl DataInit for A {
        fn init(resources: &mut Resources) {
            resources.insert(A(1));
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct B(u32);

    impl DataInit for B {
        fn init(resources: &mut Resources) {
            resources.insert(B(2));
        }
    }
}
