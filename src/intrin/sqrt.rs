// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Sqrt {
    /// Return a vector the containing square roots of the elements of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(4.0).sqrt(), f32s(2.0));
    /// assert_eq!(f64s(9.0).sqrt(), f64s(3.0));
    /// # }
    /// ```
    fn sqrt(&self) -> Self;
}
