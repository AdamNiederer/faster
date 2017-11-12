// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedRecip {
    fn recip(&self) -> Self;
}

rust_fallback_impl! {
    impl PackedRecip for f32x8 where "avx" {
        recip => _mm256_rcp_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl PackedRecip for f32x4 where "sse" {
        recip => _mm_rcp_ps(), [0, 1, 2, 3];
    }
}
