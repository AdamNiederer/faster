// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
use crate::intrin::transmute::*;
use crate::intrin::saturating_hsub::*;



#[cfg(target_feature = "ssse3")]
impl SaturatingHSub for i16x8 {
    #[inline(always)]
    fn saturating_hsub(&self, other: Self) -> Self {
        unsafe { _mm_hsubs_epi16(_mm_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                 _mm_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

#[cfg(target_feature = "avx2")]
impl SaturatingHSub for i16x16 {
    #[inline(always)]
    fn saturating_hsub(&self, other: Self) -> Self {
        unsafe { _mm256_hsubs_epi16(_mm256_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                    _mm256_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

impl SaturatingHSub for u64x2 { hop!(saturating_hsub, u64::saturating_sub, 0, 1); }
impl SaturatingHSub for u64x4 { hop!(saturating_hsub, u64::saturating_sub, 0, 1, 2, 3); }
impl SaturatingHSub for u64x8 { hop!(saturating_hsub, u64::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHSub for u32x4 { hop!(saturating_hsub, u32::saturating_sub, 0, 1, 2, 3); }
impl SaturatingHSub for u32x8 { hop!(saturating_hsub, u32::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHSub for u32x16 { hop!(saturating_hsub, u32::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHSub for u16x8 { hop!(saturating_hsub, u16::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHSub for u16x16 { hop!(saturating_hsub, u16::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHSub for u16x32 { hop!(saturating_hsub, u16::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl SaturatingHSub for u8x16 { hop!(saturating_hsub, u8::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHSub for u8x32 { hop!(saturating_hsub, u8::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl SaturatingHSub for u8x64 { hop!(saturating_hsub, u8::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }
impl SaturatingHSub for i64x2 { hop!(saturating_hsub, i64::saturating_sub, 0, 1); }
impl SaturatingHSub for i64x4 { hop!(saturating_hsub, i64::saturating_sub, 0, 1, 2, 3); }
impl SaturatingHSub for i64x8 { hop!(saturating_hsub, i64::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHSub for i32x4 { hop!(saturating_hsub, i32::saturating_sub, 0, 1, 2, 3); }
impl SaturatingHSub for i32x8 { hop!(saturating_hsub, i32::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl SaturatingHSub for i32x16 { hop!(saturating_hsub, i32::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
#[cfg(not(target_feature = "ssse3"))]
impl SaturatingHSub for i16x8 { hop!(saturating_hsub, i16::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "avx2"))]
impl SaturatingHSub for i16x16 { hop!(saturating_hsub, i16::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHSub for i16x32 { hop!(saturating_hsub, i16::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl SaturatingHSub for i8x16 { hop!(saturating_hsub, i8::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl SaturatingHSub for i8x32 { hop!(saturating_hsub, i8::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl SaturatingHSub for i8x64 { hop!(saturating_hsub, i8::saturating_sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }

#[cfg(test)]
mod tests {
    use crate::vecs::*;
    use crate::intrin::*;

    #[test]
    fn saturating_hsub_i8s() {
        assert_eq!(i8s(1).saturating_hsub(i8s(2)), i8s::interleave(0, 0));
        assert_eq!(i8s::interleave(1, 2).saturating_hsub(i8s::interleave(3, 4)), i8s::interleave(-1, -1));
        assert_eq!(i8s::interleave(-100, 100).saturating_hsub(i8s::interleave(100, -100)), i8s::interleave(i8::min_value(), i8::max_value()));
    }

    #[test]
    fn saturating_hsub_i16s() {
        assert_eq!(i16s(1).saturating_hsub(i16s(2)), i16s::interleave(0, 0));
        assert_eq!(i16s::interleave(1, 2).saturating_hsub(i16s::interleave(3, 4)), i16s::interleave(-1, -1));
        assert_eq!(i16s::interleave(-30000, 30000).saturating_hsub(i16s::interleave(30000, -30000)), i16s::interleave(i16::min_value(), i16::max_value()));
    }

    #[test]
    fn saturating_hsub_i32s() {
        assert_eq!(i32s(1).saturating_hsub(i32s(2)), i32s::interleave(0, 0));
        assert_eq!(i32s::interleave(1, 2).saturating_hsub(i32s::interleave(3, 4)), i32s::interleave(-1, -1));
        assert_eq!(i32s::interleave(-2_000_000_000, 2_000_000_000).saturating_hsub(i32s::interleave(2_000_000_000, -2_000_000_000)), i32s::interleave(i32::min_value(), i32::max_value()));
    }

    #[test]
    fn saturating_hsub_i64s() {
        assert_eq!(i64s(1).saturating_hsub(i64s(2)), i64s::interleave(0, 0));
        assert_eq!(i64s::interleave(1, 2).saturating_hsub(i64s::interleave(3, 4)), i64s::interleave(-1, -1));
        assert_eq!(i64s::interleave(-9_000_000_000_000_000_000, 9_000_000_000_000_000_000).saturating_hsub(i64s::interleave(9_000_000_000_000_000_000, -9_000_000_000_000_000_000)), i64s::interleave(i64::min_value(), i64::max_value()));
    }

    #[test]
    fn saturating_hsub_u8s() {
        assert_eq!(u8s(1).saturating_hsub(u8s(2)), u8s::interleave(0, 0));
        assert_eq!(u8s::interleave(1, 2).saturating_hsub(u8s::interleave(3, 4)), u8s::interleave(0, 0));
        assert_eq!(u8s::interleave(2, 1).saturating_hsub(u8s::interleave(4, 3)), u8s::interleave(1, 1));
    }

    #[test]
    fn saturating_hsub_u16s() {
        assert_eq!(u16s(1).saturating_hsub(u16s(2)), u16s::interleave(0, 0));
        assert_eq!(u16s::interleave(1, 2).saturating_hsub(u16s::interleave(3, 4)), u16s::interleave(0, 0));
        assert_eq!(u16s::interleave(2, 1).saturating_hsub(u16s::interleave(4, 3)), u16s::interleave(1, 1));
    }

    #[test]
    fn saturating_hsub_u32s() {
        assert_eq!(u32s(1).saturating_hsub(u32s(2)), u32s::interleave(0, 0));
        assert_eq!(u32s::interleave(1, 2).saturating_hsub(u32s::interleave(3, 4)), u32s::interleave(0, 0));
        assert_eq!(u32s::interleave(2, 1).saturating_hsub(u32s::interleave(4, 3)), u32s::interleave(1, 1));
    }

    #[test]
    fn saturating_hsub_u64s() {
        assert_eq!(u64s(1).saturating_hsub(u64s(2)), u64s::interleave(0, 0));
        assert_eq!(u64s::interleave(1, 2).saturating_hsub(u64s::interleave(3, 4)), u64s::interleave(0, 0));
        assert_eq!(u64s::interleave(2, 1).saturating_hsub(u64s::interleave(4, 3)), u64s::interleave(1, 1));
    }
}
