// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Swizzle {
    /// Return a vector containing elements of self, but with even and odd
    /// elements swapped in-place. For (n = 0, 2, ... Self::WIDTH), elements at
    /// indices n and n + 1 are swapped.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s::interleave(2, 1).flip(), u8s::interleave(1, 2));
    /// assert_eq!(u64s::interleave(2, 1).flip(), u64s::interleave(1, 2));
    /// # }
    /// ```
    fn flip(&self) -> Self;
}

macro_rules! impl_packed_swizzle {
    ($vec:tt, $uvec:tt, $feat:expr, $mmfn:tt, ($($c:expr),*), ($($a:expr, $b:expr),*)) => {
        impl Swizzle for $vec {
            #[cfg(not(target_feature = $feat))]
            #[inline(always)]
            fn flip(&self) -> Self {
                fallback!();
                $vec::new($(self.extract($b), self.extract($a)),*)
            }

            #[cfg(target_feature = $feat)]
            #[inline(always)]
            fn flip(&self) -> Self {
                unsafe {
                    transmute($mmfn(self.be_i8s(), $uvec::new($($c),*).be_i8s()))
                }
            }
        }
    }
}
