use stdsimd::vendor::*;
use stdsimd::simd::*;
use std::mem::transmute;

pub trait PackedAbs {
    type Out;
    fn abs(&self) -> Self::Out;
}

pub trait PackedTransmute {
    type i8s;
    type u8s;
    type i16s;
    type u16s;
    type i32s;
    type u32s;
    type f32s;
    type i64s;
    type u64s;
    type f64s;

    fn be_i8s(&self) -> Self::i8s;
    fn be_u8s(&self) -> Self::u8s;
    fn be_i16s(&self) -> Self::i16s;
    fn be_u16s(&self) -> Self::u16s;
    fn be_i32s(&self) -> Self::i32s;
    fn be_u32s(&self) -> Self::u32s;
    // TODO: Remove possibility of signalling NaNs
    unsafe fn be_f32s_unchecked(&self) -> Self::f32s;
    fn be_i64s(&self) -> Self::i64s;
    fn be_u64s(&self) -> Self::u64s;
    // TODO: Remove possibility of signalling NaNs
    unsafe fn be_f64s_unchecked(&self) -> Self::f64s;
}

pub trait PackedSqrt {
    fn sqrt(&self) -> Self;
}

pub trait PackedRsqrt {
    fn rsqrt(&self) -> Self;
}

pub trait PackedRound {
    fn round(&self) -> Self;
    fn ceil(&self) -> Self;
    fn floor(&self) -> Self;
    fn trunc(&self) -> Self;
}

pub trait PackedCmp {
    fn max(&self, other: Self) -> Self;
    fn min(&self, other: Self) -> Self;
}

pub trait PackedHadd {
    fn hadd(&self, other: Self) -> Self;
}

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


impl PackedAbs for f32x4 {
    type Out = f32x4;

    #[inline(always)]
    #[cfg(target_feature = "sse")]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_and_ps(*self, Self::splat(transmute::<u32, f32>(0x7FFFFFFF))) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse"))]
    fn abs(&self) -> Self::Out {
        Self::Out::new(self.extract(0).abs(),
                       self.extract(1).abs(),
                       self.extract(2).abs(),
                       self.extract(3).abs())
    }
}

impl PackedAbs for f64x2 {
    type Out = f64x2;

    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_and_pd(*self, Self::splat(transmute::<u64, f64>(0x7FFFFFFFFFFFFFFF))) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn abs(&self) -> Self::Out {
        Self::Out::new(self.extract(0).abs(),
                       self.extract(1).abs())
    }
}

impl PackedAbs for f32x8 {
    type Out = f32x8;

    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_and_ps(*self, Self::splat(transmute::<u32, f32>(0x7FFFFFFF))) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn abs(&self) -> Self::Out {
        Self::Out::new(self.extract(0).abs(),
                       self.extract(1).abs(),
                       self.extract(2).abs(),
                       self.extract(3).abs(),
                       self.extract(4).abs(),
                       self.extract(5).abs(),
                       self.extract(6).abs(),
                       self.extract(7).abs())
    }
}

impl PackedAbs for f64x4 {
    type Out = f64x4;

    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_and_pd(*self, Self::splat(transmute::<u64, f64>(0x7FFFFFFFFFFFFFFF))) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn abs(&self) -> Self::Out {
        Self::Out::new(self.extract(0).abs(),
                       self.extract(1).abs(),
                       self.extract(2).abs(),
                       self.extract(3).abs())
    }
}

impl PackedAbs for i8x16 {
    type Out = u8x16;

    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi8(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i8, u8>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(7).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(8).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(9).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(10).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(11).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(12).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(13).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(14).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(15).overflowing_abs().0) })
    }
}

impl PackedAbs for i16x8 {
    type Out = u16x8;

    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi16(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i16, u16>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(7).overflowing_abs().0) })
    }
}

impl PackedAbs for i32x4 {
    type Out = u32x4;

    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn abs(&self) -> Self::Out {
        unsafe { _mm_abs_epi32(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i32, u32>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(3).overflowing_abs().0) })
    }
}

#[cfg(target_feature = "avx2")]
impl PackedAbs for i8x32 {
    type Out = u8x32; // awaiting https://github.com/rust-lang-nursery/stdsimd/pull/173
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_abs_epi8(*self).as_u8x32() }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedAbs for i16x16 {
    type Out = u16x16; // awaiting https://github.com/rust-lang-nursery/stdsimd/pull/173
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        unsafe { _mm256_abs_epi16(*self).as_u16x16() }
    }
}

#[cfg(target_feature = "avx2")]
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
    #[cfg(target_feature = "sse2")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_ps(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3))
    }
}

impl PackedHadd for f64x2 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_pd(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1))
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
    #[cfg(target_feature = "ssse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi16(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  self.extract(4) + self.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3),
                  other.extract(4) + other.extract(5),
                  other.extract(6) + other.extract(7))
    }
}

impl PackedHadd for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi32(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3))
    }
}

#[cfg(target_feature = "avx2")]
impl PackedHadd for i16x16 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi16(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedHadd for i32x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi32(*self, other) }
    }
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
    #[cfg(target_feature = "sse4.1")]
    fn round(&self) -> Self {
        unsafe { _mm_round_ps(*self, _MM_FROUND_TO_NEAREST_INT) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn round(&self) -> Self {
        Self::new(self.extract(0).round(),
                  self.extract(1).round(),
                  self.extract(2).round(),
                  self.extract(3).round())
    }

    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn ceil(&self) -> Self {
        unsafe { _mm_ceil_ps(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn ceil(&self) -> Self {
        Self::new(self.extract(0).ceil(),
                  self.extract(1).ceil(),
                  self.extract(2).ceil(),
                  self.extract(3).ceil())
    }

    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn floor(&self) -> Self {
        unsafe { _mm_floor_ps(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn floor(&self) -> Self {
        Self::new(self.extract(0).floor(),
                  self.extract(1).floor(),
                  self.extract(2).floor(),
                  self.extract(3).floor())
    }

    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn trunc(&self) -> Self {
        unsafe { _mm_round_ps(*self, _MM_FROUND_TRUNC) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn trunc(&self) -> Self {
        Self::new(self.extract(0).trunc(),
                  self.extract(1).trunc(),
                  self.extract(2).trunc(),
                  self.extract(3).trunc())
    }
}

impl PackedRound for f64x2 {
    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn round(&self) -> Self {
        unsafe { _mm_round_pd(*self, _MM_FROUND_TO_NEAREST_INT) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn round(&self) -> Self {
        Self::new(self.extract(0).round(),
                  self.extract(1).round())
    }

    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn ceil(&self) -> Self {
        unsafe { _mm_ceil_pd(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn ceil(&self) -> Self {
        Self::new(self.extract(0).ceil(),
                  self.extract(1).ceil())
    }

    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn floor(&self) -> Self {
        unsafe { _mm_floor_pd(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn floor(&self) -> Self {
        Self::new(self.extract(0).floor(),
                  self.extract(1).floor())
    }

    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn trunc(&self) -> Self {
        unsafe { _mm_round_pd(*self, _MM_FROUND_TRUNC) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn trunc(&self) -> Self {
        Self::new(self.extract(0).trunc(),
                  self.extract(1).trunc())
    }
}

impl PackedRound for f32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn round(&self) -> Self {
        unsafe { _mm256_round_ps(*self, _MM_FROUND_TO_NEAREST_INT) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn round(&self) -> Self {
        Self::new(self.extract(0).round(),
                  self.extract(1).round(),
                  self.extract(2).round(),
                  self.extract(3).round(),
                  self.extract(4).round(),
                  self.extract(5).round(),
                  self.extract(6).round(),
                  self.extract(7).round())
    }

    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn ceil(&self) -> Self {
        unsafe { _mm256_ceil_ps(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn ceil(&self) -> Self {
        Self::new(self.extract(0).ceil(),
                  self.extract(1).ceil(),
                  self.extract(2).ceil(),
                  self.extract(3).ceil(),
                  self.extract(4).ceil(),
                  self.extract(5).ceil(),
                  self.extract(6).ceil(),
                  self.extract(7).ceil())
    }

    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn floor(&self) -> Self {
        unsafe { _mm256_floor_ps(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn floor(&self) -> Self {
        Self::new(self.extract(0).floor(),
                  self.extract(1).floor(),
                  self.extract(2).floor(),
                  self.extract(3).floor(),
                  self.extract(4).floor(),
                  self.extract(5).floor(),
                  self.extract(6).floor(),
                  self.extract(7).floor())
    }

    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn trunc(&self) -> Self {
        unsafe { _mm256_round_ps(*self, _MM_FROUND_TRUNC) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn trunc(&self) -> Self {
        Self::new(self.extract(0).trunc(),
                  self.extract(1).trunc(),
                  self.extract(2).trunc(),
                  self.extract(3).trunc(),
                  self.extract(4).trunc(),
                  self.extract(5).trunc(),
                  self.extract(6).trunc(),
                  self.extract(7).trunc())
    }
}

impl PackedRound for f64x4 {
    #[inline(always)]
    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn round(&self) -> Self {
        unsafe { _mm256_round_pd(*self, _MM_FROUND_TO_NEAREST_INT) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn round(&self) -> Self {
        Self::new(self.extract(0).round(),
                  self.extract(1).round(),
                  self.extract(2).round(),
                  self.extract(3).round())
    }

    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn ceil(&self) -> Self {
        unsafe { _mm256_ceil_pd(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn ceil(&self) -> Self {
        Self::new(self.extract(0).ceil(),
                  self.extract(1).ceil(),
                  self.extract(2).ceil(),
                  self.extract(3).ceil())
    }

    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn floor(&self) -> Self {
        unsafe { _mm256_floor_pd(*self) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn floor(&self) -> Self {
        Self::new(self.extract(0).floor(),
                  self.extract(1).floor(),
                  self.extract(2).floor(),
                  self.extract(3).floor())
    }

    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn trunc(&self) -> Self {
        unsafe { _mm256_round_pd(*self, _MM_FROUND_TRUNC) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn trunc(&self) -> Self {
        Self::new(self.extract(0).trunc(),
                  self.extract(1).trunc(),
                  self.extract(2).trunc(),
                  self.extract(3).trunc())
    }
}

macro_rules! impl_packed_transmute {
    ($($t:ty,)* ... $u8s:ty, $i8s:ty, $u16s:ty, $i16s:ty, $u32s:ty, $i32s:ty,
     $f32s:ty, $u64s:ty, $i64s:ty, $f64s:ty, $feat:expr, $nfeat:expr) => (
        $(
            impl PackedTransmute for $t {
                type i8s = $i8s;
                type u8s = $u8s;
                type i16s = $i16s;
                type u16s = $u16s;
                type i32s = $i32s;
                type u32s = $u32s;
                type f32s = $f32s;
                type i64s = $i64s;
                type u64s = $u64s;
                type f64s = $f64s;

                #[inline(always)]
                fn be_i8s(&self) -> Self::i8s {
                    unsafe { transmute::<Self, Self::i8s>(*self) }
                }
                #[inline(always)]
                fn be_u8s(&self) -> Self::u8s {
                    unsafe { transmute::<Self, Self::u8s>(*self) }
                }
                #[inline(always)]
                fn be_i16s(&self) -> Self::i16s {
                    unsafe { transmute::<Self, Self::i16s>(*self) }
                }
                #[inline(always)]
                fn be_u16s(&self) -> Self::u16s {
                    unsafe { transmute::<Self, Self::u16s>(*self) }
                }
                #[inline(always)]
                fn be_i32s(&self) -> Self::i32s {
                    unsafe { transmute::<Self, Self::i32s>(*self) }
                }
                #[inline(always)]
                fn be_u32s(&self) -> Self::u32s {
                    unsafe { transmute::<Self, Self::u32s>(*self) }
                }
                #[inline(always)]
                unsafe fn be_f32s_unchecked(&self) -> Self::f32s {
                    transmute::<Self, Self::f32s>(*self)
                }
                #[inline(always)]
                fn be_i64s(&self) -> Self::i64s {
                    unsafe { transmute::<Self, Self::i64s>(*self) }
                }
                #[inline(always)]
                fn be_u64s(&self) -> Self::u64s {
                    unsafe { transmute::<Self, Self::u64s>(*self) }
                }
                #[inline(always)]
                unsafe fn be_f64s_unchecked(&self) -> Self::f64s {
                    transmute::<Self, Self::f64s>(*self)
                }
            }
        )*
    );
}

impl_packed_transmute!(u8x32, i8x32, u16x16, i16x16, u32x8, i32x8, f32x8,
                       u64x4, i64x4, f64x4, ...
                       u8x32, i8x32, u16x16, i16x16, u32x8, i32x8,
                       f32x8, u64x4, i64x4, f64x4,
                       "avx", "avx512");
impl_packed_transmute!(u8x64, i8x64, u16x32, i16x32, u32x16, i32x16, f32x16,
                       u64x8, i64x8, f64x8, ...
                       u8x64, i8x64, u16x32, i16x32, u32x16, i32x16,
                       f32x16, u64x8, i64x8, f64x8,
                       "avx512", "avx1024");
impl_packed_transmute!(u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, f32x4,
                       u64x2, i64x2, f64x2, ...
                       u8x16, i8x16, u16x8, i16x8, u32x4, i32x4,
                       f32x4, u64x2, i64x2, f64x2,
                       "sse", "avx");

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
