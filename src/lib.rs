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
//! [`simd_iter`]: iters/trait.IntoSIMDIterator.html#tymethod.into_simd_iter
//! [`simd_iter_mut`]: iters/trait.IntoSIMDIterator.html#tymethod.simd_iter
//! [`into_simd_iter`]: iters/trait.IntoSIMDRefMutIterator.html#tymethod.simd_iter_mut
//! [`simd_map`]: iters/trait.SIMDIterator.html#tymethod.simd_map
//! [`simd_reduce`]: iters/trait.SIMDIterator.html#tymethod.simd_reduce
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
//! # #[cfg(feature = "no-std")]
//! # fn main() { }
//!
//! # #[cfg(not(feature = "no-std"))]
//! # fn main() {
//! let lots_of_10s = [-10i8; 3000].simd_iter(i8s(0))
//!    .simd_map(|v| v.abs())
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
//! let two_hundred = [2.0f32; 100].simd_iter(f32s(0.0))
//!    .simd_reduce(f32s(0.0), |acc, v| acc + v)
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
//! ## Multiple collections
//!
//! Faster supports vectorized lockstep iteration over multiple collections.
//! Simply [`zip`] them up, and proceed as normal.
//!
//! [`zip`]: zip/trait.IntoSIMDZip.html
//!
//! ```
//! extern crate faster;
//! use faster::*;
//!
//! # #[cfg(feature = "no-std")]
//! # fn main() { }
//!
//! # #[cfg(not(feature = "no-std"))]
//! # fn main() {
//! let sevens = ([4i32; 200].simd_iter(i32s(0)), [3i32; 200].simd_iter(i32s(0)))
//!     .zip()
//!     .simd_map(|(a, b)| a + b)
//!     .scalar_collect();
//! # }
//! ```
//!
//! ## Striping Collections
//!
//! Reading every nth element of a collection can be vectorized on most
//! machines. Simply call [`stride`], or one of the slightly-faster tuple-based
//! functions, such as [`stride_two`].
//!
//! [`stride`]: iters/struct.SIMDRefIter.html#method.stride
//! [`stride_two`]: iters/struct.SIMDRefIter.html#method.stride_two
//!
//! ```
//! extern crate faster;
//! use faster::*;
//!
//! # #[cfg(feature = "no-std")]
//! # fn main() { }
//!
//! # #[cfg(not(feature = "no-std"))]
//! # fn main() {
//!     // Computes the determinant of matrices arranged as [a, b, c, d, a, b, c...]
//!     let slice: &[f32] = &[1.0f32; 1024];
//!     let determinant = slice.stride_four(tuplify!(4, f32s(0.0))).zip()
//!         .simd_map(|(a, b, c, d)| a * d - b * c)
//!         .scalar_collect();
//! # }
//! ```
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
//! # #[cfg(feature = "no-std")]
//! # fn main() { }
//!
//! # #[cfg(not(feature = "no-std"))]
//! # fn main() {
//! let mut flip = true;
//! let impure = [1i8; 3000].simd_iter(i8s(0))
//!    .simd_map(|v| { flip = !flip; if flip { v + i8s(1) } else { v } })
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
//! let length_dependent = [0i8; 10].simd_iter(i8s(0))
//!    .simd_reduce(i8s(0), |acc, v| acc + v + i8s(1)).sum();
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
#![cfg_attr(test, feature(test))]
#![feature(rust_2018_preview, stdsimd)]

#[cfg(feature = "no-std")]
pub use ::core as std;

#[cfg(test)] extern crate test;
extern crate vektor;

mod shimvecs;

#[macro_use] pub mod zip;
pub mod vecs;
pub mod vec_patterns;
pub mod iters;
pub mod into_iters;
pub mod intrin;
pub mod prelude;
pub mod stride_zip;
pub mod stride;

pub use crate::prelude::*;

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use test::{Bencher, black_box};

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn nop_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [0u8; 1024].simd_iter(u8s(0)).simd_map(|v| v).scalar_collect())
        });
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn nop_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [0u8; 1024].iter().map(|e| *e).collect::<Vec<u8>>())
        });
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn map_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024].simd_iter(f32s(0.0)).simd_map(|v| {
                    f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0)
                }).scalar_collect())
        })
    }

    #[bench]
    fn for_fill_simd(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).enumerate() {
                let ans = f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0);
                ans.put(&mut out, i * ans.width());
            }
            out
        })
    }

    #[bench]
    fn for_unrolled_simd(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).unroll(8).enumerate() {
                macro_rules! compute {
                    ($($idx:expr),*) => {
                        $(
                            let a = f32s(9.0) * v[$idx].abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0);
                            a.put(&mut out, (i + $idx) * a.width());
                        )*
                    }
                }
                compute!(0, 1, 2, 3, 4, 5, 6, 7);
            }
            out
        })
    }

    #[bench]
    fn high_latency(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            [-123.456f32; 1024].simd_iter(f32s(0.0)).simd_map(|v| {
                let (a, b) = (v * f32s(1.20)).upcast();
                (a.sqrt() * f64s(3.141592653589793)).saturating_downcast(
                    b.sqrt() * f64s(3.141592653589793))
            }).scalar_fill(&mut out);
        });
    }

    #[bench]
    fn high_latency_for(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).enumerate() {
                let (a, b) = (v * f32s(1.20)).upcast();
                let ans = (a.sqrt() * f64s(3.141592653589793)).saturating_downcast(
                    b.sqrt() * f64s(3.141592653589793));
                ans.put(&mut out, i * ans.width());
            }
        })
    }

    #[bench]
    fn high_latency_unrolled(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).unroll(8).enumerate() {
                macro_rules! compute {
                    ($($idx:expr),*) => {
                        $(
                            let (a, b) = (v[$idx] * f32s(1.20)).upcast();
                            let ans = (a.sqrt() * f64s(3.141592653589793)).saturating_downcast(
                                b.sqrt() * f64s(3.141592653589793));
                            ans.put(&mut out, (i + $idx) * ans.width());
                        )*
                    }
                }
                compute!(0, 1, 2, 3, 4, 5, 6, 7);
            }
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn for_simd(b: &mut Bencher) {
        b.iter(|| {
            let mut out = vec![0f32; 1024];
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).enumerate() {
                let ans = f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0);
                ans.put(&mut out, i * ans.width());
            }
            out
        })
    }


    // #[bench]
    // fn map_eager_simd(b: &mut Bencher) {
    //     let mut into = [0f32; 1024];
    //     b.iter(|| {
    //         black_box(
    //             [-123.456f32; 1024].simd_iter()
    //                 .simd_map_into(&mut into, f32s(0.0), |v| {
    //                     f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()
    //                         - f32s(4.0) - f32s(2.0)
    //                 }));
    //     })
    // }

    #[bench]
    fn map_fill_simd(b: &mut Bencher) {
        let mut into = [0f32; 1024];
        b.iter(|| {
            black_box(
                [-123.456f32; 1024].simd_iter(f32s(0.0)).simd_map(|v| {
                    f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0)
                }).scalar_fill(&mut into));
        })
    }


    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn map_uneven_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1025].simd_iter(f32s(0.0)).simd_map(|v| {
                    f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0)
                }).scalar_collect())
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn map_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024].iter()
                    .map(|v| { 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() -
                               4.0 - 2.0 })
                    .collect::<Vec<f32>>())
        });
    }

    #[bench]
    fn reduce_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024].simd_iter(f32s(0.0))
                    .simd_reduce(f32s(0.0), |a, v| a + f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()).sum())
        })
    }

    #[bench]
    fn reduce_uneven_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1025].simd_iter(f32s(0.0))
                    .simd_reduce(f32s(0.0), |a, v| a + f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()).sum())
        })
    }

    #[bench]
    fn reduce_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024].iter()
                    .fold(0.0, |a, v| a + 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt()))
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn determinant3_simd(b: &mut Bencher) {
        // TODO: Why is this so slow? Cache locality?
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1026][..]).stride_nine(tuplify!(9, f32s(0.0))).zip()
                    .simd_map(|(a, b, c, d, e, f, g, h, i)| {
                        (a * e * i) + (b * f * g) + (c * d * h) - (c * e * g) - (b * d * i) - (a * f * h)
                    })
                    .scalar_collect())
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn determinant3_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1026].chunks(9).map(|m| {
                    (m[0] * m[4] * m[8]) + (m[1] * m[5] * m[6]) + (m[2] * m[3] * m[7]) - (m[2] * m[4] * m[6]) - (m[1] * m[3] * m[8]) - (m[0] * m[5] * m[7])
                }).collect::<Vec<f32>>())
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn determinant2_simd(b: &mut Bencher) {
        // TODO: Why is this so slow? Cache locality?
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).stride_four(tuplify!(4, f32s(0.0))).zip()
                    .simd_map(|(a, b, c, d)| {
                        a * d - b * c
                    }).scalar_collect())
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn determinant2_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024].chunks(4).map(|m| {
                    m[0] * m[3] - m[1] * m[2]
                }).collect::<Vec<f32>>())
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn zip_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123i32; 1024][..]).stride_two(tuplify!(2, i32s(0))).zip()
                    .simd_map(|(a, b)| {
                        let (aa, ab): (i64s, i64s) = a.upcast();
                        let (ba, bb): (i64s, i64s) = b.upcast();
                        (aa.abs() + ba.abs()).saturating_downcast(ab.abs() + bb.abs())
                    }).scalar_collect())
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn zip_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123i32; 1024].chunks(2).map(|a| {
                    ((a[0] as f64).abs() + (a[1] as f64).abs()) as f32
                }).collect::<Vec<f32>>())
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn zip_nop_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..]).stride_two(tuplify!(2, f32s(0.0))).zip()
                    .simd_map(|(a, b)| a + b)
                    .scalar_collect())
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn zip_nop_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024].chunks(2).map(|a| {
                    a[0] + a[1]
                }).collect::<Vec<f32>>())
        })
    }

    #[bench]
    fn base100_enc_simd(b: &mut Bencher) {
        let mut out = [0u8; 4096];
        b.iter(|| {
            let mut i = 0;
            [123u8; 1024].simd_iter(u8s(0)).simd_do_each(|v| {
                let (a, b): (u16s, u16s) = v.upcast();
                let third = ((a + u16s(55)) / u16s(64) + u16s(143)).saturating_downcast((b + u16s(55)) / u16s(64) + u16s(143));
                let fourth = ((v + u8s(55)) & u8s(0x3f)) + u8s(128);

                // Make some room for interleaving
                let (ta, tb) = third.upcast();
                let (fa, fb) = fourth.upcast();

                // Interleave third and fourth bytes
                let third_fourth_a = ta.swap_bytes().merge_interleaved(fa);
                let third_fourth_b = tb.swap_bytes().merge_interleaved(fb);

                // Make some more room for another interleaving
                let (tfa, tfb) = third_fourth_a.be_u16s().upcast();
                let (tfc, tfd) = third_fourth_b.be_u16s().upcast();

                // Interleave a constant 0xf09f with the third and fourth bytes,
                // and store into out buffer
                u32s(0xf09f0000).merge_interleaved(tfa).be_u8s().put(&mut out, i);
                u32s(0xf09f0000).merge_interleaved(tfb).be_u8s().put(&mut out, i + v.width());
                u32s(0xf09f0000).merge_interleaved(tfc).be_u8s().put(&mut out, i + v.width() * 2);
                u32s(0xf09f0000).merge_interleaved(tfd).be_u8s().put(&mut out, i + v.width() * 3);
                i += v.width() * 4;
            });
            out
        })
    }

    #[bench]
    fn base100_enc_scalar(b: &mut Bencher) {
        let mut out = [0u8; 4096];
        b.iter(|| {
            for (i, ch) in [123u8; 1024].iter().enumerate() {
                out[4 * i + 0] = 0xf0;
                out[4 * i + 1] = 0x9f;
                out[4 * i + 2] = ((((*ch as u16).wrapping_add(55)) >> 6) + 143) as u8;
                out[4 * i + 3] = (ch.wrapping_add(55) & 0x3f).wrapping_add(128);
            }
            out
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn stride_zip_naive(b: &mut Bencher) {
        let a = [0u8; 4096];
        b.iter(|| {
            (&a[..]).stride_two(tuplify!(2, u8s(0))).zip()
                .simd_map(|(a, b)| a + b)
                .scalar_collect()
        })
    }

    #[bench]
    #[cfg(not(feature = "no-std"))]
    fn stride_zip(b: &mut Bencher) {
        let a = [0u8; 4096];
        b.iter(|| {
            (&a[..]).simd_iter(u8s(0)).stride_zip()
                .simd_map(|(a, b)| a + b)
                .scalar_collect()
        })
    }
}
