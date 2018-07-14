// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::std::ops::BitXor;
use crate::arch::current::vecs::*;

pub trait Eq : Packed {
    type Out : Pattern + BitXor<Self::Out, Output = Self::Out>;

    /// Return a vector where each element at an index i is filled with 1s if
    /// the elements of `self` and `other` at index i are equal, and filled with
    /// zeroes otherwise.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s::interleave(0, 2).eq_mask(u8s(0)).be_u8s(), u8s::interleave(0xFF, 0).be_u8s());
    /// assert_eq!(u32s::halfs(1, 0).eq_mask(u32s(0)), u32s::halfs(0, 0xFFFFFFFF));
    /// # }
    /// ```
    fn eq_mask(&self, other: Self) -> Self::Out;

    /// Return a vector where each element at an index i is filled with 1s if
    /// the elements of `self` and `other` at index i are not equal, and filled
    /// with zeroes otherwise.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s::interleave(0, 2).ne_mask(u8s(0)), u8s::interleave(0, 0xFF));
    /// assert_eq!(u32s::halfs(1, 0).ne_mask(u32s(0)), u32s::halfs(0xFFFFFFFF, 0));
    /// # }
    /// ```
    fn ne_mask(&self, other: Self) -> Self::Out {
        self.eq_mask(other) ^ Self::Out::ones()
    }
}
