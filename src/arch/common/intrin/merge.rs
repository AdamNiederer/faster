macro_rules! impl_packed_merge {
    ($vec:ty, $uvec:tt, $uscl:tt, $mmfn:expr, $feat:expr, ($($a:expr),*), ($($b:expr),*), $($na:expr, $nb:expr),*) => {
        #[cfg(not(target_feature = $feat))]
        impl Merge for $vec {

            #[inline(always)]
            fn merge_halves(&self, other: Self) -> Self {
                unsafe {
                    Self::new($(self.extract_unchecked($a)),*,
                              $(other.extract_unchecked($b)),*)
                }
            }

            #[inline(always)]
            fn merge_interleaved(&self, other: Self) -> Self {
                unsafe {
                    Self::new($(self.extract_unchecked($na), other.extract_unchecked($nb)),*)
                }
            }

            #[inline(always)]
            fn merge_partitioned(&self, other: Self, offset: usize) -> Self {
                assert!(offset < Self::WIDTH);
                let mut ret = self.clone();
                for i in offset..Self::WIDTH {
                    unsafe {
                        ret = ret.replace_unchecked(i, other.extract_unchecked(i));
                    }
                }
                ret
            }
        }

        #[cfg(target_feature = $feat)]
        impl Merge for $vec {

            #[inline(always)]
            fn merge_halves(&self, other: Self) -> Self {
                unsafe {
                    transmute($mmfn(
                        self.be_i8s(), other.be_i8s(),
                        transmute($uvec::halfs($uscl::min_value(), $uscl::max_value()))))
                }
            }

            #[inline(always)]
            fn merge_interleaved(&self, other: Self) -> Self {
                unsafe {
                    transmute($mmfn(
                        self.be_i8s(), other.be_i8s(),
                        transmute($uvec::interleave($uscl::min_value(), $uscl::max_value()))))
                }
            }

            #[inline(always)]
            fn merge_partitioned(&self, other: Self, offset: usize) -> Self {
                unsafe {
                    transmute($mmfn(
                        self.be_i8s(), other.be_i8s(),
                        transmute(Self::partition_mask(offset))))
                }
            }
        }
    }
}

