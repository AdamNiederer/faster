// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use stdsimd::simd::*;

// TODO: Guards and non-simd

pub trait PackedRsqrt {
    fn rsqrt(&self) -> Self;
}

impl PackedRsqrt for f32x4 {
    #[inline(always)]
    fn rsqrt(&self) -> Self {
        unsafe { _mm_rsqrt_ps(*self) }
    }
}

impl PackedRsqrt for f32x8 {
    #[inline(always)]
    fn rsqrt(&self) -> Self {
        unsafe { _mm256_rsqrt_ps(*self) }
    }
}
