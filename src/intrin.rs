use stdsimd::vendor::*;
use stdsimd::simd::*;
use std::mem::transmute;

pub trait PackedAbs {
    type Out;
    #[inline(always)]
    fn abs(&self) -> Self::Out;
}

pub trait PackedSqrt {
    #[inline(always)]
    fn sqrt(&self) -> Self;
}

pub trait PackedRsqrt {
    #[inline(always)]
    fn rsqrt(&self) -> Self;
}

pub trait PackedRound {
    #[inline(always)]
    fn round(&self) -> Self;
    #[inline(always)]
    fn ceil(&self) -> Self;
    #[inline(always)]
    fn floor(&self) -> Self;
    #[inline(always)]
    fn truncate(&self) -> Self;
}

pub trait PackedCmp {
    #[inline(always)]
    fn max(&self, other: Self) -> Self;
    #[inline(always)]
    fn min(&self, other: Self) -> Self;
}

pub trait PackedHadd {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self;
}

pub trait PackedSaturatingHadd {
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self;
}

pub trait PackedHsub {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self;
}

pub trait PackedSaturatingHsub {
    #[inline(always)]
    fn saturating_hsub(&self, other: Self) -> Self;
}

pub trait PackedAddSub {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self;
}

pub trait PackedSaturatingAdd {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self;
}

impl PackedAbs for f32x4 {
    type Out = f32x4;
    fn abs(&self) -> Self::Out {
        unsafe { _mm_and_ps(*self, Self::splat(transmute::<u32, f32>(0x7FFFFFFF))) }
    }
}

impl PackedAbs for f64x2 {
    type Out = f64x2;
    fn abs(&self) -> Self::Out {
        unsafe { _mm_and_pd(*self, Self::splat(transmute::<u64, f64>(0x7FFFFFFFFFFFFFFF))) }
    }
}

impl PackedAbs for f32x8 {
    type Out = f32x8;
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_and_ps(*self, Self::splat(transmute::<u32, f32>(0x7FFFFFFF))) }
    }
}

impl PackedAbs for f64x4 {
    type Out = f64x4;
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_and_pd(*self, Self::splat(transmute::<u64, f64>(0x7FFFFFFFFFFFFFFF))) }
    }
}

impl PackedAbs for i8x16 {
    type Out = u8x16;
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi8(*self) }
    }
}

impl PackedAbs for i16x8 {
    type Out = u16x8;
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi16(*self) }
    }
}

impl PackedAbs for i32x4 {
    type Out = u32x4;
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi32(*self) }
    }
}

impl PackedAbs for i8x32 {
    type Out = i8x32; // u8x32; awaiting https://github.com/rust-lang-nursery/stdsimd/pull/173
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_abs_epi8(*self) }
    }
}

impl PackedAbs for i16x16 {
    type Out = i16x16; // u16x16; awaiting https://github.com/rust-lang-nursery/stdsimd/pull/173
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_abs_epi16(*self) }
    }
}

impl PackedAbs for i32x8 {
    type Out = i32x8; // u32x8; awaiting https://github.com/rust-lang-nursery/stdsimd/pull/173
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_abs_epi32(*self) }
    }
}

impl PackedSqrt for f32x4 {
    fn sqrt(&self) -> Self {
        unsafe { _mm_sqrt_ps(*self) }
    }
}

impl PackedSqrt for f64x2 {
    fn sqrt(&self) -> Self {
        unsafe { _mm_sqrt_pd(*self) }
    }
}

impl PackedSqrt for f32x8 {
    fn sqrt(&self) -> Self {
        unsafe { _mm256_sqrt_ps(*self) }
    }
}

impl PackedSqrt for f64x4 {
    fn sqrt(&self) -> Self {
        unsafe { _mm256_sqrt_pd(*self) }
    }
}

impl PackedHadd for f32x4 {
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_ps(*self, other) }
    }
}

impl PackedHadd for f64x2 {
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_pd(*self, other) }
    }
}

impl PackedHadd for f32x8 {
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_ps(*self, other) }
    }
}

impl PackedHadd for f64x4 {
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_pd(*self, other) }
    }
}

impl PackedHadd for i16x8 {
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi16(*self, other) }
    }
}

impl PackedHadd for i32x4 {
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi32(*self, other) }
    }
}

impl PackedHadd for i16x16 {
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi16(*self, other) }
    }
}

impl PackedHadd for i32x8 {
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi32(*self, other) }
    }
}

impl PackedSaturatingHadd for i16x8 {
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadds_epi16(*self, other) }
    }
}

impl PackedSaturatingHadd for i16x16 {
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadds_epi16(*self, other) }
    }
}

impl PackedHsub for f32x4 {
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_ps(*self, other) }
    }
}

impl PackedHsub for f64x2 {
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_pd(*self, other) }
    }
}

impl PackedHsub for f32x8 {
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_ps(*self, other) }
    }
}

impl PackedHsub for f64x4 {
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_pd(*self, other) }
    }
}

impl PackedHsub for i16x8 {
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_epi16(*self, other) }
    }
}

impl PackedHsub for i32x4 {
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_epi32(*self, other) }
    }
}

impl PackedHsub for i16x16 {
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_epi16(*self, other) }
    }
}

impl PackedHsub for i32x8 {
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_epi32(*self, other) }
    }
}

impl PackedSaturatingHsub for i16x8 {
    fn saturating_hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsubs_epi16(*self, other) }
    }
}

impl PackedSaturatingHsub for i16x16 {
    fn saturating_hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsubs_epi16(*self, other) }
    }
}

impl PackedAddSub for f32x4 {
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm_addsub_ps(*self, other) }
    }
}

impl PackedAddSub for f64x2 {
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm_addsub_pd(*self, other) }
    }
}

impl PackedAddSub for f32x8 {
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm256_addsub_ps(*self, other) }
    }
}

impl PackedAddSub for f64x4 {
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm256_addsub_pd(*self, other) }
    }
}

impl PackedRsqrt for f32x4 {
    fn rsqrt(&self) -> Self {
        unsafe { _mm_rsqrt_ps(*self) }
    }
}

impl PackedRsqrt for f32x8 {
    fn rsqrt(&self) -> Self {
        unsafe { _mm256_rsqrt_ps(*self) }
    }
}

impl PackedCmp for f32x4 {
    fn min(&self, other: Self) -> Self {
        unsafe { _mm_min_ps(*self, other) }
    }
    fn max(&self, other: Self) -> Self {
        unsafe { _mm_max_ps(*self, other) }
    }
}

impl PackedCmp for f64x2 {
    fn min(&self, other: Self) -> Self {
        unsafe { _mm_min_pd(*self, other) }
    }
    fn max(&self, other: Self) -> Self {
        unsafe { _mm_max_pd(*self, other) }
    }
}

impl PackedCmp for f32x8 {
    fn min(&self, other: Self) -> Self {
        unsafe { _mm256_min_ps(*self, other) }
    }
    fn max(&self, other: Self) -> Self {
        unsafe { _mm256_max_ps(*self, other) }
    }
}

impl PackedCmp for f64x4 {
    fn min(&self, other: Self) -> Self {
        unsafe { _mm256_min_pd(*self, other) }
    }
    fn max(&self, other: Self) -> Self {
        unsafe { _mm256_max_pd(*self, other) }
    }
}

impl PackedRound for f32x4 {
    fn round(&self) -> Self {
        unsafe { _mm_round_ps(*self, _MM_FROUND_TO_NEAREST_INT) }
    }
    fn ceil(&self) -> Self {
        unsafe { _mm_ceil_ps(*self) }
    }
    fn floor(&self) -> Self {
        unsafe { _mm_floor_ps(*self) }
    }
    fn truncate(&self) -> Self {
        unsafe { _mm_round_ps(*self, _MM_FROUND_TRUNC) }
    }
}

impl PackedRound for f64x2 {
    fn round(&self) -> Self {
        unsafe { _mm_round_pd(*self, _MM_FROUND_TO_NEAREST_INT) }
    }
    fn ceil(&self) -> Self {
        unsafe { _mm_ceil_pd(*self) }
    }
    fn floor(&self) -> Self {
        unsafe { _mm_floor_pd(*self) }
    }
    fn truncate(&self) -> Self {
        unsafe { _mm_round_pd(*self, _MM_FROUND_TRUNC) }
    }
}

impl PackedRound for f32x8 {
    fn round(&self) -> Self {
        unsafe { _mm256_round_ps(*self, _MM_FROUND_TO_NEAREST_INT) }
    }
    fn ceil(&self) -> Self {
        unsafe { _mm256_ceil_ps(*self) }
    }
    fn floor(&self) -> Self {
        unsafe { _mm256_floor_ps(*self) }
    }
    fn truncate(&self) -> Self {
        unsafe { _mm256_round_ps(*self, _MM_FROUND_TRUNC) }
    }
}

impl PackedRound for f64x4 {
    fn round(&self) -> Self {
        unsafe { _mm256_round_pd(*self, _MM_FROUND_TO_NEAREST_INT) }
    }
    fn ceil(&self) -> Self {
        unsafe { _mm256_ceil_pd(*self) }
    }
    fn floor(&self) -> Self {
        unsafe { _mm256_floor_pd(*self) }
    }
    fn truncate(&self) -> Self {
        unsafe { _mm256_round_pd(*self, _MM_FROUND_TRUNC) }
    }
}
