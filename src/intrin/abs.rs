// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use stdsimd::simd::*;
use std::mem::transmute;

// TODO: AVX2 scalar impls

pub trait PackedAbs {
    type Out;
    fn abs(&self) -> Self::Out;
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
