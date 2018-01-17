// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The SIMD library for humans.

//! Faster allows convenient application of explicit SIMD to existing code. It
//! allows you to write explicit SIMD code once and compile it for any target,
//! regardless of architecture, SIMD capability, or age.

//! # SIMD Iterators
//!
//! SIMD iterators are formed using [`simd_iter`], [`simd_iter_mut`], and
//! [`into_simd_iter`], which return types which allow the usage of the
//! [`simd_map`] and [`simd_reduce`] functions. These functions automatically
//! pack your iterator's data into SIMD vectors and allow you to transparently
//! operate on them in a closure.
//!
//! [`simd_iter`]: iters/trait.IntoPackedIterator.html#tymethod.into_simd_iter
//! [`simd_iter_mut`]: iters/trait.IntoPackedRefIterator.html#tymethod.simd_iter
//! [`into_simd_iter`]: iters/trait.IntoPackedRefMutIterator.html#tymethod.simd_iter_mut
//! [`simd_map`]: iters/trait.PackedIterator.html#tymethod.simd_map
//! [`simd_reduce`]: iters/trait.PackedIterator.html#tymethod.simd_reduce
//!
//! # SIMD Polyfills
//!
//! Once your data is packed into a SIMD vector, you may perform many common
//! SIMD operations on it. These operations have names and behavior independent
//! of any vendor-specific ISA, and have non-SIMD polyfills for machines which
//! cannot perform these operations in a single cycle. See the [`intrin`] module
//! for all available operations.
//!
//! [`intrin`]: intrin/index.html
//!
//! # Examples
//!
//! Faster is currently capable of mapping and reductive operations in SIMD.
//!
//! ## Mapping
//!
//! The simplest example of a computation with `faster` is a single map
//! operation.
//!
//! ```
//! extern crate faster;
//! use faster::*;
//!
//! # fn main() {
//! let lots_of_10s = (&[-10i8; 3000][..]).simd_iter()
//!    .simd_map(i8s::splat(0), |v| v.abs())
//!    .scalar_collect();
//! assert_eq!(lots_of_10s, vec![10u8; 3000]);
//! # }
//! ```
//!
//! In this example, a vector of type [`i8s`] is passed into the closure. The
//! exact type of [`i8s`] is dependent on compilation target, but it will always
//! implement the same operations. Because taking the absolute value of a vector
//! converts it to [`u8s`], the closure will return [`u8s`].
//!
//! [`scalar_collect`] takes the iterator of [`u8s`] and converts it into a
//! `Vec<u8>`.
//!
//! [`i8s`]: vecs/type.i8s.html
//! [`u8s`]: vecs/type.u8s.html
//! [`scalar_collect`]: iters/trait.IntoScalar.html#tymethod.scalar_collect
//!
//! ## Reduction
//!
//! Faster can perform reductive operations with similar power to mapping
//! operations:
//!
//! ```
//! extern crate faster;
//! use faster::*;
//!
//! # fn main() {
//! let two_hundred = (&[2.0f32; 100][..]).simd_iter()
//!    .simd_reduce(f32s::splat(0.0), f32s::splat(0.0), |acc, v| acc + v)
//!    .sum();
//! assert_eq!(two_hundred, 200.0f32);
//! # }
//! ```
//!
//! This example sums every number in the collection. The first parameter to
//! simd_reduce is the default value of the accumulator, just like any
//! other reduction. The second value is used if the collection being reduced
//! over doesn't fit evenly into your system's vectors - it is the default value
//! of the last vector, and each element of the vector is used only if it isn't
//! filled by an element of the collection. Typically, a value of 0 or 1 is a
//! suitable default.
//!
//! Minding portability is very important when performing reductive
//! operations. See below for some tips on keeping your code portable across all
//! architectures.
//!
//! # Portability
//!
//! While `faster` does most of the work ensuring your code stays portable
//! across platforms, a user of this library must still understand that it is
//! very possible to write non-portable algorithms using this library. Anything
//! which relies on vector width, anything which is impure, and anything which
//! uses constants in reductive operations is inherently nonportable. Some
//! examples below:
//!
//! ```
//! extern crate faster;
//! use faster::*;
//!
//! # fn main() {
//! let mut flip = true;
//! let impure = (&[1i8; 3000][..]).simd_iter()
//!    .simd_map(i8s::splat(0), |v| { flip = !flip; if flip { v + i8s::splat(1) } else { v } })
//!    .scalar_collect();
//! // Depending on the width of your target's SIMD vectors, `impure` could be
//! // [1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, ...] or
//! // [1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, ...], etc.
//! # }
//! ```
//!
//! ```
//! extern crate faster;
//! use faster::*;
//!
//! # fn main() {
//! let length_dependent = (&[0i8; 10][..]).simd_iter()
//!    .simd_reduce(i8s::splat(0), i8s::splat(0), |acc, v| acc + v + i8s::splat(1)).sum();
//! // `length_dependent` could be a different number on a different target!
//! # }
//! ```
//!
//! As a precaution, it is best practice to keep all functions pure, and only
//! operate on SIMD vectors in your SIMD-enabled closures unless you know
//! exactly what is happening under the hood. It's also important to remember
//! that these problems will crop up even if you only support x86; the width
//! difference between AVX and SSE is the primary source of these issues!

#![cfg_attr(feature = "no-std", no_std)]
#![feature(cfg_target_feature)]
#![feature(specialization)]
#![feature(const_fn)]
#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(inclusive_range))]

mod core_or_std {
    #[cfg(not(feature = "no-std"))] pub use ::std::*;
    #[cfg(feature = "no-std")] pub use ::core::*;
}

#[cfg(test)] extern crate test;

extern crate stdsimd;

pub mod vecs;
pub mod vec_patterns;
pub mod iters;
pub mod intrin;
pub mod prelude;

mod shimvecs;

pub use prelude::*;

#[macro_use] pub mod zip;
pub mod swizzle;

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_nop_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[0u8; 1024][..]).simd_iter().simd_map(u8s::splat(0), |v| v).scalar_collect())
        });
    }

    #[bench]
    fn bench_nop_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[0u8; 1024][..]).iter().map(|e| *e).collect::<Vec<u8>>())
        });
    }

    #[bench]
    fn bench_map_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).simd_iter()
                    .simd_map(f32s::splat(0.0), |v| {
                        f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()
                            - f32s::splat(4.0) - f32s::splat(2.0) })
                    .scalar_collect())
        })
    }

    #[bench]
    fn bench_map_uneven_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1025][..]).simd_iter()
                    .simd_map(f32s::splat(0.0), |v| {
                        f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()
                            - f32s::splat(4.0) - f32s::splat(2.0) })
                    .scalar_collect())
        })
    }

    #[bench]
    fn bench_map_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).iter()
                    .map(|v| { 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() -
                               4.0 - 2.0 })
                    .collect::<Vec<f32>>())
        });
    }

    #[bench]
    fn bench_reduce_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).simd_iter()
                    .simd_reduce(f32s::splat(0.0), f32s::splat(0.0), |a, v| a + f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()).sum())
        })
    }

    #[bench]
    fn bench_reduce_uneven_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1025][..]).simd_iter()
                    .simd_reduce(f32s::splat(0.0), f32s::splat(0.0), |a, v| a + f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()).sum())
        })
    }

    #[bench]
    fn bench_reduce_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).iter()
                    .fold(0.0, |a, v| a + 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt()))
        })
    }

    #[bench]
    fn bench_determinant3_simd(b: &mut Bencher) {
        // TODO: Why is this so slow? Cache locality?
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1026][..]).simd_iter().stripe_nine().zip()
                    .simd_map(tuplify!(9, f32s::splat(0.0)), |(a, b, c, d, e, f, g, h, i)| {
                        (a * e * i) + (b * f * g) + (c * d * h) - (c * e * g) - (b * d * i) - (a * f * h)
                    })
                    .scalar_collect())
        })
    }

    #[bench]
    fn bench_determinant3_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1026][..]).chunks(9).map(|m| {
                    (m[0] * m[4] * m[8]) + (m[1] * m[5] * m[6]) + (m[2] * m[3] * m[7]) - (m[2] * m[4] * m[6]) - (m[1] * m[3] * m[8]) - (m[0] * m[5] * m[7])
                }).collect::<Vec<f32>>())
        })
    }

    #[bench]
    fn bench_determinant2_simd(b: &mut Bencher) {
        // TODO: Why is this so slow? Cache locality?
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).simd_iter().stripe_four().zip()
                    .simd_map(tuplify!(4, f32s::splat(0.0)), |(a, b, c, d)| {
                        a * d - b * c
                    })
                    .scalar_collect())
        })
    }

    #[bench]
    fn bench_determinant2_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).chunks(4).map(|m| {
                    m[0] * m[3] - m[1] * m[2]
                }).collect::<Vec<f32>>())
        })
    }

    #[bench]
    fn bench_zip_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123i32; 1024][..]).simd_iter().stripe_two().zip()
                    .simd_map(tuplify!(2, i32s::splat(0)), |(a, b)| {
                        let (aa, ab): (i64s, i64s) = a.upcast();
                        let (ba, bb): (i64s, i64s) = b.upcast();
                        (aa.abs() + ba.abs()).saturating_downcast(ab.abs() + bb.abs())
                    })
                    .scalar_collect())
        })
    }

    #[bench]
    fn bench_zip_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123i32; 1024][..]).chunks(2).map(|a| {
                    ((a[0] as f64).abs() + (a[1] as f64).abs()) as f32
                }).collect::<Vec<f32>>())
        })
    }

    #[bench]
    fn bench_zip_nop_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).simd_iter().stripe_two().zip()
                    .simd_map(tuplify!(2, f32s::splat(0.0)), |(a, b)| {
                        a + b
                    })
                    .scalar_collect())
        })
    }

    #[bench]
    fn bench_zip_nop_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).chunks(2).map(|a| {
                    a[0] + a[1]
                }).collect::<Vec<f32>>())
        })
    }
}
