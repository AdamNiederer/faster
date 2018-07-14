// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::recip::Recip;

rust_fallback_impl! {
    impl Recip for f32x8 where "avx" {
        recip => _mm256_rcp_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl Recip for f32x4 where "sse" {
        recip => _mm_rcp_ps(), [0, 1, 2, 3];
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::f32::INFINITY;

    #[test]
    fn recip_f32s() {
        let mut i = -1024.0;
        while i < 1024.0 {
            // This test has some pretty significant float error if done on x86
            let ans = f32s(i).recip().extract(0);
            let real = f32s(1.0 / i).extract(0);
            assert!((real == INFINITY && ans == INFINITY) || (ans - real).abs() < 0.0005);
            i += 1.0
        }
    }
}
