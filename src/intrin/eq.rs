// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::std::ops::BitXor;
use crate::vecs::*;

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
    #[inline(always)]
    fn ne_mask(&self, other: Self) -> Self::Out { self.eq_mask(other) ^ Self::Out::ones() }
}


macro_rules! rust_fallback_eq {
    (impl $trait:tt for $type:tt where $feat:tt {
        $($newfn:ident, $rustfn:ident => $mask:tt, $maskel:tt, $mmfn:tt ( $($mmfnargs:expr),* ), [$($n:expr),+]);*;}) => (
        impl $trait for $type {
            $(
                type Out = $mask;

                #[inline(always)]
                #[cfg(target_feature = $feat)]
                fn $newfn(&self, other: Self) -> $mask {
                    use crate::std::mem::transmute;
                    unsafe { transmute($mmfn(transmute(*self), transmute(other), $($mmfnargs),*)) }
                }

                #[inline(always)]
                #[cfg(not(target_feature = $feat))]
                fn $newfn(&self, other: Self) -> Self::Out {
                    use crate::std::mem::transmute;
                    unsafe {
                        Self::Out::new($(transmute(if self.extract($n).$rustfn(&other.extract($n)) {
                            $maskel::max_value()
                        } else {
                            $maskel::min_value()
                        })),*)
                    }
                }
            )*
        }
    );
}

macro_rules! test_packed_eq {
        ($vec:tt, $el:tt, $mask:tt, $maskel:tt, $name:tt) => {
            #[test]
            fn $name() {
                assert_eq!($vec::halfs(1 as $el, 0 as $el).eq_mask($vec::splat(0 as $el)),
                           $mask::halfs(0, $maskel::max_value()));

                assert_eq!($vec::interleave(1 as $el, 0 as $el).eq_mask($vec::splat(1 as $el)),
                           $mask::interleave($maskel::max_value(), 0));

                assert_eq!($vec::halfs(1 as $el, 0 as $el).ne_mask($vec::splat(0 as $el)),
                           $mask::halfs($maskel::max_value(), 0));

                assert_eq!($vec::interleave(1 as $el, 0 as $el).ne_mask($vec::splat(1 as $el)),
                           $mask::interleave(0, $maskel::max_value()));
            }
        }
    }
