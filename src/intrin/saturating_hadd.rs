// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
use intrin::transmute::*;

pub trait PackedSaturatingHadd {
    fn saturating_hadd(&self, other: Self) -> Self;
}

impl PackedSaturatingHadd for i16x8 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadds_epi16(_mm_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                 _mm_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn saturating_hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0).saturating_add(self.extract(1)),
                  other.extract(0).saturating_add(other.extract(1)),
                  self.extract(2).saturating_add(self.extract(3)),
                  other.extract(2).saturating_add(other.extract(3)),
                  self.extract(4).saturating_add(self.extract(5)),
                  other.extract(4).saturating_add(other.extract(5)),
                  self.extract(6).saturating_add(self.extract(7)),
                  other.extract(6).saturating_add(other.extract(7)))
    }
}

impl PackedSaturatingHadd for i32x4 {
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0).saturating_add(self.extract(1)),
                  other.extract(0).saturating_add(other.extract(1)),
                  self.extract(2).saturating_add(self.extract(3)),
                  other.extract(2).saturating_add(other.extract(3)))
    }
}

impl PackedSaturatingHadd for i16x16 {
    #[cfg(target_feature = "avx2")]
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadds_epi16(_mm256_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                    _mm256_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn saturating_hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0).saturating_add(self.extract(1)),
                  other.extract(0).saturating_add(other.extract(1)),
                  self.extract(2).saturating_add(self.extract(3)),
                  other.extract(2).saturating_add(other.extract(3)),
                  self.extract(4).saturating_add(self.extract(5)),
                  other.extract(4).saturating_add(other.extract(5)),
                  self.extract(6).saturating_add(self.extract(7)),
                  other.extract(6).saturating_add(other.extract(7)),
                  self.extract(8).saturating_add(self.extract(9)),
                  other.extract(8).saturating_add(other.extract(9)),
                  self.extract(10).saturating_add(self.extract(11)),
                  other.extract(10).saturating_add(other.extract(11)),
                  self.extract(12).saturating_add(self.extract(13)),
                  other.extract(12).saturating_add(other.extract(13)),
                  self.extract(14).saturating_add(self.extract(15)),
                  other.extract(14).saturating_add(other.extract(15)))
    }
}


impl PackedSaturatingHadd for i32x8 {
    #[inline(always)]
    fn saturating_hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0).saturating_add(self.extract(1)),
                  other.extract(0).saturating_add(other.extract(1)),
                  self.extract(2).saturating_add(self.extract(3)),
                  other.extract(2).saturating_add(other.extract(3)),
                  self.extract(4).saturating_add(self.extract(5)),
                  other.extract(4).saturating_add(other.extract(5)),
                  self.extract(6).saturating_add(self.extract(7)),
                  other.extract(6).saturating_add(other.extract(7)))
    }
}


#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;

    #[test]
    fn saturating_hadd_i16s() {
        assert_eq!(i16s::splat(1).saturating_hadd(i16s::splat(2)), i16s::interleave(2, 4));
        assert_eq!(i16s::interleave(1, 2).saturating_hadd(i16s::interleave(3, 4)), i16s::interleave(3, 7));
        assert_eq!(i16s::interleave(-30000, -30000).saturating_hadd(i16s::interleave(30000, 30000)), i16s::interleave(i16::min_value(), i16::max_value()));
    }

    #[test]
    fn saturating_hadd_i32s() {
        assert_eq!(i32s::splat(1).saturating_hadd(i32s::splat(2)), i32s::interleave(2, 4));
        assert_eq!(i32s::interleave(1, 2).saturating_hadd(i32s::interleave(3, 4)), i32s::interleave(3, 7));
        assert_eq!(i32s::interleave(-2_000_000_000, -2_000_000_000).saturating_hadd(i32s::interleave(2_000_000_000, 2_000_000_000)), i32s::interleave(i32::min_value(), i32::max_value()));
    }
}
