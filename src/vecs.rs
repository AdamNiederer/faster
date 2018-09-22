// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![allow(dead_code)]

pub use crate::vec_patterns::Pattern;
use crate::core::fmt::Debug;
use crate::intrin::merge::*;

/// A SIMD vector of some type.
pub trait Packed : Sized + Copy + Debug + Merge {
    /// The type which fits into this SIMD vector
    type Scalar : Packable;

    /// The number of elements in this vector
    const WIDTH: usize;

    #[inline(always)]
    /// Return the number of elements in this vector
    fn width(&self) -> usize {
        Self::WIDTH
    }

    /// Create a new vector with `Self::WIDTH` elements from `data`, beginning
    /// at `offset`.
    fn load(data: &[Self::Scalar], offset: usize) -> Self;

    /// Create a new vector with `Self::WIDTH` elements from `data`, beginning
    /// at `offset`, without asserting length of data.
    unsafe fn load_unchecked(data: &[Self::Scalar], offset: usize) -> Self;

    /// Write `Self::WIDTH` elements from this vector to `data`, beginning at
    /// `offset`.
    fn store(self, data: &mut [Self::Scalar], offset: usize);

    /// Create a new vector with `Self::WIDTH` elements from `data`, beginning
    /// at `offset`, without asserting length of data.
    unsafe fn store_unchecked(self, data: &mut [Self::Scalar], offset: usize);

    /// Assert all elements of the vector are equal, then return the
    /// element. Opposite operation of `Self::splat`.
    fn coalesce(self) -> Self::Scalar;

    /// Return a vector with all elements initialized to `data`. Opposite
    /// operation for `Self::coalesce`.
    fn splat(data: Self::Scalar) -> Self;

    /// Return a vector with all elements initialized to the default
    /// value for the underlying element type.
    fn default() -> Self;

    /// Return the `idx`th element of this vector.
    fn extract(&self, idx: usize) -> Self::Scalar;

    /// Return the `idx`th element of this vector.
    unsafe fn extract_unchecked(&self, idx: usize) -> Self::Scalar;

    /// Replace the `idx`th element of this vector with `data`.
    fn replace(&mut self, idx: usize, data: Self::Scalar) -> Self;

    /// Replace the `idx`th element of this vector with `data`.
    unsafe fn replace_unchecked(&mut self, idx: usize, data: Self::Scalar) -> Self;

    /// Return a scalar equivalent to the product of all elements of this
    /// vector.
    fn product(&self) -> Self::Scalar;

    /// Return the result of a scalar reduction over this vector
    fn scalar_reduce<T, F>(&self, acc: T, func: F) -> T
    where F: FnMut(T, Self::Scalar) -> T;
}

/// A type that may be packed into a SIMD vector.
pub trait Packable where Self : Sized + Copy + Debug {
    type Vector : Packed<Scalar = Self> + Clone;
    const SIZE: usize;
}

// Vector types which aren't interpreted as SIMD vectors, for systems which
// don't have SIMD support.

macro_rules! impl_packed {
    ($el:tt, $pvec:tt, $vec:tt, $sz:expr, $width:expr, [$($feat:expr),*], [$($nfeat:expr),*]) => (

        /// A SIMD vector of this primitive type.
        #[allow(non_camel_case_types)]
        #[cfg(all($(target_feature = $feat,)* not($(target_feature = $nfeat)*)))]
        pub type $pvec = $vec;

        /// Return a vector of this type with all elements initialized to
        /// `data`.
        #[inline(always)]
        #[cfg(all($(target_feature = $feat,)* not($(target_feature = $nfeat)*)))]
        pub fn $pvec(data: $el) -> $pvec {
            $vec::splat(data)
        }

        #[cfg(all($(target_feature = $feat,)* not($(target_feature = $nfeat)*)))]
        impl Packable for $el {
            type Vector = $vec;
            const SIZE: usize = $sz;
        }

        impl Packed for $vec {
            type Scalar = $el;

            const WIDTH: usize = $width;

            #[inline(always)]
            fn load(data: &[$el], offset: usize) -> $vec {
                $vec::from_slice_unaligned(&data[offset..])
            }

            #[inline(always)]
            unsafe fn load_unchecked(data: &[$el], offset: usize) -> $vec {
                debug_assert!(data[offset..].len() >= Self::WIDTH);
                $vec::from_slice_unaligned_unchecked(&data[offset..])
            }

            #[inline(always)]
            fn store(self, data: &mut [$el], offset: usize) {
                $vec::write_to_slice_unaligned(self, &mut data[offset..]);
            }

            #[inline(always)]
            unsafe fn store_unchecked(self, data: &mut [$el], offset: usize) {
                debug_assert!(data[offset..].len() >= Self::WIDTH);
                $vec::write_to_slice_unaligned_unchecked(self, &mut data[offset..]);
            }

            #[inline(always)]
            fn coalesce(self) -> Self::Scalar {
                for i in 1..Self::WIDTH {
                    debug_assert!(self.extract(i - 1) == self.extract(i));
                }
                self.extract(0)
            }

            #[inline(always)]
            fn extract(&self, idx: usize) -> Self::Scalar {
                $vec::extract(*self, idx )
            }

            #[inline(always)]
            unsafe fn extract_unchecked(&self, idx: usize) -> Self::Scalar {
                debug_assert!(idx < Self::WIDTH);
                $vec::extract_unchecked(*self, idx )
            }

            #[inline(always)]
            fn replace(&mut self, idx: usize, data: Self::Scalar) -> Self {
                $vec::replace(*self, idx, data)
            }

            #[inline(always)]
            unsafe fn replace_unchecked(&mut self, idx: usize, data: Self::Scalar) -> Self {
                debug_assert!(idx < Self::WIDTH);
                $vec::replace_unchecked(*self, idx, data)
            }

            #[inline(always)]
            fn splat(data: $el) -> Self {
                $vec::splat(data)
            }

            #[inline(always)]
            fn default() -> Self {
                $vec::splat($el::default())
            }

            #[inline(always)]
            fn product(&self) -> Self::Scalar {
                let mut acc = 1 as $el;
                for i in 0..Self::WIDTH {
                    acc *= self.extract(i)
                }
                acc
            }

            #[inline(always)]
            fn scalar_reduce<T, F>(&self, mut acc: T, mut func: F) -> T
            where F: FnMut(T, Self::Scalar) -> T {
                for i in 0..Self::WIDTH {
                    acc = func(acc, self.extract(i))
                }
                acc
            }
        }
    );
}

