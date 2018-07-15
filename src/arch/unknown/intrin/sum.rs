// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::sum::{Sum,UpcastSum};

impl_packed_sum!(u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, u64x2, i64x2, f32x4, f64x2);
impl_packed_upcast_sum!(u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, u64x2, i64x2, f32x4, f64x2);

mod tests {
    #![allow(unused_imports)]

    use crate::prelude::*;
    use crate::arch::current::vecs::*;

    test_packed_sum_int!(u8x16, u8, test_packed_sum_u8x16);
    test_packed_sum_int!(i8x16, i8, test_packed_sum_i8x16);
    test_packed_sum_int!(u16x8, u16, test_packed_sum_u16x8);
    test_packed_sum_int!(i16x8, i16, test_packed_sum_i16x8);
    test_packed_sum_int!(u32x4, u32, test_packed_sum_u32x4);
    test_packed_sum_int!(i32x4, i32, test_packed_sum_i32x4);
    test_packed_sum_int!(u64x2, u64, test_packed_sum_u64x2);
    test_packed_sum_int!(i64x2, i64, test_packed_sum_i64x2);

    test_packed_sum!(f32x4, f32, test_packed_sum_f32x4);
    test_packed_sum!(f64x2, f64, test_packed_sum_f64x2);
}
