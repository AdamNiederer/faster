// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};

pub trait SaturatingAdd {
    fn saturating_add(&self, other: Self) -> Self;
}

rust_fallback_impl_binary! {
    impl SaturatingAdd for u8x16 where "sse2" {
        saturating_add => _mm_adds_epu8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl SaturatingAdd for i8x16 where "sse2" {
        saturating_add => _mm_adds_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl SaturatingAdd for u16x8 where "sse2" {
        saturating_add => _mm_adds_epu16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl SaturatingAdd for i16x8 where "sse2" {
        saturating_add => _mm_adds_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}


rust_fallback_impl_binary! {
    impl SaturatingAdd for u8x32 where "avx2" {
        saturating_add => _mm256_adds_epu8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    }
}

rust_fallback_impl_binary! {
    impl SaturatingAdd for i8x32 where "avx2" {
        saturating_add => _mm256_adds_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_impl_binary! {
    impl SaturatingAdd for u16x16 where "avx2" {
        saturating_add => _mm256_adds_epu16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl SaturatingAdd for i16x16 where "avx2" {
        saturating_add => _mm256_adds_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}
