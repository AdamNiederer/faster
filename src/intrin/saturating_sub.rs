use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedSaturatingSub {
    fn saturating_sub(&self, other: Self) -> Self;
}

rust_fallback_impl_binary! {
    impl PackedSaturatingSub for i8x16 where "sse2" {
        saturating_sub => _mm_subs_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}
rust_fallback_impl_binary! {
    impl PackedSaturatingSub for i16x8 where "sse2" {
        saturating_sub => _mm_subs_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingSub for i8x32 where "avx2" {
        saturating_sub => _mm256_subs_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingSub for i16x16 where "avx2" {
        saturating_sub => _mm256_subs_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}
