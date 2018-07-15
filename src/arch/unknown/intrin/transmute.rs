// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::intrin::transmute::*;
use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::std::mem::transmute;



impl_packed_transmute!(u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, f32x4,
                       u64x2, i64x2, f64x2, ...
                       u8x16, i8x16, u16x8, i16x8, u32x4, i32x4,
                       f32x4, u64x2, i64x2, f64x2,
                       "__undefined", "__undefined");

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    macro_rules! test_transmute {
        ($name:ident, $val:expr, $xmute:ident) => (
            #[test]
            fn $name() {
                #![allow(unused_unsafe)]
                assert_eq!(unsafe { $val.be_i8s().$xmute() }, $val);
                assert_eq!(unsafe { $val.be_u8s().$xmute() }, $val);
                assert_eq!(unsafe { $val.be_i16s().$xmute() }, $val);
                assert_eq!(unsafe { $val.be_u16s().$xmute() }, $val);
                assert_eq!(unsafe { $val.be_i32s().$xmute() }, $val);
                assert_eq!(unsafe { $val.be_u32s().$xmute() }, $val);
                assert_eq!(unsafe { $val.be_i64s().$xmute() }, $val);
                assert_eq!(unsafe { $val.be_u64s().$xmute() }, $val);
            }
        )
    }

    test_transmute!(transmute_u8s, u8s(1), be_u8s);
    test_transmute!(transmute_i8s, i8s(1), be_i8s);
    test_transmute!(transmute_u16s, u16s(1), be_u16s);
    test_transmute!(transmute_i16s, i16s(1), be_i16s);
    test_transmute!(transmute_u32s, u32s(1), be_u32s);
    test_transmute!(transmute_i32s, i32s(1), be_i32s);
    test_transmute!(transmute_f32s, f32s(1.0), be_f32s_unchecked);
    test_transmute!(transmute_u64s, u64s(1), be_u64s);
    test_transmute!(transmute_i64s, i64s(1), be_i64s);
    test_transmute!(transmute_f64s, f64s(1.0), be_f64s_unchecked);
}
