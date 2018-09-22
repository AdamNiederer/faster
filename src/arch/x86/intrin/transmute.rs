// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::intrin::transmute::*;
use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::core::mem::transmute;

impl_packed_transmute!(u8x32, i8x32, u16x16, i16x16, u32x8, i32x8, f32x8,
                       u64x4, i64x4, f64x4, ...
                       u8x32, i8x32, u16x16, i16x16, u32x8, i32x8,
                       f32x8, u64x4, i64x4, f64x4,
                       "avx", "avx512");
impl_packed_transmute!(u8x64, i8x64, u16x32, i16x32, u32x16, i32x16, f32x16,
                       u64x8, i64x8, f64x8, ...
                       u8x64, i8x64, u16x32, i16x32, u32x16, i32x16,
                       f32x16, u64x8, i64x8, f64x8,
                       "avx512", "avx1024");
impl_packed_transmute!(u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, f32x4,
                       u64x2, i64x2, f64x2, ...
                       u8x16, i8x16, u16x8, i16x8, u32x4, i32x4,
                       f32x4, u64x2, i64x2, f64x2,
                       "sse", "avx");
