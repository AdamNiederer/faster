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
//! # #[cfg(not(feature = "std"))]
//! # fn main() { }
//!
//! # #[cfg(feature = "std")]
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
//! #![feature(stdsimd)]
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
//! # #[cfg(not(feature = "std"))]
//! # fn main() { }
//!
//! # #[cfg(feature = "std")]
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
//! # #[cfg(not(feature = "std"))]
//! # fn main() { }
//!
//! # #[cfg(feature = "std")]
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
//! # #[cfg(not(feature = "std"))]
//! # fn main() { }
//!
//! # #[cfg(feature = "std")]
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

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(test))]
#![feature(stdsimd)]
// , mmx_target_feature, sse4a_target_feautre, tbm_target_feature

mod core {
    #[cfg(not(feature = "std"))]
    pub use core::*;
    #[cfg(feature = "std")]
    pub use std::*;
}

extern crate packed_simd;
extern crate vektor;

#[macro_use]
pub(crate) mod debug;
#[macro_use]
pub mod zip;
#[macro_use]
pub mod vecs;
pub mod into_iters;
pub mod iters;
pub mod vec_patterns;
#[macro_use]
pub mod intrin;
#[macro_use]
pub mod arch;
pub mod prelude;
pub mod stride;
pub mod stride_zip;

pub use crate::prelude::*;
