// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
use intrin::transmute::*;
use stdsimd::vendor::*;
use stdsimd::simd::{__m256i, __m128i};
use std::mem::transmute;

pub trait Upcast<T> {
    /// Return two vectors containing elements of the same value, but different
    /// type. The first vector contains the first half of `self`, and the second
    /// vector contains the second half. Both returned vectors are equal in size
    /// to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i8s::halfs(2, 3).upcast(), (i16s::splat(2), i16s::splat(3)))
    /// # }
    /// ```
    fn upcast(self) -> (T, T);
}

impl Upcast<u16x8> for u8x16 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "sse4.1")]
    // fn upcast(self) -> (u16x8, u16x8) {
    //     // Shuffle the vector as i32s for better perf
    //     unsafe { (_mm_cvtepu8_epu16(self), _mm_cvtepu8_epu16(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_u16s())) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (u16x8, u16x8) {
        (u16x8::new(self.extract(0) as u16,
                    self.extract(1) as u16,
                    self.extract(2) as u16,
                    self.extract(3) as u16,
                    self.extract(4) as u16,
                    self.extract(5) as u16,
                    self.extract(6) as u16,
                    self.extract(7) as u16),
         u16x8::new(self.extract(8) as u16,
                    self.extract(9) as u16,
                    self.extract(10) as u16,
                    self.extract(11) as u16,
                    self.extract(12) as u16,
                    self.extract(13) as u16,
                    self.extract(14) as u16,
                    self.extract(15) as u16))
    }
}

impl Upcast<i16x8> for i8x16 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "sse4.1")]
    // fn upcast(self) -> (i16x8, i16x8) {
    //     // Shuffle the vector as i32s for better perf
    //     unsafe { (_mm_cvtepi8_epi16(self), _mm_cvtepi8_epi16(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_i16s())) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (i16x8, i16x8) {
        (i16x8::new(self.extract(0) as i16,
                    self.extract(1) as i16,
                    self.extract(2) as i16,
                    self.extract(3) as i16,
                    self.extract(4) as i16,
                    self.extract(5) as i16,
                    self.extract(6) as i16,
                    self.extract(7) as i16),
         i16x8::new(self.extract(8) as i16,
                    self.extract(9) as i16,
                    self.extract(10) as i16,
                    self.extract(11) as i16,
                    self.extract(12) as i16,
                    self.extract(13) as i16,
                    self.extract(14) as i16,
                    self.extract(15) as i16))
    }
}

impl Upcast<u32x4> for u16x8 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "sse4.1")]
    // fn upcast(self) -> (u32x4, u32x4) {
    //     unsafe { (_mm_cvtepu16_epu32(self), _mm_cvtepu16_epu32(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_u32s())) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (u32x4, u32x4) {
        (u32x4::new(self.extract(0) as u32,
                    self.extract(1) as u32,
                    self.extract(2) as u32,
                    self.extract(3) as u32),
         u32x4::new(self.extract(4) as u32,
                    self.extract(5) as u32,
                    self.extract(6) as u32,
                    self.extract(7) as u32))
    }
}

impl Upcast<i32x4> for i16x8 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "sse4.1")]
    // fn upcast(self) -> (i32x4, i32x4) {
    //     unsafe { (_mm_cvtepi16_epi32(self), _mm_cvtepi16_epi32(_mm_shuffle_epi32(self.be_i32s(), 0x0E))) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (i32x4, i32x4) {
        (i32x4::new(self.extract(0) as i32,
                    self.extract(1) as i32,
                    self.extract(2) as i32,
                    self.extract(3) as i32),
         i32x4::new(self.extract(4) as i32,
                    self.extract(5) as i32,
                    self.extract(6) as i32,
                    self.extract(7) as i32))
    }
}

impl Upcast<u16x16> for u8x32 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "avx2")]
    // fn upcast(self) -> (u16x16, u16x16) {
    //     unsafe { (_mm256_cvtepu8_epu16(self), _mm256_cvtepu8_epu16(_mm256_permute4x64_epi64(self.be_i64s(), 0x0E).be_u16s())) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (u16x16, u16x16) {
        (u16x16::new(self.extract(0) as u16,
                     self.extract(1) as u16,
                     self.extract(2) as u16,
                     self.extract(3) as u16,
                     self.extract(4) as u16,
                     self.extract(5) as u16,
                     self.extract(6) as u16,
                     self.extract(7) as u16,
                     self.extract(8) as u16,
                     self.extract(9) as u16,
                     self.extract(10) as u16,
                     self.extract(11) as u16,
                     self.extract(12) as u16,
                     self.extract(13) as u16,
                     self.extract(14) as u16,
                     self.extract(15) as u16),
         u16x16::new(self.extract(16) as u16,
                     self.extract(17) as u16,
                     self.extract(18) as u16,
                     self.extract(19) as u16,
                     self.extract(20) as u16,
                     self.extract(21) as u16,
                     self.extract(22) as u16,
                     self.extract(23) as u16,
                     self.extract(24) as u16,
                     self.extract(25) as u16,
                     self.extract(26) as u16,
                     self.extract(27) as u16,
                     self.extract(28) as u16,
                     self.extract(29) as u16,
                     self.extract(30) as u16,
                     self.extract(31) as u16))
    }
}

impl Upcast<i16x16> for i8x32 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (i16x16, i16x16) {
        unsafe {
            (_mm256_cvtepi8_epi16(
                transmute::<__m128i, i8x16>(
                    _mm256_castsi256_si128(
                        transmute::<Self, __m256i>(self)))),
             _mm256_cvtepi8_epi16(
                 transmute::<__m128i, i8x16>(
                     _mm256_castsi256_si128(
                         transmute::<i64x4, __m256i>(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (i16x16, i16x16) {
        (i16x16::new(self.extract(0) as i16,
                     self.extract(1) as i16,
                     self.extract(2) as i16,
                     self.extract(3) as i16,
                     self.extract(4) as i16,
                     self.extract(5) as i16,
                     self.extract(6) as i16,
                     self.extract(7) as i16,
                     self.extract(8) as i16,
                     self.extract(9) as i16,
                     self.extract(10) as i16,
                     self.extract(11) as i16,
                     self.extract(12) as i16,
                     self.extract(13) as i16,
                     self.extract(14) as i16,
                     self.extract(15) as i16),
         i16x16::new(self.extract(16) as i16,
                     self.extract(17) as i16,
                     self.extract(18) as i16,
                     self.extract(19) as i16,
                     self.extract(20) as i16,
                     self.extract(21) as i16,
                     self.extract(22) as i16,
                     self.extract(23) as i16,
                     self.extract(24) as i16,
                     self.extract(25) as i16,
                     self.extract(26) as i16,
                     self.extract(27) as i16,
                     self.extract(28) as i16,
                     self.extract(29) as i16,
                     self.extract(30) as i16,
                     self.extract(31) as i16))
    }
}

impl Upcast<u32x8> for u16x16 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "avx2")]
    // fn upcast(self) -> (u32x8, u32x8) {
    //     unsafe { (_mm256_cvtepu16_epu32(self), _mm256_cvtepu16_epu32(_mm256_permute4x64_epi64(self.be_i64s(), 0x0E).be_u32s())) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (u32x8, u32x8) {
        (u32x8::new(self.extract(0) as u32,
                    self.extract(1) as u32,
                    self.extract(2) as u32,
                    self.extract(3) as u32,
                    self.extract(4) as u32,
                    self.extract(5) as u32,
                    self.extract(6) as u32,
                    self.extract(7) as u32),
         u32x8::new(self.extract(8) as u32,
                    self.extract(9) as u32,
                    self.extract(10) as u32,
                    self.extract(11) as u32,
                    self.extract(12) as u32,
                    self.extract(13) as u32,
                    self.extract(14) as u32,
                    self.extract(15) as u32))

    }
}

impl Upcast<i32x8> for i16x16 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (i32x8, i32x8) {
        unsafe {
            (_mm256_cvtepi16_epi32(
                transmute::<__m128i, i16x8>(
                    _mm256_castsi256_si128(
                        transmute::<Self, __m256i>(self)))),
             _mm256_cvtepi16_epi32(
                 transmute::<__m128i, i16x8>(
                     _mm256_castsi256_si128(
                         transmute::<i64x4, __m256i>(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (i32x8, i32x8) {
        (i32x8::new(self.extract(0) as i32,
                    self.extract(1) as i32,
                    self.extract(2) as i32,
                    self.extract(3) as i32,
                    self.extract(4) as i32,
                    self.extract(5) as i32,
                    self.extract(6) as i32,
                    self.extract(7) as i32),
         i32x8::new(self.extract(8) as i32,
                    self.extract(9) as i32,
                    self.extract(10) as i32,
                    self.extract(11) as i32,
                    self.extract(12) as i32,
                    self.extract(13) as i32,
                    self.extract(14) as i32,
                    self.extract(15) as i32))
    }
}

impl Upcast<f64x2> for f32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn upcast(self) -> (f64x2, f64x2) {
        // Shuffle the vector as i32s for better perf
        unsafe { (_mm_cvtps_pd(self), _mm_cvtps_pd(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_f32s_unchecked())) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn upcast(self) -> (f64x2, f64x2) {
        (f64x2::new(self.extract(0) as f64,
                    self.extract(1) as f64),
         f64x2::new(self.extract(2) as f64,
                    self.extract(3) as f64))
    }
}

impl Upcast<f64x2> for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn upcast(self) -> (f64x2, f64x2) {
        unsafe { (_mm_cvtepi32_pd(self), _mm_cvtepi32_pd(_mm_shuffle_epi32(self, 0x0E))) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn upcast(self) -> (f64x2, f64x2) {
        (f64x2::new(self.extract(0) as f64,
                    self.extract(1) as f64),
         f64x2::new(self.extract(2) as f64,
                    self.extract(3) as f64))
    }
}

impl Upcast<i64x2> for i32x4 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "sse4.1")]
    // fn upcast(self) -> (i64x2, i64x2) {
    //     unsafe { (_mm_cvtepi32_epi64(self), _mm_cvtepi32_epi64(_mm_shuffle_epi32(0x0E))) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (i64x2, i64x2) {
        (i64x2::new(self.extract(0) as i64,
                    self.extract(1) as i64),
         i64x2::new(self.extract(2) as i64,
                    self.extract(3) as i64))
    }
}

impl Upcast<u64x2> for u32x4 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "sse4.1")]
    // fn upcast(self) -> (u64x2, u64x2) {
    //     unsafe { (_mm_cvtepu32_epu64(self), _mm_cvtepu32_epu64(_mm_shuffle_epu32(0x0E))) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (u64x2, u64x2) {
        (u64x2::new(self.extract(0) as u64,
                    self.extract(1) as u64),
         u64x2::new(self.extract(2) as u64,
                    self.extract(3) as u64))
    }
}

impl Upcast<f64x4> for f32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (f64x4, f64x4) {
        // Shuffle the vector as i32s for better perf
        unsafe {
            (_mm256_cvtps_pd(
                transmute::<__m128i, f32x4>(
                    _mm256_castsi256_si128(
                        transmute::<Self, __m256i>(self)))),
             _mm256_cvtps_pd(
                 transmute::<__m128i, f32x4>(
                     _mm256_castsi256_si128(
                         transmute::<i64x4, __m256i>(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (f64x4, f64x4) {
        (f64x4::new(self.extract(0) as f64,
                    self.extract(1) as f64,
                    self.extract(2) as f64,
                    self.extract(3) as f64),
         f64x4::new(self.extract(4) as f64,
                    self.extract(5) as f64,
                    self.extract(6) as f64,
                    self.extract(7) as f64))
    }
}

impl Upcast<f64x4> for i32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (f64x4, f64x4) {
        unsafe {
            (_mm256_cvtepi32_pd(
                transmute::<__m128i, i32x4>(
                    _mm256_castsi256_si128(
                        transmute::<Self, __m256i>(self)))),
             _mm256_cvtepi32_pd(
                 transmute::<__m128i, i32x4>(
                     _mm256_castsi256_si128(
                         transmute::<i64x4, __m256i>(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (f64x4, f64x4) {
        (f64x4::new(self.extract(0) as f64,
                    self.extract(1) as f64,
                    self.extract(2) as f64,
                    self.extract(3) as f64),
         f64x4::new(self.extract(4) as f64,
                    self.extract(5) as f64,
                    self.extract(6) as f64,
                    self.extract(7) as f64))
    }
}

impl Upcast<i64x4> for i32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (i64x4, i64x4) {
        unsafe {
            (_mm256_cvtepi32_epi64(
                transmute::<__m128i, i32x4>(
                    _mm256_castsi256_si128(
                        transmute::<Self, __m256i>(self)))),
             _mm256_cvtepi32_epi64(
                 transmute::<__m128i, i32x4>(
                     _mm256_castsi256_si128(
                         transmute::<i64x4, __m256i>(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (i64x4, i64x4) {
        (i64x4::new(self.extract(0) as i64,
                    self.extract(1) as i64,
                    self.extract(2) as i64,
                    self.extract(3) as i64),
         i64x4::new(self.extract(4) as i64,
                    self.extract(5) as i64,
                    self.extract(6) as i64,
                    self.extract(7) as i64))
    }
}

impl Upcast<u64x4> for u32x8 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "avx2")]
    // fn upcast(self) -> (u64x4, u64x4) {
    //     unsafe { (_mm256_cvtepu32_epu64(self), _mm256_cvtepu32_epu64(_mm256_shuffle_epu32(0x0E))) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (u64x4, u64x4) {
        (u64x4::new(self.extract(0) as u64,
                    self.extract(1) as u64,
                    self.extract(2) as u64,
                    self.extract(3) as u64),
         u64x4::new(self.extract(4) as u64,
                    self.extract(5) as u64,
                    self.extract(6) as u64,
                    self.extract(7) as u64))
    }
}

impl Upcast<f64x8> for f32x16 {
    #[inline(always)]
    fn upcast(self) -> (f64x8, f64x8) {
        (f64x8::new(self.extract(0) as f64,
                    self.extract(1) as f64,
                    self.extract(2) as f64,
                    self.extract(3) as f64,
                    self.extract(4) as f64,
                    self.extract(5) as f64,
                    self.extract(6) as f64,
                    self.extract(7) as f64),
         f64x8::new(self.extract(8) as f64,
                    self.extract(9) as f64,
                    self.extract(10) as f64,
                    self.extract(11) as f64,
                    self.extract(12) as f64,
                    self.extract(13) as f64,
                    self.extract(14) as f64,
                    self.extract(15) as f64))
    }
}

impl Upcast<f64x8> for i32x16 {
    #[inline(always)]
    fn upcast(self) -> (f64x8, f64x8) {
        (f64x8::new(self.extract(0) as f64,
                    self.extract(1) as f64,
                    self.extract(2) as f64,
                    self.extract(3) as f64,
                    self.extract(4) as f64,
                    self.extract(5) as f64,
                    self.extract(6) as f64,
                    self.extract(7) as f64),
         f64x8::new(self.extract(8) as f64,
                    self.extract(9) as f64,
                    self.extract(10) as f64,
                    self.extract(11) as f64,
                    self.extract(12) as f64,
                    self.extract(13) as f64,
                    self.extract(14) as f64,
                    self.extract(15) as f64))
    }
}

impl Upcast<i64x8> for i32x16 {
    #[inline(always)]
    fn upcast(self) -> (i64x8, i64x8) {
        (i64x8::new(self.extract(0) as i64,
                    self.extract(1) as i64,
                    self.extract(2) as i64,
                    self.extract(3) as i64,
                    self.extract(4) as i64,
                    self.extract(5) as i64,
                    self.extract(6) as i64,
                    self.extract(7) as i64),
         i64x8::new(self.extract(8) as i64,
                    self.extract(9) as i64,
                    self.extract(10) as i64,
                    self.extract(11) as i64,
                    self.extract(12) as i64,
                    self.extract(13) as i64,
                    self.extract(14) as i64,
                    self.extract(15) as i64))
    }
}

impl Upcast<u64x8> for u32x16 {
    #[inline(always)]
    fn upcast(self) -> (u64x8, u64x8) {
        (u64x8::new(self.extract(0) as u64,
                    self.extract(1) as u64,
                    self.extract(2) as u64,
                    self.extract(3) as u64,
                    self.extract(4) as u64,
                    self.extract(5) as u64,
                    self.extract(6) as u64,
                    self.extract(7) as u64),
         u64x8::new(self.extract(8) as u64,
                    self.extract(9) as u64,
                    self.extract(10) as u64,
                    self.extract(11) as u64,
                    self.extract(12) as u64,
                    self.extract(13) as u64,
                    self.extract(14) as u64,
                    self.extract(15) as u64))
    }
}

#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;

    #[test]
    fn upcast_i8s() {
        assert_eq!(i8s::interleave(1, 2).upcast().0, i16s::interleave(1, 2));
        assert_eq!(i8s::interleave(1, 2).upcast().1, i16s::interleave(1, 2));
    }

    #[test]
    fn upcast_u8s() {
        assert_eq!(u8s::interleave(1, 2).upcast().0, u16s::interleave(1, 2));
        assert_eq!(u8s::interleave(1, 2).upcast().1, u16s::interleave(1, 2));
    }

    #[test]
    fn upcast_i16s() {
        assert_eq!(i16s::interleave(1, 2).upcast().0, i32s::interleave(1, 2));
        assert_eq!(i16s::interleave(1, 2).upcast().1, i32s::interleave(1, 2));
    }

    #[test]
    fn upcast_u16s() {
        assert_eq!(u16s::interleave(1, 2).upcast().0, u32s::interleave(1, 2));
        assert_eq!(u16s::interleave(1, 2).upcast().1, u32s::interleave(1, 2));
    }

    #[test]
    fn upcast_i32s_i64s() {
        // TODO: Fix ugliness
        assert_eq!(Upcast::<i64s>::upcast(i32s::interleave(1, 2)).0, i64s::interleave(1, 2));
        assert_eq!(Upcast::<i64s>::upcast(i32s::interleave(1, 2)).1, i64s::interleave(1, 2));
    }

    #[test]
    fn upcast_i32s_f64s() {
        // TODO: Fix ugliness
        assert_eq!(Upcast::<f64s>::upcast(i32s::interleave(1, 2)).0, f64s::interleave(1.0, 2.0));
        assert_eq!(Upcast::<f64s>::upcast(i32s::interleave(1, 2)).1, f64s::interleave(1.0, 2.0));
    }

    #[test]
    fn upcast_u32s() {
        assert_eq!(u32s::interleave(1, 2).upcast().0, u64s::interleave(1, 2));
        assert_eq!(u32s::interleave(1, 2).upcast().1, u64s::interleave(1, 2));
    }

    #[test]
    fn upcast_f32s() {
        assert_eq!(f32s::interleave(1.0, 2.0).upcast(), (f64s::interleave(1.0, 2.0), f64s::interleave(1.0, 2.0)));
        //     assert_eq!(f32s::interleave(1.0, 2.0).upcast().0, f64s::interleave(1.0, 2.0));
        // assert_eq!(f32s::interleave(1.0, 2.0).upcast().1, f64s::interleave(1.0, 2.0));
    }
}
