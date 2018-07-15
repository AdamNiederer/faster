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