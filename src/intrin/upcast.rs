// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::simd::*;
use stdsimd::vendor::*;
use intrin::transmute::*;

pub trait Upcast<T> {
    fn upcast(self) -> (T, T);
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
            (_mm256_cvtps_pd(_mm256_castsi256_si128(self.be_i8s()).be_f32s_unchecked()),
             _mm256_cvtps_pd(_mm256_castsi256_si128(_mm256_shuffle_epi32(self.be_i32s(), 0x0E).be_i8s()).be_f32s_unchecked())) }
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
            (_mm256_cvtepi32_pd(_mm256_castsi256_si128(self.be_i8s()).be_i32s()),
             _mm256_cvtepi32_pd(_mm256_castsi256_si128(_mm256_shuffle_epi32(self, 0x0E).be_i8s()).be_i32s()))
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
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "avx2")]
    // fn upcast(self) -> (i64x4, i64x4) {
    //     unsafe { (_mm256_cvtepi32_epi64(self), _mm256_cvtepi32_epi64(_mm256_shuffle_epi32(0x0E))) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "avx2"))]
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
