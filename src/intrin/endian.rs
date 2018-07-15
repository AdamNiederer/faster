// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Reendianize : Sized + Copy {
    /// Return a vector containing elements of `self` with switched endianness.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u32s(0xDEADBEEF).swap_bytes(), u32s(0xEFBEADDE));
    /// # }
    /// ```
    fn swap_bytes(&self) -> Self;

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn to_be(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn to_be(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn to_le(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn to_le(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn from_be(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn from_be(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn from_le(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn from_le(&self) -> Self {
        *self
    }
}

macro_rules! impl_packed_swap_bytes {
    ($vec:tt, $uvec:tt, $feat:expr, $mmfn:tt, ($($c:expr),*), ($($a:expr, $b:expr),*)) => {
        impl Reendianize for $vec {
            #[cfg(not(target_feature = $feat))]
            #[inline(always)]
            fn swap_bytes(&self) -> Self {
                $vec::new($(self.extract($a).swap_bytes(),
                            self.extract($b).swap_bytes()),*)
            }

            #[cfg(target_feature = $feat)]
            #[inline(always)]
            fn swap_bytes(&self) -> Self {
                unsafe {
                    transmute($mmfn(self.be_i8s(), $uvec::new($($c),*).be_i8s()))
                }
            }
        }
    }
}

macro_rules! test_packed_swap_bytes {
        (($($vec:tt),*), ($($fn:tt),*)) => {
            $(
                #[test]
                fn $fn() {
                    let a = $vec::interleave(33u8 as <$vec as Packed>::Scalar,
                                             92u8 as <$vec as Packed>::Scalar);
                    let b = $vec::interleave((33u8 as <$vec as Packed>::Scalar).swap_bytes(),
                                             (92u8 as <$vec as Packed>::Scalar).swap_bytes());
                    assert_eq!(a.swap_bytes(), b);
                }
            )*
        }
    }
