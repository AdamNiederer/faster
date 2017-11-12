// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedSqrt {
    fn sqrt(&self) -> Self;
}

rust_fallback_impl! {
    impl PackedSqrt for f32x8 where "avx" {
        sqrt => _mm256_sqrt_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl PackedSqrt for f64x4 where "avx" {
        sqrt => _mm256_sqrt_pd(), [0, 1, 2, 3];
    }
}

rust_fallback_impl! {
    impl PackedSqrt for f32x4 where "sse" {
        sqrt => _mm_sqrt_ps(), [0, 1, 2, 3];
    }
}
rust_fallback_impl! {
    impl PackedSqrt for f64x2 where "sse2" {
        sqrt => _mm_sqrt_pd(), [0, 1];
    }
}
