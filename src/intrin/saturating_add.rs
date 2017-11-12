use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedSaturatingAdd {
    fn saturating_add(&self, other: Self) -> Self;
}

rust_fallback_impl_binary! {
    impl PackedSaturatingAdd for u8x16 where "sse2" {
        saturating_add => _mm_adds_epu8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingAdd for i8x16 where "sse2" {
        saturating_add => _mm_adds_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingAdd for u16x8 where "sse2" {
        saturating_add => _mm_adds_epu16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingAdd for i16x8 where "sse2" {
        saturating_add => _mm_adds_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}


rust_fallback_impl_binary! {
    impl PackedSaturatingAdd for u8x32 where "avx2" {
        saturating_add => _mm256_adds_epu8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingAdd for i8x32 where "avx2" {
        saturating_add => _mm256_adds_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingAdd for u16x16 where "avx2" {
        saturating_add => _mm256_adds_epu16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingAdd for i16x16 where "avx2" {
        saturating_add => _mm256_adds_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}
