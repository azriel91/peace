use crate::{Presentable, Presenter};

#[async_trait::async_trait(?Send)]
impl<T0> Presentable for (T0,)
where
    T0: Presentable,
{
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        self.0.present(presenter).await?;
        Ok(())
    }
}

macro_rules! tuple_presentable_impl {
    (($($Tn:ident),+), [$($n:tt),+]) => {
        #[async_trait::async_trait(?Send)]
        impl<$($Tn),+> Presentable for ($($Tn),+)
        where
            $($Tn: Presentable),+
        {
            async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
            where
                PR: Presenter<'output>,
            {
                $(self.$n.present(presenter).await?;)+
                Ok(())
            }
        }
    };
}

tuple_presentable_impl!((T0, T1), [0, 1]);
tuple_presentable_impl!((T0, T1, T2), [0, 1, 2]);
tuple_presentable_impl!((T0, T1, T2, T3), [0, 1, 2, 3]);
tuple_presentable_impl!((T0, T1, T2, T3, T4), [0, 1, 2, 3, 4]);
tuple_presentable_impl!((T0, T1, T2, T3, T4, T5), [0, 1, 2, 3, 4, 5]);
tuple_presentable_impl!((T0, T1, T2, T3, T4, T5, T6), [0, 1, 2, 3, 4, 5, 6]);
tuple_presentable_impl!((T0, T1, T2, T3, T4, T5, T6, T7), [0, 1, 2, 3, 4, 5, 6, 7]);
tuple_presentable_impl!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8),
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
);
tuple_presentable_impl!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9),
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
);
tuple_presentable_impl!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
);
tuple_presentable_impl!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
);
tuple_presentable_impl!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
);
tuple_presentable_impl!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13),
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
);
tuple_presentable_impl!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14),
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
);
tuple_presentable_impl!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15),
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);
