// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::recip::Recip;

rust_fallback_impl! {
    impl Recip for f32x4 where "__undefined" {
        recip => __undefined(), [0, 1, 2, 3];
    }
}
