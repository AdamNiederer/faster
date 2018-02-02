// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};

pub trait Round {
    /// Return a vector with all elements of `self` rounded to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).round(), f32s(3.0));
    /// assert_eq!(f32s(2.4).round(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).round(), f32s(-3.0));
    /// assert_eq!(f32s(-2.4).round(), f32s(-2.0));
    /// # }
    /// ```
    fn round(&self) -> Self;

    /// Return a vector with all elements of `self` rounded up to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).ceil(), f32s(3.0));
    /// assert_eq!(f32s(2.4).ceil(), f32s(3.0));
    /// assert_eq!(f32s(-2.7).ceil(), f32s(-2.0));
    /// assert_eq!(f32s(-2.4).ceil(), f32s(-2.0));
    /// # }
    /// ```
    fn ceil(&self) -> Self;

    /// Return a vector with all elements of `self` rounded down to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).floor(), f32s(2.0));
    /// assert_eq!(f32s(2.4).floor(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).floor(), f32s(-3.0));
    /// assert_eq!(f32s(-2.4).floor(), f32s(-3.0));
    /// # }
    /// ```
    fn floor(&self) -> Self;

    /// Return a vector with all elements of `self` truncated. Effectively
    /// rounds the elements towards zero.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).trunc(), f32s(2.0));
    /// assert_eq!(f32s(2.4).trunc(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).trunc(), f32s(-2.0));
    /// assert_eq!(f32s(-2.4).trunc(), f32s(-2.0));
    /// # }
    /// ```
    fn trunc(&self) -> Self;
}

rust_fallback_impl! {
    impl Round for f32x4 where "sse4.1" {
        round => _mm_round_ps(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3];
        ceil => _mm_ceil_ps(), [0, 1, 2, 3];
        floor => _mm_floor_ps(), [0, 1, 2, 3];
        trunc => _mm_round_ps(_MM_FROUND_TRUNC), [0, 1, 2, 3];
    }
}

rust_fallback_impl! {
    impl Round for f64x2 where "sse4.1" {
        round => _mm_round_pd(_MM_FROUND_TO_NEAREST_INT), [0, 1];
        ceil => _mm_ceil_pd(), [0, 1];
        floor => _mm_floor_pd(), [0, 1];
        trunc => _mm_round_pd(_MM_FROUND_TRUNC), [0, 1];
    }
}

rust_fallback_impl! {
    impl Round for f32x8 where "avx" {
        round => _mm256_round_ps(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3, 4, 5, 6, 7];
        ceil => _mm256_ceil_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
        floor => _mm256_floor_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
        trunc => _mm256_round_ps(_MM_FROUND_TRUNC), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl Round for f64x4 where "sse4.1" {
        round => _mm256_round_pd(_MM_FROUND_TO_NEAREST_INT), [0, 1, 2, 3];
        ceil => _mm256_ceil_pd(), [0, 1, 2, 3];
        floor => _mm256_floor_pd(), [0, 1, 2, 3];
        trunc => _mm256_round_pd(_MM_FROUND_TRUNC), [0, 1, 2, 3];
    }
}

#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;

    #[test]
    fn round_f32s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f32s(i), f32s(i).round());
            assert_eq!(f32s(i), f32s(i + 0.4).round());
            assert_eq!(f32s(i + 1.0), f32s(i + 0.8).round());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f32s(i), f32s(i).round());
            assert_eq!(f32s(i), f32s(i - 0.4).round());
            assert_eq!(f32s(i - 1.0), f32s(i - 0.8).round());
            i -= 1.0;
        }

        assert_eq!(f32s(9.8).round(), f32s(10.0));
        assert_eq!(f32s(9.1).round(), f32s(9.0));
        assert_eq!(f32s(9.0).round(), f32s(9.0));
    }

    #[test]
    fn trunc_f32s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f32s(i), f32s(i).trunc());
            assert_eq!(f32s(i), f32s(i + 0.4).trunc());
            assert_eq!(f32s(i), f32s(i + 0.8).trunc());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f32s(i), f32s(i).trunc());
            assert_eq!(f32s(i), f32s(i - 0.4).trunc());
            assert_eq!(f32s(i), f32s(i - 0.8).trunc());
            i -= 1.0;
        }

        assert_eq!(f32s(9.8).trunc(), f32s(9.0));
        assert_eq!(f32s(9.1).trunc(), f32s(9.0));
        assert_eq!(f32s(9.0).trunc(), f32s(9.0));
    }

    #[test]
    fn ceil_f32s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f32s(i), f32s(i).ceil());
            assert_eq!(f32s(i + 1.0), f32s(i + 0.4).ceil());
            assert_eq!(f32s(i + 1.0), f32s(i + 0.8).ceil());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f32s(i), f32s(i).ceil());
            assert_eq!(f32s(i), f32s(i - 0.4).ceil());
            assert_eq!(f32s(i), f32s(i - 0.8).ceil());
            i -= 1.0;
        }

        assert_eq!(f32s(9.8).ceil(), f32s(10.0));
        assert_eq!(f32s(9.1).ceil(), f32s(10.0));
        assert_eq!(f32s(9.0).ceil(), f32s(9.0));
    }

    #[test]
    fn floor_f32s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f32s(i), f32s(i).floor());
            assert_eq!(f32s(i), f32s(i + 0.4).floor());
            assert_eq!(f32s(i), f32s(i + 0.8).floor());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f32s(i), f32s(i).floor());
            assert_eq!(f32s(i - 1.0), f32s(i - 0.4).floor());
            assert_eq!(f32s(i - 1.0), f32s(i - 0.8).floor());
            i -= 1.0;
        }

        assert_eq!(f32s(9.8).floor(), f32s(9.0));
        assert_eq!(f32s(9.1).floor(), f32s(9.0));
        assert_eq!(f32s(9.0).floor(), f32s(9.0));
    }

    #[test]
    fn round_f64s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f64s(i), f64s(i).round());
            assert_eq!(f64s(i), f64s(i + 0.4).round());
            assert_eq!(f64s(i + 1.0), f64s(i + 0.8).round());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f64s(i), f64s(i).round());
            assert_eq!(f64s(i), f64s(i - 0.4).round());
            assert_eq!(f64s(i - 1.0), f64s(i - 0.8).round());
            i -= 1.0;
        }

        assert_eq!(f64s(9.8).round(), f64s(10.0));
        assert_eq!(f64s(9.1).round(), f64s(9.0));
        assert_eq!(f64s(9.0).round(), f64s(9.0));
    }

    #[test]
    fn trunc_f64s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f64s(i), f64s(i).trunc());
            assert_eq!(f64s(i), f64s(i + 0.4).trunc());
            assert_eq!(f64s(i), f64s(i + 0.8).trunc());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f64s(i), f64s(i).trunc());
            assert_eq!(f64s(i), f64s(i - 0.4).trunc());
            assert_eq!(f64s(i), f64s(i - 0.8).trunc());
            i -= 1.0;
        }

        assert_eq!(f64s(9.8).trunc(), f64s(9.0));
        assert_eq!(f64s(9.1).trunc(), f64s(9.0));
        assert_eq!(f64s(9.0).trunc(), f64s(9.0));
    }

    #[test]
    fn ceil_f64s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f64s(i), f64s(i).ceil());
            assert_eq!(f64s(i + 1.0), f64s(i + 0.4).ceil());
            assert_eq!(f64s(i + 1.0), f64s(i + 0.8).ceil());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f64s(i), f64s(i).ceil());
            assert_eq!(f64s(i), f64s(i - 0.4).ceil());
            assert_eq!(f64s(i), f64s(i - 0.8).ceil());
            i -= 1.0;
        }

        assert_eq!(f64s(9.8).ceil(), f64s(10.0));
        assert_eq!(f64s(9.1).ceil(), f64s(10.0));
        assert_eq!(f64s(9.0).ceil(), f64s(9.0));
    }

    #[test]
    fn floor_f64s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f64s(i), f64s(i).floor());
            assert_eq!(f64s(i), f64s(i + 0.4).floor());
            assert_eq!(f64s(i), f64s(i + 0.8).floor());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f64s(i), f64s(i).floor());
            assert_eq!(f64s(i - 1.0), f64s(i - 0.4).floor());
            assert_eq!(f64s(i - 1.0), f64s(i - 0.8).floor());
            i -= 1.0;
        }

        assert_eq!(f64s(9.8).floor(), f64s(9.0));
        assert_eq!(f64s(9.1).floor(), f64s(9.0));
        assert_eq!(f64s(9.0).floor(), f64s(9.0));
    }
}
