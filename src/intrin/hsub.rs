// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use stdsimd::simd::*;
use intrin::transmute::*;

pub trait PackedHsub {
    fn hsub(&self, other: Self) -> Self;
}

impl PackedHsub for f32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_ps(_mm_shuffle_ps(*self, other, 0b01000100),
                             _mm_shuffle_ps(*self, other, 0b11101110)) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn hsub(&self, other: Self) -> Self {
        Self::new(self.extract(0) - self.extract(1),
                  other.extract(0) - other.extract(1),
                  self.extract(2) - self.extract(3),
                  other.extract(2) - other.extract(3))
    }
}

impl PackedHsub for f64x2 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_pd(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn hsub(&self, other: Self) -> Self {
        Self::new(self.extract(0) - self.extract(1),
                  other.extract(0) - other.extract(1))
    }
}

impl PackedHsub for f32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_ps(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked(),
                                _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn hsub(&self, other: Self) -> Self {
        Self::new(self.extract(0) - self.extract(1),
                  other.extract(0) - other.extract(1),
                  self.extract(2) - self.extract(3),
                  other.extract(2) - other.extract(3),
                  self.extract(4) - self.extract(5),
                  other.extract(4) - other.extract(5),
                  self.extract(6) - self.extract(7),
                  other.extract(6) - other.extract(7)) }
}

impl PackedHsub for f64x4 {
    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_pd(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn hsub(&self, other: Self) -> Self {
        Self::new(self.extract(0) - self.extract(1),
                  other.extract(0) - other.extract(1),
                  self.extract(2) - self.extract(3),
                  other.extract(2) - other.extract(3))
    }
}

impl PackedHsub for i16x8 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_epi16(_mm_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                _mm_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn hsub(&self, other: Self) -> Self {
        Self::new(self.extract(0) - self.extract(1),
                  other.extract(0) - other.extract(1),
                  self.extract(2) - self.extract(3),
                  other.extract(2) - other.extract(3),
                  self.extract(4) - self.extract(5),
                  other.extract(4) - other.extract(5),
                  self.extract(6) - self.extract(7),
                  other.extract(6) - other.extract(7))
    }
}

impl PackedHsub for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_epi32(_mm_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                _mm_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn hsub(&self, other: Self) -> Self {
        Self::new(self.extract(0) - self.extract(1),
                  other.extract(0) - other.extract(1),
                  self.extract(2) - self.extract(3),
                  other.extract(2) - other.extract(3))
    }
}

impl PackedHsub for i16x16 {
    #[cfg(target_feature = "avx2")]
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_epi16(_mm256_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                   _mm256_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn hsub(&self, other: Self) -> Self {
        Self::new(self.extract(0) - self.extract(1),
                  other.extract(0) - other.extract(1),
                  self.extract(2) - self.extract(3),
                  other.extract(2) - other.extract(3),
                  self.extract(4) - self.extract(5),
                  other.extract(4) - other.extract(5),
                  self.extract(6) - self.extract(7),
                  other.extract(6) - other.extract(7),
                  self.extract(8) - self.extract(9),
                  other.extract(8) - other.extract(9),
                  self.extract(10) - self.extract(11),
                  other.extract(10) - other.extract(11),
                  self.extract(12) - self.extract(13),
                  other.extract(12) - other.extract(13),
                  self.extract(14) - self.extract(15),
                  other.extract(14) - other.extract(15))
    }
}


impl PackedHsub for i32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_epi32(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                   _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn hsub(&self, other: Self) -> Self {
        Self::new(self.extract(0) - self.extract(1),
                  other.extract(0) - other.extract(1),
                  self.extract(2) - self.extract(3),
                  other.extract(2) - other.extract(3),
                  self.extract(4) - self.extract(5),
                  other.extract(4) - other.extract(5),
                  self.extract(6) - self.extract(7),
                  other.extract(6) - other.extract(7))
    }
}


#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;

    #[test]
    fn hsub_i16s() {
        assert_eq!(i16s::splat(1).hsub(i16s::splat(2)), i16s::interleave(0, 0));
        assert_eq!(i16s::interleave(1, 2).hsub(i16s::interleave(3, 4)), i16s::interleave(-1, -1));
    }

    #[test]
    fn hsub_i32s() {
        assert_eq!(i32s::splat(1).hsub(i32s::splat(2)), i32s::interleave(0, 0));
        assert_eq!(i32s::interleave(1, 2).hsub(i32s::interleave(3, 4)), i32s::interleave(-1, -1));
    }

    #[test]
    fn hsub_f32s() {
        assert_eq!(f32s::splat(1.0).hsub(f32s::splat(2.0)), f32s::interleave(0.0, 0.0));
        assert_eq!(f32s::interleave(1.0, 2.0).hsub(f32s::interleave(3.0, 4.0)), f32s::interleave(-1.0, -1.0));
    }

    #[test]
    fn hsub_f64s() {
        assert_eq!(f64s::splat(1.0).hsub(f64s::splat(2.0)), f64s::interleave(0.0, 0.0));
        assert_eq!(f64s::interleave(1.0, 2.0).hsub(f64s::interleave(3.0, 4.0)), f64s::interleave(-1.0, -1.0));
    }


}
