use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedAddSub {
    fn addsub(&self, other: Self) -> Self;
}

impl PackedAddSub for f32x4 {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm_addsub_ps(*self, other) }
    }
}

impl PackedAddSub for f64x2 {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm_addsub_pd(*self, other) }
    }
}

impl PackedAddSub for f32x8 {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm256_addsub_ps(*self, other) }
    }
}

impl PackedAddSub for f64x4 {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm256_addsub_pd(*self, other) }
    }
}
