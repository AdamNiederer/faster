// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::arch::current::vecs::*;
use crate::vecs::*;
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
