// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
pub use crate::vec_patterns::Pattern;
use crate::std::fmt::Debug;
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

