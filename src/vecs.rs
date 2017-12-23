// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(target_feature = "sse")]
pub use stdsimd::simd::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
#[cfg(not(target_feature = "sse"))]
pub use shimvecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
use iters::{IntoPackedRefIterator, IntoPackedRefMutIterator, PackedIter};
use core_or_std::fmt::Debug;

/// A SIMD vector of some type.
pub trait Packed : Sized + Copy + Debug {
    /// The type which fits into this SIMD vector
    type Scalar : Packable;

    /// The number of elements in this vector
    const WIDTH: usize;

    #[inline(always)]
    /// Return the number of elements in this vector
    fn width(&self) -> usize {
        Self::WIDTH
    }

    /// Create a new vector with `Self::WIDTH` elements from `data`,
    /// beginning at `offset`.
    fn load(data: &[Self::Scalar], offset: usize) -> Self;

    /// Write `Self::WIDTH` elements from this vector to `data`,
    /// beginning at `offset`.
    fn store(self, data: &mut [Self::Scalar], offset: usize);

    /// Assert all elements of the vector are equal, then return the
    /// element. Opposiite operation of `Self::splat`.
    fn coalesce(self) -> Self::Scalar;

    /// Return a vector with all elements initialized to
    /// `data`. Opposite operation for `Self::coalesce`.
    fn splat(data: Self::Scalar) -> Self;

    /// Return a vector with all elements initialized to the default
    /// value for the underlying element type.
    fn default() -> Self;

    /// Return the `idx`th element of this vector.
    fn extract(&self, idx: usize) -> Self::Scalar;

    /// Replace the `idx`th element of this vector with `data`.
    fn replace(&mut self, idx: usize, data: Self::Scalar) -> Self;

    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self;

    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self;

    /// Return a scalar equivalent to the sum of all elements of this
    /// vector.
    fn sum(&self) -> Self::Scalar;

    /// Return a scalar equivalent to the product of all elements of
    /// this vector.
    fn product(&self) -> Self::Scalar;
}

/// A type that may be packed into a SIMD vector.
pub trait Packable where Self : Sized + Copy + Debug {
    type Vector : Packed<Scalar = Self> + Clone;
    const SIZE: usize;
}

macro_rules! impl_packed {
    ($el:tt, $pvec:tt, $vec:tt, $sz:expr, $width:expr, [$($feat:expr),*], [$($nfeat:expr),*],
     halfs => $halfs:expr,
     interleave => $interleave:expr) => (

        /// A SIMD vector of this primitive type.
        #[allow(non_camel_case_types)]
        #[cfg(all($(target_feature = $feat,)* not($(target_feature = $nfeat)*)))]
        pub type $pvec = $vec;

        #[cfg(all($(target_feature = $feat,)* not($(target_feature = $nfeat)*)))]
        impl Packable for $el {
            type Vector = $vec;
            const SIZE: usize = $sz;
        }

        #[cfg(all($(target_feature = $feat,)* not($(target_feature = $nfeat)*)))]
        impl Packed for $vec {
            type Scalar = $el;

            const WIDTH: usize = $width;

            #[inline(always)]
            fn load(data: &[$el], offset: usize) -> $vec {
                $vec::load(data, offset)
            }

            #[inline(always)]
            fn store(self, data: &mut [$el], offset: usize) {
                $vec::store(self, data, offset);
            }

            #[inline(always)]
            fn coalesce(self) -> Self::Scalar {
                for i in 1..(Self::WIDTH as u32) {
                    debug_assert!(self.extract(i - 1) == self.extract(i));
                }
                self.extract(0)
            }

            #[inline(always)]
            fn extract(&self, idx: usize) -> Self::Scalar {
                $vec::extract(*self, idx as u32)
            }

            #[inline(always)]
            fn replace(&mut self, idx: usize, data: Self::Scalar) -> Self {
                $vec::replace(*self, idx as u32, data)
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
            fn halfs(hi: $el, lo: $el) -> Self {
                $halfs(hi, lo)
            }

            #[inline(always)]
            fn interleave(hi: $el, lo: $el) -> Self {
                $interleave(hi, lo)
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
            fn sum(&self) -> Self::Scalar {
                let mut acc = 0 as $el;
                for i in 0..Self::WIDTH {
                    acc += self.extract(i)
                }
                acc
            }
        }

        #[cfg(all($(target_feature = $feat,)* not($(target_feature = $nfeat)*)))]
        impl<'a> IntoPackedRefIterator<'a> for &'a [$el] {
            type Iter = PackedIter<'a, $el>;

            #[inline(always)]
            fn simd_iter(&'a self) -> Self::Iter {
                PackedIter {
                    data: self,
                    position: 0,
                }
            }
        }

        #[cfg(all($(target_feature = $feat,)* not($(target_feature = $nfeat)*)))]
        impl<'a> IntoPackedRefMutIterator<'a> for &'a mut [$el] {
            type Iter = PackedIter<'a, $el>;

            #[inline(always)]
            fn simd_iter_mut(&'a mut self) -> Self::Iter {
                PackedIter {
                    data: self,
                    position: 0,
                }
            }
        }
    );
}

impl_packed! {
    u8, u8s, u8x64, 1, 64, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { u8x64::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x64::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u8, u8s, u8x32, 1, 32, ["avx"], ["avx512"],
    halfs => |hi, lo| { u8x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u8, u8s, u8x16, 1, 16, [], ["avx"],
    halfs => |hi, lo| { u8x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    i8, i8s, i8x64, 1, 64, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { i8x64::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x64::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i8, i8s, i8x32, 1, 32, ["avx"], ["avx512"],
    halfs => |hi, lo| { i8x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i8, i8s, i8x16, 1, 16, [], ["avx"],
    halfs => |hi, lo| { i8x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    u16, u16s, u16x32, 2, 32, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { u16x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u16, u16s, u16x16, 2, 2, ["avx"], ["avx512"],
    halfs => |hi, lo| { u16x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u16, u16s, u16x8, 2, 8, [], ["avx"],
    halfs => |hi, lo| { u16x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    i16, i16s, i16x32, 2, 32, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { i16x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i16, i16s, i16x16, 2, 2, ["avx"], ["avx512"],
    halfs => |hi, lo| { i16x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i16, i16s, i16x8, 2, 8, [], ["avx"],
    halfs => |hi, lo| { i16x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    u32, u32s, u32x16, 4, 16, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { u32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u32, u32s, u32x8, 4, 8, ["avx"], ["avx512"],
    halfs => |hi, lo| { u32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u32, u32s, u32x4, 4, 4, [], ["avx"],
    halfs => |hi, lo| { u32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { u32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    i32, i32s, i32x16, 4, 16, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { i32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i32, i32s, i32x8, 4, 8, ["avx"], ["avx512"],
    halfs => |hi, lo| { i32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i32, i32s, i32x4, 4, 4, [], ["avx"],
    halfs => |hi, lo| { i32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { i32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    f32, f32s, f32x16, 4, 16, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { f32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { f32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f32, f32s, f32x8, 4, 8, ["avx"], ["avx512"],
    halfs => |hi, lo| { f32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { f32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f32, f32s, f32x4, 4, 4, [], ["avx"],
    halfs => |hi, lo| { f32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { f32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    u64, u64s, u64x8, 8, 8, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { u64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u64, u64s, u64x4, 8, 4, ["avx"], ["avx512"],
    halfs => |hi, lo| { u64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { u64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    u64, u64s, u64x2, 8, 2, [], ["avx"],
    halfs => |hi, lo| { u64x2::new(hi, lo) },
    interleave => |hi, lo| { u64x2::new(hi, lo) }
}

impl_packed! {
    i64, i64s, i64x8, 8, 8, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { i64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i64, i64s, i64x4, 8, 4, ["avx"], ["avx512"],
    halfs => |hi, lo| { i64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { i64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    i64, i64s, i64x2, 8, 2, [], ["avx"],
    halfs => |hi, lo| { i64x2::new(hi, lo) },
    interleave => |hi, lo| { i64x2::new(hi, lo) }
}

impl_packed! {
    f64, f64s, f64x8, 8, 8, ["avx512"], ["avx1024"],
    halfs => |hi, lo| { f64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { f64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f64, f64s, f64x4, 8, 4, ["avx"], ["avx512"],
    halfs => |hi, lo| { f64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { f64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    f64, f64s, f64x2, 8, 2, [], ["avx"],
    halfs => |hi, lo| { f64x2::new(hi, lo) },
    interleave => |hi, lo| { f64x2::new(hi, lo) }
}
