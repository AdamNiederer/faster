// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::intrin::round::Round;
use crate::arch::current::vecs::*;
use crate::vecs::*;

rust_fallback_impl! {
    impl Round for f32x4 where "__undefined" {
        round => __undefined(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3];
        ceil => __undefined(), [0, 1, 2, 3];
        floor => __undefined(), [0, 1, 2, 3];
        trunc => __undefined(_MM_FROUND_TRUNC), [0, 1, 2, 3];
    }
}

rust_fallback_impl! {
    impl Round for f64x2 where "__undefined" {
        round => __undefined(_MM_FROUND_TO_NEAREST_INT), [0, 1];
        ceil => __undefined(), [0, 1];
        floor => __undefined(), [0, 1];
        trunc => __undefined(_MM_FROUND_TRUNC), [0, 1];
    }
}

rust_fallback_impl! {
    impl Round for f32x8 where "__undefined" {
        round => __undefined(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3, 4, 5, 6, 7];
        ceil => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
        floor => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
        trunc => __undefined(_MM_FROUND_TRUNC), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl Round for f64x4 where "__undefined" {
        round => __undefined(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3];
        ceil => __undefined(), [0, 1, 2, 3];
        floor => __undefined(), [0, 1, 2, 3];
        trunc => __undefined(_MM_FROUND_TRUNC), [0, 1, 2, 3];
    }
}
