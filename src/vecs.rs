// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::simd::*;
use iters::{IntoPackedRefIterator, IntoPackedRefMutIterator, PackedIter};

#[cfg(all(target_feature = "avx512"))]
pub const SIMD_SIZE: usize = 512;

#[cfg(all(target_feature = "avx2", not(target_feature="avx512")))]
pub const SIMD_SIZE: usize = 256;

#[cfg(all(target_feature = "sse", not(target_feature="avx2")))]
pub const SIMD_SIZE: usize = 128;

// A SIMD vector containing T.
pub trait Packed : Sized {
    type Scalar : Packable;

    const WIDTH: usize;

    #[inline(always)]
    fn width(&self) -> usize {
        Self::WIDTH
    }

    fn load(data: &[Self::Scalar], offset: usize) -> Self;
    fn store(self, data: &mut [Self::Scalar], offset: usize);
    fn coalesce(self) -> Self::Scalar;
    fn splat(data: Self::Scalar) -> Self;
    fn default() -> Self;
    fn extract(&self, idx: usize) -> Self::Scalar;
    fn replace(&mut self, idx: usize, data: Self::Scalar) -> Self;
    #[deprecated(since="0.2.0", note="This method leads to unportable code")]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self;
    #[deprecated(since="0.2.0", note="This method leads to unportable code")]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self;
    fn sum(&self) -> Self::Scalar;
    fn product(&self) -> Self::Scalar;
}

// A type which may be packed into a SIMD vector
pub trait Packable where Self : Sized + Copy {
    type Vector : Packed<Scalar = Self>;
    const SIZE: usize;
}

macro_rules! impl_packed {
    ($el:tt, $pvec:tt, $vec:tt, $sz:expr, $width:expr, $feat:expr, $nfeat:expr,
     halfs => $halfs:expr,
     interleave => $interleave:expr) => (
        #[allow(non_camel_case_types)]
        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        pub type $pvec = $vec;

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl Packable for $el {
            type Vector = $vec;
            const SIZE: usize = $sz;
        }

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
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
                $vec::replace(*self, idx as u32, data);
                *self
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
                eprintln!("Halfs is going away in 0.3.0! Stay away!");
                $halfs(hi, lo)
            }
            #[inline(always)]
            fn interleave(hi: $el, lo: $el) -> Self {
                eprintln!("Interleave is going away in 0.3.0! Stay away!");
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

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
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

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
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
    u8, u8s, u8x64, 1, 64, "avx512", "avx1024",
    halfs => |hi, lo| { u8x64::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x64::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u8, u8s, u8x32, 1, 32, "avx2", "avx512",
    halfs => |hi, lo| { u8x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u8, u8s, u8x16, 1, 16, "sse", "avx2",
    halfs => |hi, lo| { u8x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    i8, i8s, i8x64, 1, 64, "avx512", "avx1024",
    halfs => |hi, lo| { i8x64::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x64::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i8, i8s, i8x32, 1, 32, "avx2", "avx512",
    halfs => |hi, lo| { i8x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i8, i8s, i8x16, 1, 16, "sse", "avx2",
    halfs => |hi, lo| { i8x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    u16, u16s, u16x32, 2, 32, "avx512", "avx1024",
    halfs => |hi, lo| { u16x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u16, u16s, u16x16, 2, 2, "avx2", "avx512",
    halfs => |hi, lo| { u16x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u16, u16s, u16x8, 2, 8, "sse", "avx2",
    halfs => |hi, lo| { u16x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    i16, i16s, i16x32, 2, 32, "avx512", "avx1024",
    halfs => |hi, lo| { i16x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i16, i16s, i16x16, 2, 2, "avx2", "avx512",
    halfs => |hi, lo| { i16x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i16, i16s, i16x8, 2, 8, "sse", "avx2",
    halfs => |hi, lo| { i16x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    u32, u32s, u32x16, 4, 16, "avx512", "avx1024",
    halfs => |hi, lo| { u32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u32, u32s, u32x8, 4, 8, "avx2", "avx512",
    halfs => |hi, lo| { u32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u32, u32s, u32x4, 4, 4, "sse", "avx2",
    halfs => |hi, lo| { u32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { u32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    i32, i32s, i32x16, 4, 16, "avx512", "avx1024",
    halfs => |hi, lo| { i32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i32, i32s, i32x8, 4, 8, "avx2", "avx512",
    halfs => |hi, lo| { i32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i32, i32s, i32x4, 4, 4, "sse", "avx2",
    halfs => |hi, lo| { i32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { i32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    f32, f32s, f32x16, 4, 16, "avx512", "avx1024",
    halfs => |hi, lo| { f32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { f32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f32, f32s, f32x8, 4, 8, "avx2", "avx512",
    halfs => |hi, lo| { f32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { f32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f32, f32s, f32x4, 4, 4, "sse", "avx2",
    halfs => |hi, lo| { f32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { f32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    u64, u64s, u64x8, 8, 8, "avx512", "avx1024",
    halfs => |hi, lo| { u64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u64, u64s, u64x4, 8, 4, "avx2", "avx512",
    halfs => |hi, lo| { u64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { u64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    u64, u64s, u64x2, 8, 2, "sse", "avx2",
    halfs => |hi, lo| { u64x2::new(hi, lo) },
    interleave => |hi, lo| { u64x2::new(hi, lo) }
}

impl_packed! {
    i64, i64s, i64x8, 8, 8, "avx512", "avx1024",
    halfs => |hi, lo| { i64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i64, i64s, i64x4, 8, 4, "avx2", "avx512",
    halfs => |hi, lo| { i64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { i64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    i64, i64s, i64x2, 8, 2, "sse", "avx2",
    halfs => |hi, lo| { i64x2::new(hi, lo) },
    interleave => |hi, lo| { i64x2::new(hi, lo) }
}

impl_packed! {
    f64, f64s, f64x8, 8, 8, "avx512", "avx1024",
    halfs => |hi, lo| { f64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { f64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f64, f64s, f64x4, 8, 4, "avx2", "avx512",
    halfs => |hi, lo| { f64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { f64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    f64, f64s, f64x2, 8, 2, "sse", "avx2",
    halfs => |hi, lo| { f64x2::new(hi, lo) },
    interleave => |hi, lo| { f64x2::new(hi, lo) }
}

// Impl for scalar traits

macro_rules! impl_packed_shim {
    ($el:ty, $pvec:tt, $vec:tt, $sz:expr, $feat:expr, $nfeat:expr) => (
        #[allow(non_camel_case_types)]
        #[cfg(all(not(target_feature = $nfeat)))]
        pub type $pvec = $vec;

        #[cfg(all(not(target_feature = $nfeat)))]
        impl Packable for $el {
            type Vector = $vec;
        }

        #[cfg(all(not(target_feature = $nfeat)))]
        impl Packed for $vec {
            type Scalar = $el;
            const WIDTH: usize = $sz;

            #[inline(always)]
            fn load(data: &[$el], offset: usize) -> $vec {
                data[offset]
            }
            #[inline(always)]
            fn store(self, data: &mut [$el], offset: usize) {
                data[offset] = self;
            }
            fn coalesce(self) -> Self::Scalar {
                self
            }
            #[inline(always)]
            fn splat(data: $el) -> Self {
                data
            }
            #[inline(always)]
            #[allow(unused_variables)]
            fn halfs(hi: $el, lo: $el) -> Self {
                eprintln!("Halfs is going away in 0.3.0! Stay away!");
                hi
            }
            #[inline(always)]
            #[allow(unused_variables)]
            fn interleave(hi: $el, lo: $el) -> Self {
                eprintln!("Interleave is going away in 0.3.0! Stay away!");
                hi
            }
            #[inline(always)]
            fn product(&self) -> Self::Scalar {
                *self
            }
            #[inline(always)]
            fn sum(&self) -> Self::Scalar {
                *self
            }
        }

        #[cfg(all(not(target_feature = $nfeat)))]
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

        #[cfg(all(not(target_feature = $nfeat)))]
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

impl_packed_shim!(u8, u8s, u8, 1, "", "sse");
impl_packed_shim!(i8, i8s, i8, 1, "", "sse");
impl_packed_shim!(u16, u16s, u16, 1, "", "sse");
impl_packed_shim!(i16, i16s, i16, 1, "", "sse");
impl_packed_shim!(u32, u32s, u32, 1, "", "sse");
impl_packed_shim!(i32, i32s, i32, 1, "", "sse");
impl_packed_shim!(f32, f32s, f32, 1, "", "sse");
impl_packed_shim!(u64, u64s, u64, 1, "", "sse");
impl_packed_shim!(i64, i64s, i64, 1, "", "sse");
impl_packed_shim!(f64, f64s, f64, 1, "", "sse");
