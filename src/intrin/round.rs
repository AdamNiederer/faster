use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedRound {
    fn round(&self) -> Self;
    fn ceil(&self) -> Self;
    fn floor(&self) -> Self;
    fn trunc(&self) -> Self;
}

rust_fallback_impl! {
    impl PackedRound for f32x4 where "sse4.1" {
        round => _mm_round_ps(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3];
        ceil => _mm_ceil_ps(), [0, 1, 2, 3];
        floor => _mm_floor_ps(), [0, 1, 2, 3];
        trunc => _mm_round_ps(_MM_FROUND_TRUNC), [0, 1, 2, 3];
    }
}

rust_fallback_impl! {
    impl PackedRound for f64x2 where "sse4.1" {
        round => _mm_round_pd(_MM_FROUND_TO_NEAREST_INT), [0, 1];
        ceil => _mm_ceil_pd(), [0, 1];
        floor => _mm_floor_pd(), [0, 1];
        trunc => _mm_round_pd(_MM_FROUND_TRUNC), [0, 1];
    }
}

rust_fallback_impl! {
    impl PackedRound for f32x8 where "avx" {
        round => _mm256_round_ps(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3, 4, 5, 6, 7];
        ceil => _mm256_ceil_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
        floor => _mm256_floor_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
        trunc => _mm256_round_ps(_MM_FROUND_TRUNC), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl PackedRound for f64x4 where "sse4.1" {
        round => _mm256_round_pd(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3];
        ceil => _mm256_ceil_pd(), [0, 1, 2, 3];
        floor => _mm256_floor_pd(), [0, 1, 2, 3];
        trunc => _mm256_round_pd(_MM_FROUND_TRUNC), [0, 1, 2, 3];
    }
}
