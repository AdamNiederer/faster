
macro_rules! impl_packed_sum {
    ($($vec:tt),*) => {
        $(
            impl Sum for $vec {
                #[inline(always)]
                fn sum(&self) -> Self::Scalar {
                    self.scalar_reduce(0 as Self::Scalar, |acc, s| acc + s)
                }
            }
        )*
    }
}

macro_rules! impl_packed_upcast_sum {
    ($($vec:tt),*) => {
        $(
            impl UpcastSum for $vec {
                #[inline(always)]
                fn sum_upcast(&self) -> i64 {
                    self.scalar_reduce(0i64, |acc, s| acc + (s as i64))
                }
            }
        )*
    }
}
