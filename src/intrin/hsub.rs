// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
use crate::intrin::transmute::*;
use crate::std::ops::Sub;

pub trait HSub {
    /// Return a vector containing the interleaved differences of elements in
    /// `self` and `other`. The returned vector will begin with the difference
    /// of the first two elements in `self`, and end with the difference of the
    /// last two elements in `other`
    fn hsub(&self, other: Self) -> Self;
}

#[cfg(target_feature = "sse3")]
impl HSub for f32x4 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_ps(_mm_shuffle_ps(*self, other, 0b01000100),
                             _mm_shuffle_ps(*self, other, 0b11101110)) }
    }
}

#[cfg(target_feature = "sse3")]
impl HSub for f64x2 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_pd(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
impl HSub for f32x8 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_ps(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked(),
                                _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked()) }
    }
}

#[cfg(target_feature = "avx")]
impl HSub for f64x4 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_pd(*self, other) }
    }
}

#[cfg(target_feature = "ssse3")]
impl HSub for i16x8 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_epi16(_mm_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                _mm_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

#[cfg(target_feature = "ssse3")]
impl HSub for i32x4 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsub_epi32(_mm_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                _mm_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }
}

#[cfg(target_feature = "avx2")]
impl HSub for i16x16 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_epi16(_mm256_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                   _mm256_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

#[cfg(target_feature = "avx2")]
impl HSub for i32x8 {
    #[inline(always)]
    fn hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsub_epi32(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                   _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }
}

impl HSub for u64x2 { hop!(hsub, Sub::sub, 0, 1); }
impl HSub for u64x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
impl HSub for u64x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HSub for u32x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
impl HSub for u32x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HSub for u32x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HSub for u16x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HSub for u16x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HSub for u16x32 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl HSub for u8x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HSub for u8x32 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl HSub for u8x64 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }
impl HSub for i64x2 { hop!(hsub, Sub::sub, 0, 1); }
impl HSub for i64x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
impl HSub for i64x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "ssse3"))]
impl HSub for i32x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
#[cfg(not(target_feature = "avx2"))]
impl HSub for i32x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HSub for i32x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
#[cfg(not(target_feature = "ssse3"))]
impl HSub for i16x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "avx2"))]
impl HSub for i16x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HSub for i16x32 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl HSub for i8x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HSub for i8x32 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl HSub for i8x64 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }
#[cfg(not(target_feature = "sse3"))]
impl HSub for f64x2 { hop!(hsub, Sub::sub, 0, 1); }
#[cfg(not(target_feature = "avx"))]
impl HSub for f64x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
impl HSub for f64x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "sse3"))]
impl HSub for f32x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
#[cfg(not(target_feature = "avx2"))]
impl HSub for f32x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HSub for f32x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }


#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;

    #[test]
    fn hsub_i8s() {
        assert_eq!(i8s(1).hsub(i8s(2)), i8s::interleave(0, 0));
        assert_eq!(i8s::interleave(1, 2).hsub(i8s::interleave(3, 4)), i8s::interleave(-1, -1));
    }

    #[test]
    fn hsub_i16s() {
        assert_eq!(i16s(1).hsub(i16s(2)), i16s::interleave(0, 0));
        assert_eq!(i16s::interleave(1, 2).hsub(i16s::interleave(3, 4)), i16s::interleave(-1, -1));
    }

    #[test]
    fn hsub_i32s() {
        assert_eq!(i32s(1).hsub(i32s(2)), i32s::interleave(0, 0));
        assert_eq!(i32s::interleave(1, 2).hsub(i32s::interleave(3, 4)), i32s::interleave(-1, -1));
    }

    #[test]
    fn hsub_i64s() {
        assert_eq!(i64s(1).hsub(i64s(2)), i64s::interleave(0, 0));
        assert_eq!(i64s::interleave(1, 2).hsub(i64s::interleave(3, 4)), i64s::interleave(-1, -1));
    }

    #[test]
    fn hsub_u8s() {
        assert_eq!(u8s(1).hsub(u8s(2)), u8s::interleave(0, 0));
        assert_eq!(u8s::interleave(2, 1).hsub(u8s::interleave(4, 3)), u8s::interleave(1, 1));
    }

    #[test]
    fn hsub_u16s() {
        assert_eq!(u16s(1).hsub(u16s(2)), u16s::interleave(0, 0));
        assert_eq!(u16s::interleave(2, 1).hsub(u16s::interleave(4, 3)), u16s::interleave(1, 1));
    }

    #[test]
    fn hsub_u32s() {
        assert_eq!(u32s(1).hsub(u32s(2)), u32s::interleave(0, 0));
        assert_eq!(u32s::interleave(2, 1).hsub(u32s::interleave(4, 3)), u32s::interleave(1, 1));
    }

    #[test]
    fn hsub_u64s() {
        assert_eq!(u64s(1).hsub(u64s(2)), u64s::interleave(0, 0));
        assert_eq!(u64s::interleave(2, 1).hsub(u64s::interleave(4, 3)), u64s::interleave(1, 1));
    }

    #[test]
    fn hsub_f32s() {
        assert_eq!(f32s(1.0).hsub(f32s(2.0)), f32s::interleave(0.0, 0.0));
        assert_eq!(f32s::interleave(1.0, 2.0).hsub(f32s::interleave(3.0, 4.0)), f32s::interleave(-1.0, -1.0));
    }

    #[test]
    fn hsub_f64s() {
        assert_eq!(f64s(1.0).hsub(f64s(2.0)), f64s::interleave(0.0, 0.0));
        assert_eq!(f64s::interleave(1.0, 2.0).hsub(f64s::interleave(3.0, 4.0)), f64s::interleave(-1.0, -1.0));
    }
}
