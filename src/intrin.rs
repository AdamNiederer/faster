use stdsimd::vendor::*;
use stdsimd::simd::*;
use std::mem::transmute;
use vecs::{u8s, i8s, u16s, i16s, u32s, i32s, f32s, u64s, i64s, f64s};

pub trait PackedAbs {
    type Out;
    #[inline(always)]
    fn abs(&self) -> Self::Out;
}

pub trait PackedTransmute {
    #[inline(always)]
    fn as_i8s(&self) -> i8s;
    #[inline(always)]
    fn as_u8s(&self) -> u8s;
    #[inline(always)]
    fn as_i16s(&self) -> i16s;
    #[inline(always)]
    fn as_u16s(&self) -> u16s;
    #[inline(always)]
    fn as_i32s(&self) -> i32s;
    #[inline(always)]
    fn as_u32s(&self) -> u32s;
    #[inline(always)]
    fn as_f32s(&self) -> f32s;
    #[inline(always)]
    fn as_i64s(&self) -> i64s;
    #[inline(always)]
    fn as_u64s(&self) -> u64s;
    #[inline(always)]
    fn as_f64s(&self) -> f64s;
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
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_and_ps(*self, Self::splat(transmute::<u32, f32>(0x7FFFFFFF))) }
    }
}

impl PackedAbs for f64x2 {
    type Out = f64x2;
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_and_pd(*self, Self::splat(transmute::<u64, f64>(0x7FFFFFFFFFFFFFFF))) }
    }
}

impl PackedAbs for f32x8 {
    type Out = f32x8;
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_and_ps(*self, Self::splat(transmute::<u32, f32>(0x7FFFFFFF))) }
    }
}

impl PackedAbs for f64x4 {
    type Out = f64x4;
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_and_pd(*self, Self::splat(transmute::<u64, f64>(0x7FFFFFFFFFFFFFFF))) }
    }
}

impl PackedAbs for i8x16 {
    type Out = u8x16;
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi8(*self) }
    }
}

impl PackedAbs for i16x8 {
    type Out = u16x8;
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi16(*self) }
    }
}

impl PackedAbs for i32x4 {
    type Out = u32x4;
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi32(*self) }
    }
}

impl PackedAbs for i8x32 {
    type Out = u8x32; // awaiting https://github.com/rust-lang-nursery/stdsimd/pull/173
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_abs_epi8(*self).as_u8x32() }
    }
}

impl PackedAbs for i16x16 {
    type Out = u16x16; // awaiting https://github.com/rust-lang-nursery/stdsimd/pull/173
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_abs_epi16(*self).as_u16x16() }
    }
}

impl PackedAbs for i32x8 {
    type Out = u32x8; // awaiting https://github.com/rust-lang-nursery/stdsimd/pull/173
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_abs_epi32(*self).as_u32x8() }
    }
}

impl PackedSqrt for f32x4 {
    #[inline(always)]
    fn sqrt(&self) -> Self {
        unsafe { _mm_sqrt_ps(*self) }
    }
}

impl PackedSqrt for f64x2 {
    #[inline(always)]
    fn sqrt(&self) -> Self {
        unsafe { _mm_sqrt_pd(*self) }
    }
}

impl PackedSqrt for f32x8 {
    #[inline(always)]
    fn sqrt(&self) -> Self {
        unsafe { _mm256_sqrt_ps(*self) }
    }
}

impl PackedSqrt for f64x4 {
    #[inline(always)]
    fn sqrt(&self) -> Self {
        unsafe { _mm256_sqrt_pd(*self) }
    }
}

impl PackedHadd for f32x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_ps(*self, other) }
    }
}

impl PackedHadd for f64x2 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_pd(*self, other) }
    }
}

impl PackedHadd for f32x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_ps(*self, other) }
    }
}

impl PackedHadd for f64x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_pd(*self, other) }
    }
}

impl PackedHadd for i16x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi16(*self, other) }
    }
}

impl PackedHadd for i32x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi32(*self, other) }
    }
}

impl PackedHadd for i16x16 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi16(*self, other) }
    }
}

impl PackedHadd for i32x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi32(*self, other) }
    }
}

impl PackedSaturatingHadd for i16x8 {
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadds_epi16(*self, other) }
    }
}

impl PackedSaturatingHadd for i16x16 {
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadds_epi16(*self, other) }
    }
}

impl PackedHsub for f32x4 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_ps(*self, other) }
    }
}

impl PackedHsub for f64x2 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_pd(*self, other) }
    }
}

impl PackedHsub for f32x8 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_ps(*self, other) }
    }
}

impl PackedHsub for f64x4 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_pd(*self, other) }
    }
}

impl PackedHsub for i16x8 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_epi16(*self, other) }
    }
}

impl PackedHsub for i32x4 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_epi32(*self, other) }
    }
}

impl PackedHsub for i16x16 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_epi16(*self, other) }
    }
}

impl PackedHsub for i32x8 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_epi32(*self, other) }
    }
}

impl PackedSaturatingHsub for i16x8 {
    #[inline(always)]
    fn saturating_hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsubs_epi16(*self, other) }
    }
}

impl PackedSaturatingHsub for i16x16 {
    #[inline(always)]
    fn saturating_hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsubs_epi16(*self, other) }
    }
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

impl PackedCmp for f32x4 {
    #[inline(always)]
    fn min(&self, other: Self) -> Self {
        unsafe { _mm_min_ps(*self, other) }
    }
    #[inline(always)]
    fn max(&self, other: Self) -> Self {
        unsafe { _mm_max_ps(*self, other) }
    }
}

impl PackedCmp for f64x2 {
    #[inline(always)]
    fn min(&self, other: Self) -> Self {
        unsafe { _mm_min_pd(*self, other) }
    }
    #[inline(always)]
    fn max(&self, other: Self) -> Self {
        unsafe { _mm_max_pd(*self, other) }
    }
}

impl PackedCmp for f32x8 {
    #[inline(always)]
    fn min(&self, other: Self) -> Self {
        unsafe { _mm256_min_ps(*self, other) }
    }
    #[inline(always)]
    fn max(&self, other: Self) -> Self {
        unsafe { _mm256_max_ps(*self, other) }
    }
}

impl PackedCmp for f64x4 {
    #[inline(always)]
    fn min(&self, other: Self) -> Self {
        unsafe { _mm256_min_pd(*self, other) }
    }
    #[inline(always)]
    fn max(&self, other: Self) -> Self {
        unsafe { _mm256_max_pd(*self, other) }
    }
}

impl PackedRound for f32x4 {
    #[inline(always)]
    fn round(&self) -> Self {
        unsafe { _mm_round_ps(*self, _MM_FROUND_TO_NEAREST_INT) }
    }
    #[inline(always)]
    fn ceil(&self) -> Self {
        unsafe { _mm_ceil_ps(*self) }
    }
    #[inline(always)]
    fn floor(&self) -> Self {
        unsafe { _mm_floor_ps(*self) }
    }
    #[inline(always)]
    fn truncate(&self) -> Self {
        unsafe { _mm_round_ps(*self, _MM_FROUND_TRUNC) }
    }
}

impl PackedRound for f64x2 {
    #[inline(always)]
    fn round(&self) -> Self {
        unsafe { _mm_round_pd(*self, _MM_FROUND_TO_NEAREST_INT) }
    }
    #[inline(always)]
    fn ceil(&self) -> Self {
        unsafe { _mm_ceil_pd(*self) }
    }
    #[inline(always)]
    fn floor(&self) -> Self {
        unsafe { _mm_floor_pd(*self) }
    }
    #[inline(always)]
    fn truncate(&self) -> Self {
        unsafe { _mm_round_pd(*self, _MM_FROUND_TRUNC) }
    }
}

impl PackedRound for f32x8 {
    #[inline(always)]
    fn round(&self) -> Self {
        unsafe { _mm256_round_ps(*self, _MM_FROUND_TO_NEAREST_INT) }
    }
    #[inline(always)]
    fn ceil(&self) -> Self {
        unsafe { _mm256_ceil_ps(*self) }
    }
    #[inline(always)]
    fn floor(&self) -> Self {
        unsafe { _mm256_floor_ps(*self) }
    }
    #[inline(always)]
    fn truncate(&self) -> Self {
        unsafe { _mm256_round_ps(*self, _MM_FROUND_TRUNC) }
    }
}

impl PackedRound for f64x4 {
    #[inline(always)]
    fn round(&self) -> Self {
        unsafe { _mm256_round_pd(*self, _MM_FROUND_TO_NEAREST_INT) }
    }
    #[inline(always)]
    fn ceil(&self) -> Self {
        unsafe { _mm256_ceil_pd(*self) }
    }
    #[inline(always)]
    fn floor(&self) -> Self {
        unsafe { _mm256_floor_pd(*self) }
    }
    #[inline(always)]
    fn truncate(&self) -> Self {
        unsafe { _mm256_round_pd(*self, _MM_FROUND_TRUNC) }
    }
}

macro_rules! impl_packed_transmute {
    ($($t:ty),*) => (
        $(
            impl PackedTransmute for $t {
                #[inline(always)]
                fn as_i8s(&self) -> i8s {
                    unsafe { transmute::<Self, i8s>(*self) }
                }
                #[inline(always)]
                fn as_u8s(&self) -> u8s {
                    unsafe { transmute::<Self, u8s>(*self) }
                }
                #[inline(always)]
                fn as_i16s(&self) -> i16s {
                    unsafe { transmute::<Self, i16s>(*self) }
                }
                #[inline(always)]
                fn as_u16s(&self) -> u16s {
                    unsafe { transmute::<Self, u16s>(*self) }
                }
                #[inline(always)]
                fn as_i32s(&self) -> i32s {
                    unsafe { transmute::<Self, i32s>(*self) }
                }
                #[inline(always)]
                fn as_u32s(&self) -> u32s {
                    unsafe { transmute::<Self, u32s>(*self) }
                }
                #[inline(always)]
                fn as_f32s(&self) -> f32s {
                    unsafe { transmute::<Self, f32s>(*self) }
                }
                #[inline(always)]
                fn as_i64s(&self) -> i64s {
                    unsafe { transmute::<Self, i64s>(*self) }
                }
                #[inline(always)]
                fn as_u64s(&self) -> u64s {
                    unsafe { transmute::<Self, u64s>(*self) }
                }
                #[inline(always)]
                fn as_f64s(&self) -> f64s {
                    unsafe { transmute::<Self, f64s>(*self) }
                }
            }
        )*
    );
}

impl_packed_transmute!(u8s, i8s, u16s, i16s, u32s, i32s, f32s, u64s, i64s, f64s);
