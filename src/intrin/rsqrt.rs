// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


// TODO: Guards and non-simd

pub trait Rsqrt {
    /// Return a vector containing an approximation of the reciprocals of the
    /// square-roots of elements in `self`. May contain significant float error
    /// past 10^-3.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert!(0.33333333 - 0.01 < f32s(9.0).rsqrt().coalesce() &&
    ///         0.33333333 + 0.01 > f32s(9.0).rsqrt().coalesce());
    /// # }
    /// ```
    fn rsqrt(&self) -> Self;
}
