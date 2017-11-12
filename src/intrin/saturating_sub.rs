// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedSaturatingSub {
    fn saturating_sub(&self, other: Self) -> Self;
}

rust_fallback_impl_binary! {
    impl PackedSaturatingSub for i8x16 where "sse2" {
        saturating_sub => _mm_subs_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}
rust_fallback_impl_binary! {
    impl PackedSaturatingSub for i16x8 where "sse2" {
        saturating_sub => _mm_subs_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingSub for i8x32 where "avx2" {
        saturating_sub => _mm256_subs_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_impl_binary! {
    impl PackedSaturatingSub for i16x16 where "avx2" {
        saturating_sub => _mm256_subs_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}
