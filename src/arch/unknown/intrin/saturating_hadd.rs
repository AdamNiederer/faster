// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arch::current::vecs::*;
use crate::intrin::saturating_hadd::*;
use crate::vecs::*;

impl SaturatingHAdd for u64x2 {
    hop!(saturating_hadd, u64::saturating_add, 0, 1);
}
impl SaturatingHAdd for u32x4 {
    hop!(saturating_hadd, u32::saturating_add, 0, 1, 2, 3);
}
impl SaturatingHAdd for u16x8 {
    hop!(saturating_hadd, u16::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7);
}
impl SaturatingHAdd for u8x16 {
    hop!(
        saturating_hadd,
        u8::saturating_add,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15
    );
}
impl SaturatingHAdd for i64x2 {
    hop!(saturating_hadd, i64::saturating_add, 0, 1);
}
impl SaturatingHAdd for i32x4 {
    hop!(saturating_hadd, i32::saturating_add, 0, 1, 2, 3);
}
impl SaturatingHAdd for i16x8 {
    hop!(saturating_hadd, i16::saturating_add, 0, 1, 2, 3, 4, 5, 6, 7);
}
impl SaturatingHAdd for i8x16 {
    hop!(
        saturating_hadd,
        i8::saturating_add,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15
    );
}
