// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
use crate::intrin::transmute::*;

pub trait SaturatingHAdd {
    /// Return a vector containing the interleaved sums of elements in `self`
    /// and `other`, using saturating addition. The returned vector will begin
    /// with the sum of the first two elements in `self`, and end with the sum
    /// of the last two elements in `other`
    fn saturating_hadd(&self, other: Self) -> Self;
}

#[cfg(target_feature = "ssse3")]
impl SaturatingHAdd for i16x8 {
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadds_epi16(_mm_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                 _mm_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

#[cfg(target_feature = "avx2")]
impl SaturatingHAdd for i16x16 {
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadds_epi16(_mm256_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                    _mm256_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

impl SaturatingHAdd for u64x2 { hop!(saturating_hadd, u64::saturating_add, 0, 1); }
impl SaturatingHAdd for u64x4 { hop!(saturating_hadd, u64::saturating_add, 0, 1, 2, 3); }
impl SaturatingHAdd for u64x8 { hop!(saturating_hadd, u64::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHAdd for u32x4 { hop!(saturating_hadd, u32::saturating_add, 0, 1, 2, 3); }
impl SaturatingHAdd for u32x8 { hop!(saturating_hadd, u32::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHAdd for u32x16 { hop!(saturating_hadd, u32::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHAdd for u16x8 { hop!(saturating_hadd, u16::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHAdd for u16x16 { hop!(saturating_hadd, u16::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHAdd for u16x32 { hop!(saturating_hadd, u16::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl SaturatingHAdd for u8x16 { hop!(saturating_hadd, u8::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHAdd for u8x32 { hop!(saturating_hadd, u8::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl SaturatingHAdd for u8x64 { hop!(saturating_hadd, u8::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }
impl SaturatingHAdd for i64x2 { hop!(saturating_hadd, i64::saturating_add, 0, 1); }
impl SaturatingHAdd for i64x4 { hop!(saturating_hadd, i64::saturating_add, 0, 1, 2, 3); }
impl SaturatingHAdd for i64x8 { hop!(saturating_hadd, i64::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHAdd for i32x4 { hop!(saturating_hadd, i32::saturating_add, 0, 1, 2, 3); }
impl SaturatingHAdd for i32x8 { hop!(saturating_hadd, i32::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHAdd for i32x16 { hop!(saturating_hadd, i32::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
#[cfg(not(target_feature = "ssse3"))]
impl SaturatingHAdd for i16x8 { hop!(saturating_hadd, i16::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "avx2"))]
impl SaturatingHAdd for i16x16 { hop!(saturating_hadd, i16::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHAdd for i16x32 { hop!(saturating_hadd, i16::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl SaturatingHAdd for i8x16 { hop!(saturating_hadd, i8::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHAdd for i8x32 { hop!(saturating_hadd, i8::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl SaturatingHAdd for i8x64 { hop!(saturating_hadd, i8::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }

#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;

    #[test]
    fn saturating_hadd_i8s() {
        assert_eq!(i8s(1).saturating_hadd(i8s(2)), i8s::interleave(2, 4));
        assert_eq!(i8s::interleave(1, 2).saturating_hadd(i8s::interleave(3, 4)), i8s::interleave(3, 7));
        assert_eq!(i8s::interleave(-100, -100).saturating_hadd(i8s::interleave(100, 100)), i8s::interleave(i8::min_value(), i8::max_value()));
    }

    #[test]
    fn saturating_hadd_i16s() {
        assert_eq!(i16s(1).saturating_hadd(i16s(2)), i16s::interleave(2, 4));
        assert_eq!(i16s::interleave(1, 2).saturating_hadd(i16s::interleave(3, 4)), i16s::interleave(3, 7));
        assert_eq!(i16s::interleave(-30000, -30000).saturating_hadd(i16s::interleave(30000, 30000)), i16s::interleave(i16::min_value(), i16::max_value()));
    }

    #[test]
    fn saturating_hadd_i32s() {
        assert_eq!(i32s(1).saturating_hadd(i32s(2)), i32s::interleave(2, 4));
        assert_eq!(i32s::interleave(1, 2).saturating_hadd(i32s::interleave(3, 4)), i32s::interleave(3, 7));
        assert_eq!(i32s::interleave(-2_000_000_000, -2_000_000_000).saturating_hadd(i32s::interleave(2_000_000_000, 2_000_000_000)), i32s::interleave(i32::min_value(), i32::max_value()));
    }

    #[test]
    fn saturating_hadd_i64s() {
        assert_eq!(i64s(1).saturating_hadd(i64s(2)), i64s::interleave(2, 4));
        assert_eq!(i64s::interleave(1, 2).saturating_hadd(i64s::interleave(3, 4)), i64s::interleave(3, 7));
        assert_eq!(i64s::interleave(-9_000_000_000_000_000_000, -9_000_000_000_000_000_000).saturating_hadd(i64s::interleave(9_000_000_000_000_000_000, 9_000_000_000_000_000_000)), i64s::interleave(i64::min_value(), i64::max_value()));
    }

    #[test]
    fn saturating_hadd_u8s() {
        assert_eq!(u8s(1).saturating_hadd(u8s(2)), u8s::interleave(2, 4));
        assert_eq!(u8s::interleave(1, 2).saturating_hadd(u8s::interleave(3, 4)), u8s::interleave(3, 7));
        assert_eq!(u8s(200).saturating_hadd(u8s(200)), u8s(u8::max_value()));
    }

    #[test]
    fn saturating_hadd_u16s() {
        assert_eq!(u16s(1).saturating_hadd(u16s(2)), u16s::interleave(2, 4));
        assert_eq!(u16s::interleave(1, 2).saturating_hadd(u16s::interleave(3, 4)), u16s::interleave(3, 7));
        assert_eq!(u16s(60000).saturating_hadd(u16s(60000)), u16s(u16::max_value()));
    }

    #[test]
    fn saturating_hadd_u32s() {
        assert_eq!(u32s(1).saturating_hadd(u32s(2)), u32s::interleave(2, 4));
        assert_eq!(u32s::interleave(1, 2).saturating_hadd(u32s::interleave(3, 4)), u32s::interleave(3, 7));
        assert_eq!(u32s(4_000_000_000).saturating_hadd(u32s(4_000_000_000)), u32s(u32::max_value()));
    }

    #[test]
    fn saturating_hadd_u64s() {
        assert_eq!(u64s(1).saturating_hadd(u64s(2)), u64s::interleave(2, 4));
        assert_eq!(u64s::interleave(1, 2).saturating_hadd(u64s::interleave(3, 4)), u64s::interleave(3, 7));
        assert_eq!(u64s(18_000_000_000_000_000_000).saturating_hadd(u64s(18_000_000_000_000_000_000)), u64s(u64::max_value()));
    }
}
