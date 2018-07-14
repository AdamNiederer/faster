// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::intrin::sum::*;
use crate::intrin::transmute::*;
use crate::intrin::popcnt::*;
use crate::intrin::sum::*;
use crate::arch::x86::intrin::upcast::*;
use crate::arch::x86::intrin::sum::UpcastSum;
use crate::intrin::upcast::*;
use crate::vecs::*;


#[inline(always)]
#[cfg(target_feature = "ssse3")]
unsafe fn popcnt128(v: u8x16) -> usize {
    // SSE3 popcnt algorithm by Wojciech Muła
    // http://wm.ite.pl/articles/sse-popcount.html
    let lookup = i8x16::new(0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4);
    let lo = v.be_i8s() & i8x16::splat(0x0f);
    let hi = v.be_i8s() >> 4 & i8x16::splat(0x0f);
    (_mm_shuffle_epi8(lookup, hi) + _mm_shuffle_epi8(lookup, lo))
        .sum_upcast() as usize
}

#[inline(always)]
#[cfg(not(target_feature = "ssse3"))]
#[allow(unused_unsafe)]
unsafe fn popcnt128(v: u8x16) -> usize {
    v.be_u64s(). scalar_reduce(0, |acc, s| acc + (s.count_ones() as usize))
}

#[inline(always)]
#[cfg(target_feature = "avx2")]
unsafe fn popcnt256(v: u8x32) -> usize {
    // AVX2 popcnt algorithm by Wojciech Muła, Nathan Kurz, and Daniel Lemire
    // https://arxiv.org/abs/1611.07612
    let lookup = i8x32::new(0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4,
                            0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4);
    let lo = v.be_i8s() & i8x32::splat(0x0f);
    let hi = (v.be_i8s() >> 4) & i8x32::splat(0x0f);
    (_mm256_shuffle_epi8(lookup, hi) + _mm256_shuffle_epi8(lookup, lo))
        .sum_upcast() as usize
}

#[inline(always)]
#[cfg(not(target_feature = "avx2"))]
#[allow(unused_unsafe)]
unsafe fn popcnt256(v: u8x32) -> usize {
    v.be_u64s().scalar_reduce(0, |acc, s| acc + (s.count_ones() as usize))
}

#[inline(always)]
// #[cfg(not(target_feature = "avx512"))]
unsafe fn popcnt512(v: u8x64) -> usize {
    v.be_u64s().scalar_reduce(0, |acc, s| acc + (s.count_ones() as usize))
}


impl_popcnt!(u8x64, popcnt512, u8x32, popcnt256, u8x16, popcnt128);
impl_popcnt!(i8x64, popcnt512, i8x32, popcnt256, i8x16, popcnt128);
impl_popcnt!(u16x32, popcnt512, u16x16, popcnt256, u16x8, popcnt128);
impl_popcnt!(i16x32, popcnt512, i16x16, popcnt256, i16x8, popcnt128);
impl_popcnt!(u32x16, popcnt512, u32x8, popcnt256, u32x4, popcnt128);
impl_popcnt!(i32x16, popcnt512, i32x8, popcnt256, i32x4, popcnt128);
impl_popcnt!(u64x8, popcnt512, u64x4, popcnt256, u64x2, popcnt128);
impl_popcnt!(i64x8, popcnt512, i64x4, popcnt256, i64x2, popcnt128);

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

    test_popcnt!((u8, u8, u8, i8, i8, i8, u16, u16, u16, i16, i16, i16, u32, u32, u32, i32, i32, i32, u64, u64, u64, i64, i64, i64),
                 (u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2),
                 (popcnt_u8x64, popcnt_u8x32, popcnt_u8x16, popcnt_i8x64, popcnt_i8x32, popcnt_i8x16, popcnt_u16x32, popcnt_u16x16, popcnt_u16x8, popcnt_i16x32, popcnt_i16x16, popcnt_i16x8, popcnt_u32x16, popcnt_u32x8, popcnt_u32x4, popcnt_i32x16, popcnt_i32x8, popcnt_i32x4, popcnt_u64x8, popcnt_u64x4, popcnt_u64x2, popcnt_i64x8, popcnt_i64x4, popcnt_i64x2));
}
