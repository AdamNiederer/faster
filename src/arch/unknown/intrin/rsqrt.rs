// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::intrin::rsqrt::*;
use crate::arch::current::vecs::*;
use crate::vecs::*;

// TODO: Guards and non-simd
//
//rust_fallback_impl! {
//    impl Rsqrt for f32x8 where "__undefined" {
//        rsqrt => _mm256_rsqrt_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
//    }
//}

rust_fallback_impl! {
    impl Rsqrt for f32x4 where "__undefined" {
        rsqrt => __undefined(), [0, 1, 2, 3];
    }
}

rust_fallback_impl! {
    impl Rsqrt for f64x2 where "__undefined" {
        rsqrt => __undefined(), [0, 1];
    }
}

impl Rsqrt for f32 {
    #[inline(always)]
    fn rsqrt(&self) -> Self {
        self.sqrt().recip()
    }
}

impl Rsqrt for f64 {
    #[inline(always)]
    fn rsqrt(&self) -> Self {
        self.sqrt().recip()
    }
}
