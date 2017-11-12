use stdsimd::vendor::*;
use stdsimd::simd::*;

#[macro_use]
mod macros;

mod abs;
pub use self::abs::*;
mod sqrt;
pub use self::sqrt::*;
mod transmute;
pub use self::transmute::*;
mod round;
pub use self::round::*;
mod recip;
pub use self::recip::*;
mod hadd;
pub use self::hadd::*;
mod rsqrt;
pub use self::rsqrt::*;
mod cmp;
pub use self::cmp::*;

pub trait PackedSaturatingHadd {
    fn saturating_hadd(&self, other: Self) -> Self;
}

pub trait PackedHsub {
    fn hsub(&self, other: Self) -> Self;
}

pub trait PackedSaturatingHsub {
    fn saturating_hsub(&self, other: Self) -> Self;
}

pub trait PackedAddSub {
    fn addsub(&self, other: Self) -> Self;
}

pub trait PackedSaturatingAdd {
    fn saturating_add(&self, other: Self) -> Self;
}

#[cfg(target_feature = "ssse3")]
impl PackedSaturatingHadd for i16x8 {
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadds_epi16(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
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

#[cfg(target_feature = "sse2")]
impl PackedSaturatingAdd for u8x16 {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self {
        unsafe { _mm_adds_epu8(*self, other) }
    }
}

#[cfg(target_feature = "sse2")]
impl PackedSaturatingAdd for i8x16 {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self {
        unsafe { _mm_adds_epi8(*self, other) }
    }
}

#[cfg(target_feature = "sse2")]
impl PackedSaturatingAdd for u16x8 {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self {
        unsafe { _mm_adds_epu16(*self, other) }
    }
}

#[cfg(target_feature = "sse2")]
impl PackedSaturatingAdd for i16x8 {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self {
        unsafe { _mm_adds_epi16(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSaturatingAdd for u8x32 {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self {
        unsafe { _mm256_adds_epu8(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSaturatingAdd for i8x32 {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self {
        unsafe { _mm256_adds_epi8(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSaturatingAdd for u16x16 {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self {
        unsafe { _mm256_adds_epu16(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSaturatingAdd for i16x16 {
    #[inline(always)]
    fn saturating_add(&self, other: Self) -> Self {
        unsafe { _mm256_adds_epi16(*self, other) }
    }
}
