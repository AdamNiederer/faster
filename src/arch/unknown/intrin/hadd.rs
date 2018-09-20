// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::intrin::hadd::*;
use crate::core::ops::Add;
use crate::arch::current::vecs::*;
use crate::vecs::*;

impl HAdd for u64x2 { hop!(hadd, Add::add, 0, 1); }
impl HAdd for u32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl HAdd for u16x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HAdd for u8x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HAdd for i64x2 { hop!(hadd, Add::add, 0, 1); }
impl HAdd for i32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl HAdd for i16x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HAdd for i8x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HAdd for f64x2 { hop!(hadd, Add::add, 0, 1); }
impl HAdd for f32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
