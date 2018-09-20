// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::intrin::downcast::*;
use crate::intrin::transmute::*;
use crate::core::mem::transmute;

impl Downcast<i16x8> for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> i16x8 {
        optimized!();
        unsafe { _mm_packs_epi32(self.be_i32s(), other.be_i32s()).be_i16s() }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> i16x8 {
        fallback!();
        i16x8::new(self.extract(0).min(0x00007FFF).max(-0x00008000) as i16,
                   self.extract(1).min(0x00007FFF).max(-0x00008000) as i16,
                   self.extract(2).min(0x00007FFF).max(-0x00008000) as i16,
                   self.extract(3).min(0x00007FFF).max(-0x00008000) as i16,
                   other.extract(0).min(0x00007FFF).max(-0x00008000) as i16,
                   other.extract(1).min(0x00007FFF).max(-0x00008000) as i16,
                   other.extract(2).min(0x00007FFF).max(-0x00008000) as i16,
                   other.extract(3).min(0x00007FFF).max(-0x00008000) as i16)
    }
}

impl Downcast<i32x4> for i64x2 {
    #[inline(always)]
    fn saturating_downcast(self, other: Self) -> i32x4 {
        fallback!();
        i32x4::new(self.extract(0).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   self.extract(1).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   other.extract(0).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   other.extract(1).min(0x7FFFFFFF).max(-0x80000000) as i32)
    }
}

impl Downcast<u32x4> for u64x2 {
    #[inline(always)]
    fn saturating_downcast(self, other: Self) -> u32x4 {
        fallback!();
        u32x4::new(self.extract(0).min(0xFFFFFFFF) as u32,
                   self.extract(1).min(0x7FFFFFFF) as u32,
                   other.extract(0).min(0x7FFFFFFF) as u32,
                   other.extract(1).min(0x7FFFFFFF) as u32)
    }
}

impl Downcast<f32x4> for f64x2 {
    #[inline(always)]
    fn saturating_downcast(self, other: Self) -> f32x4 {
        fallback!();
        f32x4::new(self.extract(0) as f32,
                   self.extract(1) as f32,
                   other.extract(0) as f32,
                   other.extract(1) as f32)
    }
}

impl Downcast<i8x16> for i16x8 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> i8x16 {
        optimized!();
        unsafe { _mm_packs_epi16(self.be_i16s(), other.be_i16s()).be_i8s() }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> i8x16 {
        fallback!();
        i8x16::new(self.extract(0).min(0x007F).max(-0x0080) as i8,
                   self.extract(1).min(0x007F).max(-0x0080) as i8,
                   self.extract(2).min(0x007F).max(-0x0080) as i8,
                   self.extract(3).min(0x007F).max(-0x0080) as i8,
                   self.extract(4).min(0x007F).max(-0x0080) as i8,
                   self.extract(5).min(0x007F).max(-0x0080) as i8,
                   self.extract(6).min(0x007F).max(-0x0080) as i8,
                   self.extract(7).min(0x007F).max(-0x0080) as i8,
                   other.extract(0).min(0x007F).max(-0x0080) as i8,
                   other.extract(1).min(0x007F).max(-0x0080) as i8,
                   other.extract(2).min(0x007F).max(-0x0080) as i8,
                   other.extract(3).min(0x007F).max(-0x0080) as i8,
                   other.extract(4).min(0x007F).max(-0x0080) as i8,
                   other.extract(5).min(0x007F).max(-0x0080) as i8,
                   other.extract(6).min(0x007F).max(-0x0080) as i8,
                   other.extract(7).min(0x007F).max(-0x0080) as i8)
    }
}

impl Downcast<u16x8> for u32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn saturating_downcast(self, other: Self) -> u16x8 {
        optimized!();
        unsafe { transmute(_mm_packus_epi32(transmute(self), transmute(other))) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn saturating_downcast(self, other: Self) -> u16x8 {
        fallback!();
        u16x8::new(self.extract(0).min(0x0000FFFF) as u16,
                   self.extract(1).min(0x0000FFFF) as u16,
                   self.extract(2).min(0x0000FFFF) as u16,
                   self.extract(3).min(0x0000FFFF) as u16,
                   other.extract(0).min(0x0000FFFF) as u16,
                   other.extract(1).min(0x0000FFFF) as u16,
                   other.extract(2).min(0x0000FFFF) as u16,
                   other.extract(3).min(0x0000FFFF) as u16)
    }
}

impl Downcast<u8x16> for u16x8 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> u8x16 {
        optimized!();
        unsafe { _mm_packus_epi16(self.be_i16s(), other.be_i16s()).be_u8s() }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> u8x16 {
        fallback!();
        u8x16::new(self.extract(0).min(0x00FF) as u8,
                   self.extract(1).min(0x00FF) as u8,
                   self.extract(2).min(0x00FF) as u8,
                   self.extract(3).min(0x00FF) as u8,
                   self.extract(4).min(0x00FF) as u8,
                   self.extract(5).min(0x00FF) as u8,
                   self.extract(6).min(0x00FF) as u8,
                   self.extract(7).min(0x00FF) as u8,
                   other.extract(0).min(0x00FF) as u8,
                   other.extract(1).min(0x00FF) as u8,
                   other.extract(2).min(0x00FF) as u8,
                   other.extract(3).min(0x00FF) as u8,
                   other.extract(4).min(0x00FF) as u8,
                   other.extract(5).min(0x00FF) as u8,
                   other.extract(6).min(0x00FF) as u8,
                   other.extract(7).min(0x00FF) as u8)
    }
}

impl Downcast<i16x16> for i32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn saturating_downcast(self, other: Self) -> i16x16 {
        optimized!();
        unsafe { _mm256_packs_epi32(self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn saturating_downcast(self, other: Self) -> i16x16 {
        fallback!();
        i16x16::new(self.extract(0).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(1).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(2).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(3).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(4).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(5).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(6).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(7).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(0).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(1).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(2).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(3).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(4).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(5).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(6).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(7).min(0x00007FFF).max(-0x00008000) as i16)
    }
}

impl Downcast<i8x32> for i16x16 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn saturating_downcast(self, other: Self) -> i8x32 {
        optimized!();
        unsafe { _mm256_packs_epi16(self.be_i16s(), other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn saturating_downcast(self, other: Self) -> i8x32 {
        fallback!();
        i8x32::new(self.extract(0).min(0x007F).max(-0x0080) as i8,
                   self.extract(1).min(0x007F).max(-0x0080) as i8,
                   self.extract(2).min(0x007F).max(-0x0080) as i8,
                   self.extract(3).min(0x007F).max(-0x0080) as i8,
                   self.extract(4).min(0x007F).max(-0x0080) as i8,
                   self.extract(5).min(0x007F).max(-0x0080) as i8,
                   self.extract(6).min(0x007F).max(-0x0080) as i8,
                   self.extract(7).min(0x007F).max(-0x0080) as i8,
                   self.extract(8).min(0x007F).max(-0x0080) as i8,
                   self.extract(9).min(0x007F).max(-0x0080) as i8,
                   self.extract(10).min(0x007F).max(-0x0080) as i8,
                   self.extract(11).min(0x007F).max(-0x0080) as i8,
                   self.extract(12).min(0x007F).max(-0x0080) as i8,
                   self.extract(13).min(0x007F).max(-0x0080) as i8,
                   self.extract(14).min(0x007F).max(-0x0080) as i8,
                   self.extract(15).min(0x007F).max(-0x0080) as i8,
                   other.extract(0).min(0x007F).max(-0x0080) as i8,
                   other.extract(1).min(0x007F).max(-0x0080) as i8,
                   other.extract(2).min(0x007F).max(-0x0080) as i8,
                   other.extract(3).min(0x007F).max(-0x0080) as i8,
                   other.extract(4).min(0x007F).max(-0x0080) as i8,
                   other.extract(5).min(0x007F).max(-0x0080) as i8,
                   other.extract(6).min(0x007F).max(-0x0080) as i8,
                   other.extract(7).min(0x007F).max(-0x0080) as i8,
                   other.extract(8).min(0x007F).max(-0x0080) as i8,
                   other.extract(9).min(0x007F).max(-0x0080) as i8,
                   other.extract(10).min(0x007F).max(-0x0080) as i8,
                   other.extract(11).min(0x007F).max(-0x0080) as i8,
                   other.extract(12).min(0x007F).max(-0x0080) as i8,
                   other.extract(13).min(0x007F).max(-0x0080) as i8,
                   other.extract(14).min(0x007F).max(-0x0080) as i8,
                   other.extract(15).min(0x007F).max(-0x0080) as i8)
    }
}

impl Downcast<u16x16> for u32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn saturating_downcast(self, other: Self) -> u16x16 {
        optimized!();
        unsafe { transmute(_mm256_packus_epi32(transmute(self), transmute(other))) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn saturating_downcast(self, other: Self) -> u16x16 {
        fallback!();
        u16x16::new(self.extract(0).min(0x0000FFFF) as u16,
                    self.extract(1).min(0x0000FFFF) as u16,
                    self.extract(2).min(0x0000FFFF) as u16,
                    self.extract(3).min(0x0000FFFF) as u16,
                    self.extract(4).min(0x0000FFFF) as u16,
                    self.extract(5).min(0x0000FFFF) as u16,
                    self.extract(6).min(0x0000FFFF) as u16,
                    self.extract(7).min(0x0000FFFF) as u16,
                    other.extract(0).min(0x0000FFFF) as u16,
                    other.extract(1).min(0x0000FFFF) as u16,
                    other.extract(2).min(0x0000FFFF) as u16,
                    other.extract(3).min(0x0000FFFF) as u16,
                    other.extract(4).min(0x0000FFFF) as u16,
                    other.extract(5).min(0x0000FFFF) as u16,
                    other.extract(6).min(0x0000FFFF) as u16,
                    other.extract(7).min(0x0000FFFF) as u16)
    }
}

impl Downcast<u8x32> for u16x16 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn saturating_downcast(self, other: Self) -> u8x32 {
        optimized!();
        unsafe { _mm256_packus_epi16(self.be_i16s(), other.be_i16s()).be_u8s() }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn saturating_downcast(self, other: Self) -> u8x32 {
        fallback!();
        u8x32::new(self.extract(0).min(0x00FF) as u8,
                   self.extract(1).min(0x00FF) as u8,
                   self.extract(2).min(0x00FF) as u8,
                   self.extract(3).min(0x00FF) as u8,
                   self.extract(4).min(0x00FF) as u8,
                   self.extract(5).min(0x00FF) as u8,
                   self.extract(6).min(0x00FF) as u8,
                   self.extract(7).min(0x00FF) as u8,
                   self.extract(8).min(0x00FF) as u8,
                   self.extract(9).min(0x00FF) as u8,
                   self.extract(10).min(0x00FF) as u8,
                   self.extract(11).min(0x00FF) as u8,
                   self.extract(12).min(0x00FF) as u8,
                   self.extract(13).min(0x00FF) as u8,
                   self.extract(14).min(0x00FF) as u8,
                   self.extract(15).min(0x00FF) as u8,
                   other.extract(0).min(0x00FF) as u8,
                   other.extract(1).min(0x00FF) as u8,
                   other.extract(2).min(0x00FF) as u8,
                   other.extract(3).min(0x00FF) as u8,
                   other.extract(4).min(0x00FF) as u8,
                   other.extract(5).min(0x00FF) as u8,
                   other.extract(6).min(0x00FF) as u8,
                   other.extract(7).min(0x00FF) as u8,
                   other.extract(8).min(0x00FF) as u8,
                   other.extract(9).min(0x00FF) as u8,
                   other.extract(10).min(0x00FF) as u8,
                   other.extract(11).min(0x00FF) as u8,
                   other.extract(12).min(0x00FF) as u8,
                   other.extract(13).min(0x00FF) as u8,
                   other.extract(14).min(0x00FF) as u8,
                   other.extract(15).min(0x00FF) as u8)
    }
}

impl Downcast<i32x8> for i64x4 {
    #[inline(always)]
    fn saturating_downcast(self, other: Self) -> i32x8 {
        fallback!();
        i32x8::new(self.extract(0).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   self.extract(1).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   self.extract(2).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   self.extract(3).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   other.extract(0).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   other.extract(1).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   other.extract(2).min(0x7FFFFFFF).max(-0x80000000) as i32,
                   other.extract(3).min(0x7FFFFFFF).max(-0x80000000) as i32)
    }
}

impl Downcast<u32x8> for u64x4 {
    #[inline(always)]
    fn saturating_downcast(self, other: Self) -> u32x8 {
        fallback!();
        u32x8::new(self.extract(0).min(0xFFFFFFFF) as u32,
                   self.extract(1).min(0x7FFFFFFF) as u32,
                   self.extract(2).min(0xFFFFFFFF) as u32,
                   self.extract(3).min(0x7FFFFFFF) as u32,
                   other.extract(0).min(0x7FFFFFFF) as u32,
                   other.extract(1).min(0x7FFFFFFF) as u32,
                   other.extract(2).min(0x7FFFFFFF) as u32,
                   other.extract(3).min(0x7FFFFFFF) as u32)
    }
}

impl Downcast<f32x8> for f64x4 {
    #[inline(always)]
    fn saturating_downcast(self, other: Self) -> f32x8 {
        fallback!();
        f32x8::new(self.extract(0) as f32,
                   self.extract(1) as f32,
                   self.extract(2) as f32,
                   self.extract(3) as f32,
                   other.extract(0) as f32,
                   other.extract(1) as f32,
                   other.extract(2) as f32,
                   other.extract(3) as f32)
    }
}
