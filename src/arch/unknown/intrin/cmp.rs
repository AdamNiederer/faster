// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::cmp::*;

rust_fallback_impl_binary! {
    impl Cmp for u8x16 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for i8x16 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for u16x8 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for i16x8 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for u32x4 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3];
        max => __undefined(), [0, 1, 2, 3];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for i32x4 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3];
        max => __undefined(), [0, 1, 2, 3];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for f32x4 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3];
        max => __undefined(), [0, 1, 2, 3];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for f64x2 where "__undefined" {
        min => __undefined(), [0, 1];
        max => __undefined(), [0, 1];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for u8x32 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for i8x32 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for u16x16 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for i16x16 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for u32x8 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for i32x8 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for f32x8 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl Cmp for f64x4 where "__undefined" {
        min => __undefined(), [0, 1, 2, 3];
        max => __undefined(), [0, 1, 2, 3];
    }
}
