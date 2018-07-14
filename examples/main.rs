// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![feature(stdsimd)]

extern crate faster;
use faster::*;

#[cfg(feature = "std")]
fn main() {
    let lots_of_84s = (&[-10i8; 33][..]).simd_iter(i8s(0))
        .simd_map(|v| i8s(9) * v.abs().be_i8s() - i8s(4) - i8s(2))
        .simd_map(|v| v)
        .scalar_collect();

    let lots_of_3s = (&[-123.456f32; 128][..]).simd_iter(f32s(0.0))
        .simd_map(|v| { f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() -
                        f32s(4.0) - f32s(2.0) })
        .scalar_collect();

    let lots_of_3s_sc = (&[-123.456f32; 128][..]).iter()
        .map(|v| { 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() -
                   4.0 - 2.0 })
        .collect::<Vec<f32>>();

    let mut some_u8s = [0u8; 100];
    let filled_u8s = (&[5u8; 100][..]).simd_iter(u8s(0))
        .simd_map(|vector| vector * u8s(2))
        .scalar_fill(&mut some_u8s);

    let reduced = (&[-1.0f32; 128][..]).simd_iter(f32s(0.0))
        .simd_reduce(f32s(0.0), |a, v| a + v.abs().sqrt().sqrt().floor()).sum();

    let strided = (0..20u32).collect::<Vec<u32>>().as_slice()
        .stride_two(tuplify!(2, u32s(99))).zip().simd_map(|(a, b)| a + b)
        .scalar_collect();

    println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n", lots_of_84s, lots_of_3s, lots_of_3s_sc, filled_u8s, filled_u8s.len(), reduced, strided);
}

#[cfg(not(feature = "std"))]
fn main() {}
