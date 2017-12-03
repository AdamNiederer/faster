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
    const ELEMENT_SIZE: usize = Self::Scalar::SIZE;
    const WIDTH_BYTES: usize = Self::WIDTH * Self::ELEMENT_SIZE / 8;

    #[inline(always)]
    fn width(&self) -> usize {
        Self::WIDTH
    }

    #[inline(always)]
    fn width_bytes(&self) -> usize {
        Self::WIDTH_BYTES
    }

    #[inline(always)]
    fn element_size(&self) -> usize {
        Self::ELEMENT_SIZE
    }

    fn load(data: &[Self::Scalar], offset: usize) -> Self;
    fn store(self, data: &mut [Self::Scalar], offset: usize);
    fn coalesce(self) -> Self::Scalar;
    fn splat(data: Self::Scalar) -> Self;
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self;
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self;
}

// A type which may be packed into a SIMD vector
pub trait Packable where Self : Sized + Copy {
    const SIZE: usize;
    type Vector : Packed<Scalar = Self>;
}

macro_rules! impl_packed {
    ($el:ty, $pvec:tt, $vec:tt, $sz:expr, $feat:expr, $nfeat:expr,
     halfs => $halfs:expr,
     interleave => $interleave:expr) => (
        #[allow(non_camel_case_types)]
        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        pub type $pvec = $vec;

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl Packable for $el {
            const SIZE: usize = SIMD_SIZE / $sz;
            type Vector = $vec;
        }

        impl Packed for $vec {
            type Scalar = $el;
            const WIDTH: usize = $sz;

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
            fn splat(data: $el) -> Self {
                $vec::splat(data)
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

        // #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        // impl<'a> IntoUnevenPackedRefIterator<'a> for &'a [$el] {
        //     type Iter = UnevenPackedIter<'a, $el>;

        //     #[inline(always)]
        //     fn uneven_simd_iter(&'a self) -> Self::Iter {
        //         UnevenPackedIter {
        //             data: self,
        //             position: 0,
        //         }
        //     }
        // }

        // #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        // impl<'a> IntoUnevenPackedRefMutIterator<'a> for &'a mut [$el] {
        //     type Iter = UnevenPackedIter<'a, $el>;

        //     #[inline(always)]
        //     fn uneven_simd_iter_mut(&'a mut self) -> Self::Iter {
        //         UnevenPackedIter {
        //             data: self,
        //             position: 0,
        //         }
        //     }
        // }

    );
}

impl_packed! {
    u8, u8s, u8x64, 64, "avx512", "avx1024",
    halfs => |hi, lo| { u8x64::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x64::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u8, u8s, u8x32, 32, "avx2", "avx512",
    halfs => |hi, lo| { u8x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u8, u8s, u8x16, 16, "sse", "avx2",
    halfs => |hi, lo| { u8x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u8x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    i8, i8s, i8x64, 64, "avx512", "avx1024",
    halfs => |hi, lo| { i8x64::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x64::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i8, i8s, i8x32, 32, "avx2", "avx512",
    halfs => |hi, lo| { i8x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i8, i8s, i8x16, 16, "sse", "avx2",
    halfs => |hi, lo| { i8x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i8x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    u16, u16s, u16x32, 32, "avx512", "avx1024",
    halfs => |hi, lo| { u16x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u16, u16s, u16x16, 16, "avx2", "avx512",
    halfs => |hi, lo| { u16x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u16, u16s, u16x8, 8, "sse", "avx2",
    halfs => |hi, lo| { u16x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u16x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    i16, i16s, i16x32, 32, "avx512", "avx1024",
    halfs => |hi, lo| { i16x32::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x32::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i16, i16s, i16x16, 16, "avx2", "avx512",
    halfs => |hi, lo| { i16x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i16, i16s, i16x8, 8, "sse", "avx2",
    halfs => |hi, lo| { i16x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i16x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}

impl_packed! {
    u32, u32s, u32x16, 16, "avx512", "avx1024",
    halfs => |hi, lo| { u32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { u32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u32, u32s, u32x8, 8, "avx2", "avx512",
    halfs => |hi, lo| { u32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u32, u32s, u32x4, 4, "sse", "avx2",
    halfs => |hi, lo| { u32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { u32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    i32, i32s, i32x16, 16, "avx512", "avx1024",
    halfs => |hi, lo| { i32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { i32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i32, i32s, i32x8, 8, "avx2", "avx512",
    halfs => |hi, lo| { i32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i32, i32s, i32x4, 4, "sse", "avx2",
    halfs => |hi, lo| { i32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { i32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    f32, f32s, f32x16, 16, "avx512", "avx1024",
    halfs => |hi, lo| { f32x16::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo) },
    interleave => |hi, lo| { f32x16::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f32, f32s, f32x8, 8, "avx2", "avx512",
    halfs => |hi, lo| { f32x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { f32x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f32, f32s, f32x4, 4, "sse", "avx2",
    halfs => |hi, lo| { f32x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { f32x4::new(hi, lo, hi, lo) }
}

impl_packed! {
    u64, u64s, u64x8, 8, "avx512", "avx1024",
    halfs => |hi, lo| { u64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { u64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    u64, u64s, u64x4, 4, "avx2", "avx512",
    halfs => |hi, lo| { u64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { u64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    u64, u64s, u64x2, 2, "sse", "avx2",
    halfs => |hi, lo| { u64x2::new(hi, lo) },
    interleave => |hi, lo| { u64x2::new(hi, lo) }
}

impl_packed! {
    i64, i64s, i64x8, 8, "avx512", "avx1024",
    halfs => |hi, lo| { i64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { i64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    i64, i64s, i64x4, 4, "avx2", "avx512",
    halfs => |hi, lo| { i64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { i64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    i64, i64s, i64x2, 2, "sse", "avx2",
    halfs => |hi, lo| { i64x2::new(hi, lo) },
    interleave => |hi, lo| { i64x2::new(hi, lo) }
}

impl_packed! {
    f64, f64s, f64x8, 8, "avx512", "avx1024",
    halfs => |hi, lo| { f64x8::new(hi, hi, hi, hi, lo, lo, lo, lo) },
    interleave => |hi, lo| { f64x8::new(hi, lo, hi, lo, hi, lo, hi, lo) }
}
impl_packed! {
    f64, f64s, f64x4, 4, "avx2", "avx512",
    halfs => |hi, lo| { f64x4::new(hi, hi, lo, lo) },
    interleave => |hi, lo| { f64x4::new(hi, lo, hi, lo) }
}
impl_packed! {
    f64, f64s, f64x2, 2, "sse", "avx2",
    halfs => |hi, lo| { f64x2::new(hi, lo) },
    interleave => |hi, lo| { f64x2::new(hi, lo) }
}

// Impl for scalar traits

macro_rules! impl_packed_shim {
    ($el:ty, $pvec:tt, $vec:tt, $sz:expr, $feat:expr, $nfeat:expr) => (
        #[allow(non_camel_case_types)]
        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        pub type $pvec = $vec;

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl Packable for $el {
            const SIZE: usize = SIMD_SIZE / $sz;
            type Vector = $vec;
        }

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
            fn halfs(hi: $el, lo: $el) -> Self {
                eprintln!("Halfs is going away in 0.3.0! Stay away!");
                hi
            }
            #[inline(always)]
            fn interleave(hi: $el, lo: $el) -> Self {
                eprintln!("Interleave is going away in 0.3.0! Stay away!");
                hi
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
