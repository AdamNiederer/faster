macro_rules! impl_packed_swizzle {
    ($vec:tt, $uvec:tt, $feat:expr, $mmfn:tt, ($($c:expr),*), ($($a:expr, $b:expr),*)) => {
        impl Swizzle for $vec {
            #[cfg(not(target_feature = $feat))]
            #[inline(always)]
            fn flip(&self) -> Self {
                $vec::new($(self.extract($b), self.extract($a)),*)
            }

            #[cfg(target_feature = $feat)]
            #[inline(always)]
            fn flip(&self) -> Self {
                unsafe {
                    transmute($mmfn(self.be_i8s(), $uvec::new($($c),*).be_i8s()))
                }
            }
        }
    }
}
