// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Downcast<T> {
    /// Return a vector containing elements of the same value as `self` and
    /// `other`, but different type. The first half of the returned vector
    /// contains the downcast values of `self`, whereas the second half of the
    /// returned vector contains the downcast values of `other`. The returned
    /// vector is equal in size to `self` and `other`. If an element exceeds
    /// the maximum or minimum value of the downcast type, it is saturated.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i32s(2).saturating_downcast(i32s(3)), i16s::halfs(2, 3));
    /// assert_eq!(i16s(128).saturating_downcast(i16s(-129)), i8s::halfs(127, -128));
    /// # }
    /// ```
    fn saturating_downcast(self, other: Self) -> T;
}
