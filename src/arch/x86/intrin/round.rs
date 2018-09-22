// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::intrin::round::Round;
use crate::core::arch::x86_64::{_MM_FROUND_TO_NEAREST_INT, _MM_FROUND_TRUNC};
use crate::arch::current::vecs::*;
use crate::vecs::*;

rust_fallback_impl! {
    impl Round for f32x4 where "sse4.1" {
        round => _mm_round_ps(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3];
        ceil => _mm_ceil_ps(), [0, 1, 2, 3];
        floor => _mm_floor_ps(), [0, 1, 2, 3];
        trunc => _mm_round_ps(_MM_FROUND_TRUNC), [0, 1, 2, 3];
    }
}

rust_fallback_impl! {
    impl Round for f64x2 where "sse4.1" {
        round => _mm_round_pd(_MM_FROUND_TO_NEAREST_INT), [0, 1];
        ceil => _mm_ceil_pd(), [0, 1];
        floor => _mm_floor_pd(), [0, 1];
        trunc => _mm_round_pd(_MM_FROUND_TRUNC), [0, 1];
    }
}

rust_fallback_impl! {
    impl Round for f32x8 where "avx" {
        round => _mm256_round_ps(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3, 4, 5, 6, 7];
        ceil => _mm256_ceil_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
        floor => _mm256_floor_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
        trunc => _mm256_round_ps(_MM_FROUND_TRUNC), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl Round for f64x4 where "sse4.1" {
        round => _mm256_round_pd(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3];
        ceil => _mm256_ceil_pd(), [0, 1, 2, 3];
        floor => _mm256_floor_pd(), [0, 1, 2, 3];
        trunc => _mm256_round_pd(_MM_FROUND_TRUNC), [0, 1, 2, 3];
    }
}
