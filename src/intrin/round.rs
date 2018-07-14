// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


pub trait Round {
    /// Return a vector with all elements of `self` rounded to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).round(), f32s(3.0));
    /// assert_eq!(f32s(2.4).round(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).round(), f32s(-3.0));
    /// assert_eq!(f32s(-2.4).round(), f32s(-2.0));
    /// # }
    /// ```
    fn round(&self) -> Self;

    /// Return a vector with all elements of `self` rounded up to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).ceil(), f32s(3.0));
    /// assert_eq!(f32s(2.4).ceil(), f32s(3.0));
    /// assert_eq!(f32s(-2.7).ceil(), f32s(-2.0));
    /// assert_eq!(f32s(-2.4).ceil(), f32s(-2.0));
    /// # }
    /// ```
    fn ceil(&self) -> Self;

    /// Return a vector with all elements of `self` rounded down to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).floor(), f32s(2.0));
    /// assert_eq!(f32s(2.4).floor(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).floor(), f32s(-3.0));
    /// assert_eq!(f32s(-2.4).floor(), f32s(-3.0));
    /// # }
    /// ```
    fn floor(&self) -> Self;

    /// Return a vector with all elements of `self` truncated. Effectively
    /// rounds the elements towards zero.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).trunc(), f32s(2.0));
    /// assert_eq!(f32s(2.4).trunc(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).trunc(), f32s(-2.0));
    /// assert_eq!(f32s(-2.4).trunc(), f32s(-2.0));
    /// # }
    /// ```
    fn trunc(&self) -> Self;
}

