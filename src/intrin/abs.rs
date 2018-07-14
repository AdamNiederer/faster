// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


pub trait Abs {
    type Out;
    /// Return a vector the absolute value of the elements of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i32s(-2).abs(), u32s(2));
    /// assert_eq!(i8s(-256).abs(), u8s(256));
    /// # }
    /// ```
    fn abs(&self) -> Self::Out;
}
