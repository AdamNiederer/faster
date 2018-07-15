// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


pub trait Recip {
    /// Return a vector containing an estimation of the reciprocal of the
    /// corresponding elements of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert!(0.25 - 0.01 < f32s(4.0).recip().coalesce() &&
    ///         0.25 + 0.01 > f32s(4.0).recip().coalesce());
    /// # }
    /// ```
    fn recip(&self) -> Self;
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
