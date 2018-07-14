// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vecs::*;

pub trait Cmp {
    /// Return a vector where each element at an index i is the maximum of the
    /// elements at index i in `self` and `other`.
    ///
    /// ```ignore
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i8s(0).max(i8s(2)), i8s(2));
    /// assert_eq!(i8s::halfs(1, 0).max(i8s::halfs(2, -1)), i8s::halfs(2, 0));
    /// # }
    /// ```
    fn max(&self, other: Self) -> Self;

    /// Return a vector where each element at an index i is the minimum of the
    /// elements at index i in `self` and `other`.
    ///
    /// ```ignore
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i8s(0).min(i8s(2)), i8s(0));
    /// assert_eq!(i8s::halfs(1, 0).min(i8s::halfs(2, -1)), i8s::halfs(1, -1));
    /// # }
    /// ```
    fn min(&self, other: Self) -> Self;
}
