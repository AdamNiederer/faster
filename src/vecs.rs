// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(target_feature = "sse")]
pub use crate::std::simd::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
#[cfg(not(target_feature = "sse"))]
pub use shimvecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
pub use crate::vec_patterns::Pattern;
use crate::std::fmt::Debug;
use crate::intrin::*;

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
                $vec::load(data, offset)
            }

            #[inline(always)]
            unsafe fn load_unchecked(data: &[$el], offset: usize) -> $vec {
                debug_assert!(data[offset..].len() >= Self::WIDTH);
                $vec::load_unchecked(data, offset)
            }

            #[inline(always)]
            fn store(self, data: &mut [$el], offset: usize) {
                $vec::store(self, data, offset);
            }

            #[inline(always)]
            unsafe fn store_unchecked(self, data: &mut [$el], offset: usize) {
                debug_assert!(data[offset..].len() >= Self::WIDTH);
                $vec::store_unchecked(self, data, offset);
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

impl_packed!(u8, u8s, u8x64, 1, 64, ["avx512"], ["avx1024"]);
impl_packed!(u8, u8s, u8x32, 1, 32, ["avx2"], ["avx512"]);
impl_packed!(u8, u8s, u8x16, 1, 16, [], ["avx2"]);
impl_packed!(i8, i8s, i8x64, 1, 64, ["avx512"], ["avx1024"]);
impl_packed!(i8, i8s, i8x32, 1, 32, ["avx2"], ["avx512"]);
impl_packed!(i8, i8s, i8x16, 1, 16, [], ["avx2"]);
impl_packed!(u16, u16s, u16x32, 2, 32, ["avx512"], ["avx1024"]);
impl_packed!(u16, u16s, u16x16, 2, 16, ["avx2"], ["avx512"]);
impl_packed!(u16, u16s, u16x8, 2, 8, [], ["avx2"]);
impl_packed!(i16, i16s, i16x32, 2, 32, ["avx512"], ["avx1024"]);
impl_packed!(i16, i16s, i16x16, 2, 16, ["avx2"], ["avx512"]);
impl_packed!(i16, i16s, i16x8, 2, 8, [], ["avx2"]);
impl_packed!(u32, u32s, u32x16, 4, 16, ["avx512"], ["avx1024"]);
impl_packed!(u32, u32s, u32x8, 4, 8, ["avx2"], ["avx512"]);
impl_packed!(u32, u32s, u32x4, 4, 4, [], ["avx2"]);
impl_packed!(i32, i32s, i32x16, 4, 16, ["avx512"], ["avx1024"]);
impl_packed!(i32, i32s, i32x8, 4, 8, ["avx2"], ["avx512"]);
impl_packed!(i32, i32s, i32x4, 4, 4, [], ["avx2"]);
impl_packed!(f32, f32s, f32x16, 4, 16, ["avx512"], ["avx1024"]);
impl_packed!(f32, f32s, f32x8, 4, 8, ["avx2"], ["avx512"]);
impl_packed!(f32, f32s, f32x4, 4, 4, [], ["avx2"]);
impl_packed!(u64, u64s, u64x8, 8, 8, ["avx512"], ["avx1024"]);
impl_packed!(u64, u64s, u64x4, 8, 4, ["avx2"], ["avx512"]);
impl_packed!(u64, u64s, u64x2, 8, 2, [], ["avx2"]);
impl_packed!(i64, i64s, i64x8, 8, 8, ["avx512"], ["avx1024"]);
impl_packed!(i64, i64s, i64x4, 8, 4, ["avx2"], ["avx512"]);
impl_packed!(i64, i64s, i64x2, 8, 2, [], ["avx2"]);
impl_packed!(f64, f64s, f64x8, 8, 8, ["avx512"], ["avx1024"]);
impl_packed!(f64, f64s, f64x4, 8, 4, ["avx2"], ["avx512"]);
impl_packed!(f64, f64s, f64x2, 8, 2, [], ["avx2"]);

#[cfg(test)]
mod tests {
    use super::Packed;
    use super::*;

    macro_rules! test_product {
        (($($el:tt),*), ($($vec:tt),*), ($($fn:tt),*), ($($sum:tt),*)) => (
            $(
                #[test]
                fn $fn() {
                    assert_eq!($vec::splat(1i8 as $el).product(), $sum as $el);
                }
            )*
        )
    }

    // TODO: Do we need better test cases for this?
    test_product!((u8, u8, u8, i8, i8, i8, u16, u16, u16, i16, i16, i16, u32, u32, u32, i32, i32, i32, f32, f32, f32, u64, u64, u64, i64, i64, i64, f64, f64, f64),
                  (u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2),
                  (scalar_product_u8x64, scalar_product_u8x32, scalar_product_u8x16, scalar_product_i8x64, scalar_product_i8x32, scalar_product_i8x16, scalar_product_u16x32, scalar_product_u16x16, scalar_product_u16x8, scalar_product_i16x32, scalar_product_i16x16, scalar_product_i16x8, scalar_product_u32x16, scalar_product_u32x8, scalar_product_u32x4, scalar_product_i32x16, scalar_product_i32x8, scalar_product_i32x4, scalar_product_f32x16, scalar_product_f32x8, scalar_product_f32x4, scalar_product_u64x8, scalar_product_u64x4, scalar_product_u64x2, scalar_product_i64x8, scalar_product_i64x4, scalar_product_i64x2, scalar_product_f64x8, scalar_product_f64x4, scalar_product_f64x2),
                  (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1));
}
