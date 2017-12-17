// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
use intrin::transmute::*;

pub trait PackedHadd {
    fn hadd(&self, other: Self) -> Self;
}

impl PackedHadd for f32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_ps(_mm_shuffle_ps(*self, other, 0b01000100),
                             _mm_shuffle_ps(*self, other, 0b11101110)) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse3"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(2) + other.extract(3))
    }
}

impl PackedHadd for f64x2 {
    #[inline(always)]
    #[cfg(target_feature = "sse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_pd(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse3"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1))
    }
}

impl PackedHadd for f32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_ps(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked(),
                                _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(2) + other.extract(3),
                  self.extract(4) + self.extract(5),
                  other.extract(4) + other.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(6) + other.extract(7)) }
}

impl PackedHadd for f64x4 {
    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_pd(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(2) + other.extract(3))
    }
}

impl PackedHadd for i16x8 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi16(_mm_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                _mm_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(2) + other.extract(3),
                  self.extract(4) + self.extract(5),
                  other.extract(4) + other.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(6) + other.extract(7))
    }
}

impl PackedHadd for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi32(_mm_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                _mm_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(2) + other.extract(3))
    }
}

impl PackedHadd for i16x16 {
    #[cfg(target_feature = "avx2")]
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi16(_mm256_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                   _mm256_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(2) + other.extract(3),
                  self.extract(4) + self.extract(5),
                  other.extract(4) + other.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(6) + other.extract(7),
                  self.extract(8) + self.extract(9),
                  other.extract(8) + other.extract(9),
                  self.extract(10) + self.extract(11),
                  other.extract(10) + other.extract(11),
                  self.extract(12) + self.extract(13),
                  other.extract(12) + other.extract(13),
                  self.extract(14) + self.extract(15),
                  other.extract(14) + other.extract(15))
    }
}


impl PackedHadd for i32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi32(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                   _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(2) + other.extract(3),
                  self.extract(4) + self.extract(5),
                  other.extract(4) + other.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(6) + other.extract(7))
    }
}


#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;

    #[test]
    fn hadd_i16s() {
        assert_eq!(i16s::splat(1).hadd(i16s::splat(2)), i16s::interleave(2, 4));
        assert_eq!(i16s::interleave(1, 2).hadd(i16s::interleave(3, 4)), i16s::interleave(3, 7));
    }

    #[test]
    fn hadd_i32s() {
        assert_eq!(i32s::splat(1).hadd(i32s::splat(2)), i32s::interleave(2, 4));
        assert_eq!(i32s::interleave(1, 2).hadd(i32s::interleave(3, 4)), i32s::interleave(3, 7));
    }

    #[test]
    fn hadd_f32s() {
        assert_eq!(f32s::splat(1.0).hadd(f32s::splat(2.0)), f32s::interleave(2.0, 4.0));
        assert_eq!(f32s::interleave(1.0, 2.0).hadd(f32s::interleave(3.0, 4.0)), f32s::interleave(3.0, 7.0));
    }

    #[test]
    fn hadd_f64s() {
        assert_eq!(f64s::splat(1.0).hadd(f64s::splat(2.0)), f64s::interleave(2.0, 4.0));
        assert_eq!(f64s::interleave(1.0, 2.0).hadd(f64s::interleave(3.0, 4.0)), f64s::interleave(3.0, 7.0));
    }


}
