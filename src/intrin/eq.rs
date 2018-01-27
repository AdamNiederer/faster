// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::*;

pub trait PackedEq : Packed {
    /// Return a vector where each element at an index i is filled with 1s if
    /// the elements of `self` and `other` at index i are equal, and filled with
    /// zeroes otherwise.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s::interleave(0, 2).eq(u8s(0)), u8s::interleave(0xFF, 0));
    /// assert_eq!(u32s::halfs(1, 0).min(u32s(0)), u32s::halfs(0, 0xFFFFFFFF));
    /// # }
    /// ```
    fn eq(&self, other: &Self) -> Self;

    // /// Return a vector where each element at an index i is filled with 1s if
    // /// the elements of `self` and `other` at index i are not equal, and filled
    // /// with zeroes otherwise.
    // ///
    // /// ```
    // /// extern crate faster;
    // /// use faster::*;
    // ///
    // /// # fn main() {
    // /// assert_eq!(u8s::interleave(0, 2).eq(u8s(0)), u8s::interleave(0, 0xFF));
    // /// assert_eq!(u32s::halfs(1, 0).min(u32s(0)), u32s::halfs(0xFFFFFFFF, 0));
    // /// # }
    // /// ```
    // fn ne(&self, other: &Self) -> Self {
    //     TODO: implement masks
    // }
}

macro_rules! rust_fallback_eq {
    (impl $trait:tt for $type:tt where $feat:tt {
        $($rustfn:ident => $mask:tt, $mmfn:tt  ( $($mmfnargs:expr),* ), [$($n:expr),+]);*;}) => (
        impl $trait for $type {
            $(
                #[inline(always)]
                #[cfg(target_feature = $feat)]
                fn $rustfn(&self, other: &Self) -> Self {
                    use core_or_std::mem::transmute;
                    unsafe { transmute($mmfn(transmute(*self), transmute(*other), $($mmfnargs),*)) }
                }

                #[inline(always)]
                #[cfg(not(target_feature = $feat))]
                fn $rustfn(&self, other: &Self) -> Self {
                    use core_or_std::mem::transmute;
                    unsafe {
                        Self::new($(transmute(if self.extract($n).$rustfn(&other.extract($n)) {
                            $mask::max_value()
                        } else {
                            $mask::min_value()
                        })),*)
                    }
                }
            )*
        }
    );
}

rust_fallback_eq! {
    impl PackedEq for u8x16 where "sse2" {
        eq => u8, _mm_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_eq! {
    impl PackedEq for i8x16 where "sse4.1" {
        eq => u8, _mm_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_eq! {
    impl PackedEq for u16x8 where "sse4.1" {
        eq => u16, _mm_cmpeq_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_eq! {
    impl PackedEq for i16x8 where "sse4.1" {
        eq => u16, _mm_cmpeq_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_eq! {
    impl PackedEq for u32x4 where "sse4.1" {
        eq => u32, _mm_cmpeq_epi32(), [0, 1, 2, 3];
    }
}

rust_fallback_eq! {
    impl PackedEq for i32x4 where "sse4.1" {
        eq => u32, _mm_cmpeq_epi32(), [0, 1, 2, 3];
    }
}

rust_fallback_eq! {
    impl PackedEq for f32x4 where "sse" {
        eq => u32, _mm_cmpeq_ps(), [0, 1, 2, 3];
    }
}

rust_fallback_eq! {
    impl PackedEq for f64x2 where "sse2" {
        eq => u64, _mm_cmpeq_pd(), [0, 1];
    }
}

rust_fallback_eq! {
    impl PackedEq for u64x2 where "sse4.1" {
        eq => u64, _mm_cmpeq_epi64(), [0, 1];
    }
}

rust_fallback_eq! {
    impl PackedEq for i64x2 where "sse4.1" {
        eq => u64, _mm_cmpeq_epi64(), [0, 1];
    }
}

rust_fallback_eq! {
    impl PackedEq for u8x32 where "avx2" {
        eq => u8, _mm256_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                     17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_eq! {
    impl PackedEq for i8x32 where "avx2" {
        eq => u8, _mm256_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_eq! {
    impl PackedEq for u16x16 where "avx2" {
        eq => u16, _mm256_cmpeq_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}


rust_fallback_eq! {
    impl PackedEq for i16x16 where "avx2" {
        eq => u16, _mm256_cmpeq_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_eq! {
    impl PackedEq for u32x8 where "avx" {
        eq => u32, _mm256_cmpeq_epi32(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_eq! {
    impl PackedEq for i32x8 where "avx" {
        eq => u32, _mm256_cmpeq_epi32(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_eq! {
    impl PackedEq for f32x8 where "avx" {
        eq => u32, _mm256_cmp_ps(0x00), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_eq! {
    impl PackedEq for f64x4 where "avx" {
        eq => u64, _mm256_cmp_pd(0x00), [0, 1, 2, 3];
    }
}

rust_fallback_eq! {
    impl PackedEq for u64x4 where "avx2" {
        eq => u64, _mm256_cmpeq_epi64(), [0, 1, 2, 3];
    }
}
rust_fallback_eq! {
    impl PackedEq for i64x4 where "avx2" {
        eq => u64, _mm256_cmpeq_epi64(), [0, 1, 2, 3];
    }
}
