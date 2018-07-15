// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::intrin::saturating_sub::*;
use crate::arch::current::vecs::*;
use crate::vecs::*;

rust_fallback_impl_binary! {
    impl SaturatingSub for u8x16 where "__undefined" {
        saturating_sub => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl SaturatingSub for i8x16 where "__undefined" {
        saturating_sub => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl SaturatingSub for u16x8 where "__undefined" {
        saturating_sub => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl SaturatingSub for i16x8 where "__undefined" {
        saturating_sub => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}
