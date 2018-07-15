// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::hsub::*;
use crate::std::ops::Sub;

impl HSub for u64x2 { hop!(hsub, Sub::sub, 0, 1); }
impl HSub for u32x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
impl HSub for u16x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HSub for u8x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HSub for i64x2 { hop!(hsub, Sub::sub, 0, 1); }
impl HSub for i32x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
impl HSub for i16x8 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HSub for i8x16 { hop!(hsub, Sub::sub, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HSub for f64x2 { hop!(hsub, Sub::sub, 0, 1); }
impl HSub for f32x4 { hop!(hsub, Sub::sub, 0, 1, 2, 3); }
