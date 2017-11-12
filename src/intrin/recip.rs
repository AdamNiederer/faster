use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedRecip {
    fn recip(&self) -> Self;
}

rust_fallback_impl! {
    impl PackedRecip for f32x8 where "avx" {
        recip => _mm256_rcp_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl PackedRecip for f32x4 where "sse" {
        recip => _mm_rcp_ps(), [0, 1, 2, 3];
    }
}
