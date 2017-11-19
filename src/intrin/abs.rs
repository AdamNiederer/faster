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

impl PackedAbs for i64x2 {
    type Out = u64x2;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i64, u64>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(1).overflowing_abs().0) })
    }
}

impl PackedAbs for i64x4 {
    type Out = u64x4;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i64, u64>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(3).overflowing_abs().0) })
    }
}

impl PackedAbs for i64x8 {
    type Out = u64x8;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i64, u64>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(7).overflowing_abs().0) })
    }
}


#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;
    use std::f32::INFINITY;

    #[test]
    fn abs_i8s() {
        for i in -128..127 {
            assert_eq!(i8s::splat(i).abs().extract(0), (i as i64).abs() as u8);
        }
    }

    #[test]
    fn abs_i16s() {
        for i in -32768..32767 {
            assert_eq!(i16s::splat(i).abs().extract(0), (i as i64).abs() as u16);
        }
    }

    #[test]
    fn abs_i32s() {
        for i in -65536..65536 {
            assert_eq!(i32s::splat(i).abs().extract(0), (i as i64).abs() as u32);
        }
    }

    #[test]
    fn abs_i64s() {
        for i in -65536..65536 {
            assert_eq!(i64s::splat(i).abs().extract(0), (i as i64).abs() as u64);
        }
    }

    #[test]
    fn abs_f32s() {
        let mut i = -1024.0;
        while i < 1024.0 {
            // This test has some pretty significant float error if done on x86
            assert_eq!(f32s::splat(i).abs().extract(0), i.abs());
            i += 1.0
        }
    }

    #[test]
    fn abs_f64s() {
        let mut i = -1024.0;
        while i < 1024.0 {
            // This test has some pretty significant float error if done on x86
            assert_eq!(f64s::splat(i).abs().extract(0), i.abs());
            i += 1.0
        }
    }
}
