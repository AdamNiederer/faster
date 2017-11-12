// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedAddSub {
    fn addsub(&self, other: Self) -> Self;
}

impl PackedAddSub for f32x4 {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm_addsub_ps(*self, other) }
    }
}

impl PackedAddSub for f64x2 {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm_addsub_pd(*self, other) }
    }
}

impl PackedAddSub for f32x8 {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm256_addsub_ps(*self, other) }
    }
}

impl PackedAddSub for f64x4 {
    #[inline(always)]
    fn addsub(&self, other: Self) -> Self {
        unsafe { _mm256_addsub_pd(*self, other) }
    }
}
