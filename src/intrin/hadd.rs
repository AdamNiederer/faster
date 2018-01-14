// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
use intrin::transmute::*;
use core_or_std::ops::Add;

pub trait PackedHadd {
    /// Return a vector containing the interleaved sums of elements in `self`
    /// and `other`. The returned vector will begin with the sum of the first
    /// two elements in `self`, and end with the sum of the last two elements in
    /// `other`
    fn hadd(&self, other: Self) -> Self;
}

#[cfg(target_feature = "sse3")]
impl PackedHadd for f32x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_ps(_mm_shuffle_ps(*self, other, 0b01000100),
                             _mm_shuffle_ps(*self, other, 0b11101110)) }
    }
}

#[cfg(target_feature = "sse3")]
impl PackedHadd for f64x2 {
    #[inline(always)]
    #[cfg(target_feature = "sse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_pd(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedHadd for f32x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_ps(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked(),
                                _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked()) }
    }
}

#[cfg(target_feature = "avx")]
impl PackedHadd for f64x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_pd(*self, other) }
    }
}

#[cfg(target_feature = "ssse3")]
impl PackedHadd for i16x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi16(_mm_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                _mm_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

#[cfg(target_feature = "ssse3")]
impl PackedHadd for i32x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi32(_mm_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                _mm_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedHadd for i16x16 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi16(_mm256_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                   _mm256_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}


#[cfg(target_feature = "avx2")]
impl PackedHadd for i32x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi32(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                   _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }
}

impl PackedHadd for u64x2 { hop!(hadd, Add::add, 0, 1); }
impl PackedHadd for u64x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl PackedHadd for u64x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl PackedHadd for u32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl PackedHadd for u32x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl PackedHadd for u32x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl PackedHadd for u16x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl PackedHadd for u16x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl PackedHadd for u16x32 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl PackedHadd for u8x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl PackedHadd for u8x32 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl PackedHadd for u8x64 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }
impl PackedHadd for i64x2 { hop!(hadd, Add::add, 0, 1); }
impl PackedHadd for i64x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl PackedHadd for i64x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "ssse3"))]
impl PackedHadd for i32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
#[cfg(not(target_feature = "avx2"))]
impl PackedHadd for i32x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl PackedHadd for i32x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
#[cfg(not(target_feature = "ssse3"))]
impl PackedHadd for i16x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "avx2"))]
impl PackedHadd for i16x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl PackedHadd for i16x32 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl PackedHadd for i8x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl PackedHadd for i8x32 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl PackedHadd for i8x64 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }
#[cfg(not(target_feature = "sse3"))]
impl PackedHadd for f64x2 { hop!(hadd, Add::add, 0, 1); }
#[cfg(not(target_feature = "avx"))]
impl PackedHadd for f64x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl PackedHadd for f64x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "sse3"))]
impl PackedHadd for f32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
#[cfg(not(target_feature = "avx2"))]
impl PackedHadd for f32x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl PackedHadd for f32x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }

#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;

    #[test]
    fn hadd_i8s() {
        assert_eq!(i8s::splat(1).hadd(i8s::splat(2)), i8s::interleave(2, 4));
        assert_eq!(i8s::interleave(1, 2).hadd(i8s::interleave(3, 4)), i8s::interleave(3, 7));
    }

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
    fn hadd_i64s() {
        assert_eq!(i64s::splat(1).hadd(i64s::splat(2)), i64s::interleave(2, 4));
        assert_eq!(i64s::interleave(1, 2).hadd(i64s::interleave(3, 4)), i64s::interleave(3, 7));
    }

    #[test]
    fn hadd_u8s() {
        assert_eq!(u8s::splat(1).hadd(u8s::splat(2)), u8s::interleave(2, 4));
        assert_eq!(u8s::interleave(1, 2).hadd(u8s::interleave(3, 4)), u8s::interleave(3, 7));
    }

    #[test]
    fn hadd_u16s() {
        assert_eq!(u16s::splat(1).hadd(u16s::splat(2)), u16s::interleave(2, 4));
        assert_eq!(u16s::interleave(1, 2).hadd(u16s::interleave(3, 4)), u16s::interleave(3, 7));
    }

    #[test]
    fn hadd_u32s() {
        assert_eq!(u32s::splat(1).hadd(u32s::splat(2)), u32s::interleave(2, 4));
        assert_eq!(u32s::interleave(1, 2).hadd(u32s::interleave(3, 4)), u32s::interleave(3, 7));
    }

    #[test]
    fn hadd_u64s() {
        assert_eq!(u64s::splat(1).hadd(u64s::splat(2)), u64s::interleave(2, 4));
        assert_eq!(u64s::interleave(1, 2).hadd(u64s::interleave(3, 4)), u64s::interleave(3, 7));
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
