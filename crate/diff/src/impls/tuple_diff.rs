use crate::Diff;

macro_rules! diff_tuple {
    (($($ty:ident),*), ($($access:tt),*)) => {
        impl<$($ty),*> Diff for ($($ty),*,)
            where $($ty: Diff),*
        {
            type Repr = ($(<$ty>::Repr),*,);

            fn diff(&self, other: &Self) -> Self::Repr {
                ($(self.$access.diff(&other.$access)),*,)
            }

            fn apply(&mut self, diff: &Self::Repr) {
                $(self.$access.apply(&diff.$access));*;
            }

            fn identity() -> Self {
                ($(<$ty>::identity()),*,)
            }
        }
    }
}

diff_tuple!((A), (0));
diff_tuple!((A, B), (0, 1));
diff_tuple!((A, B, C), (0, 1, 2));
diff_tuple!((A, B, C, D), (0, 1, 2, 3));
diff_tuple!((A, B, C, D, F), (0, 1, 2, 3, 4));
diff_tuple!((A, B, C, D, F, G), (0, 1, 2, 3, 4, 5));
diff_tuple!((A, B, C, D, F, G, H), (0, 1, 2, 3, 4, 5, 6));
diff_tuple!((A, B, C, D, F, G, H, I), (0, 1, 2, 3, 4, 5, 6, 7));
diff_tuple!((A, B, C, D, F, G, H, I, J), (0, 1, 2, 3, 4, 5, 6, 7, 8));
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
);
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K, L),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
);
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K, L, M),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11)
);
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K, L, M, N),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
);
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K, L, M, N, O),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13)
);
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K, L, M, N, O, P),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14)
);
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)
);
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16)
);
diff_tuple!(
    (A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R, S),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17)
);
