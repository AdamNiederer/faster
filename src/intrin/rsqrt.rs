use stdsimd::vendor::*;
use stdsimd::simd::*;

// TODO: Guards and non-simd

pub trait PackedRsqrt {
    fn rsqrt(&self) -> Self;
}

impl PackedRsqrt for f32x4 {
    #[inline(always)]
    fn rsqrt(&self) -> Self {
        unsafe { _mm_rsqrt_ps(*self) }
    }
}

impl PackedRsqrt for f32x8 {
    #[inline(always)]
    fn rsqrt(&self) -> Self {
        unsafe { _mm256_rsqrt_ps(*self) }
    }
}
