// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use intrin::sum::*;
use intrin::transmute::*;
use vecs::*;

pub trait Popcnt : Packed {
    fn count_ones(&self) -> usize;
    fn count_zeroes(&self) -> usize {
        (Self::WIDTH * Self::Scalar::SIZE) - self.count_ones()
    }
}

macro_rules! impl_popcnt_avx2 {
    ($($vec:ty),*) => {
        $(
            impl Popcnt for $vec {
                #[inline(always)]
                #[cfg(target_feature = "avx2")]
                fn count_ones(&self) -> usize {
                    // AVX2 popcnt algorithm by Wojciech Muła, Nathan Kurz, and Daniel Lemire
                    // https://arxiv.org/abs/1611.07612
                    let lookup = u8x32::new(0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4);
                    let lo = self.be_u8s() & u8x32::splat(0x0f);
                    let hi = (self.be_u8s() >> 4) & u8x32::splat(0x0f);
                    unsafe {
                        (_mm256_shuffle_epi8(lookup, hi) + _mm256_shuffle_epi8(lookup, lo))
                    }
                    .sum_upcast() as usize
                }

                #[inline(always)]
                #[cfg(not(target_feature = "avx2"))]
                fn count_ones(&self) -> usize {
                    self.scalar_reduce(0, |acc, s| acc + (s.count_ones() as usize))
                }
            }
        )*
    }
}

impl_popcnt_avx2!(u8x32, i8x32, u16x16, i16x16, u32x8, i32x8, u64x4, i64x4);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_popcnt {
        (($($el:tt),*), ($($vec:tt),*), ($($fn:tt),*)) => (
            $(
                #[test]
                fn $fn() {
                    assert_eq!($vec::splat(1i8 as $el).count_ones(), $vec::WIDTH);
                    assert_eq!($vec::splat(1i8 as $el).count_zeroes()
                               + $vec::splat(1i8 as $el).count_ones(),
                               $vec::WIDTH * <<$vec as Packed>::Scalar as Packable>::SIZE);
                }
            )*
        )
    }

    test_popcnt!((u8, i8, u16, i16, u32, i32, u64, i64),
                 (u8x32, i8x32, u16x16, i16x16, u32x8, i32x8, u64x4, i64x4),
                 (popcnt_u8x32, popcnt_i8x32, popcnt_u16x16, popcnt_i16x16, popcnt_u32x8, popcnt_i32x8, popcnt_u64x4, popcnt_i64x4));

    // test_popcnt!((u8, u8, u8, i8, i8, i8, u16, u16, u16, i16, i16, i16, u32, u32, u32, i32, i32, i32, u64, u64, u64, i64, i64, i64),
    //              (u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2),
    //              (popcnt_u8x64, popcnt_u8x32, popcnt_u8x16, popcnt_i8x64, popcnt_i8x32, popcnt_i8x16, popcnt_u16x32, popcnt_u16x16, popcnt_u16x8, popcnt_i16x32, popcnt_i16x16, popcnt_i16x8, popcnt_u32x16, popcnt_u32x8, popcnt_u32x4, popcnt_i32x16, popcnt_i32x8, popcnt_i32x4, popcnt_u64x8, popcnt_u64x4, popcnt_u64x2, popcnt_i64x8, popcnt_i64x4, popcnt_i64x2));
}
