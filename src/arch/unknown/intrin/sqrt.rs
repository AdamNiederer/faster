// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::intrin::sqrt::*;
use crate::arch::current::vecs::*;
use crate::vecs::*;

rust_fallback_impl! {
    impl Sqrt for f32x4 where "__undefined" {
        sqrt => __undefined(), [0, 1, 2, 3];
    }
}

rust_fallback_impl! {
    impl Sqrt for f64x2 where "__undefined" {
        sqrt => __undefined(), [0, 1];
    }
}
