// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// TODO: Upcast<i..> for u..

pub trait Upcast<T> {
    /// Return two vectors containing elements of the same value, but different
    /// type. The first vector contains the first half of `self`, and the second
    /// vector contains the second half. Both returned vectors are equal in size
    /// to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i8s::halfs(2, 3).upcast(), (i16s(2), i16s(3)))
    /// # }
    /// ```
    fn upcast(self) -> (T, T);
}
