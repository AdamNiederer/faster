// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate stdsimd;
extern crate faster;
use faster::*;

fn main() {
    let lots_of_84s = (&[-10i8; 128][..]).simd_iter()
        .simd_map(|v| i8s::splat(9) * v.abs().be_i8s() - i8s::splat(4) - i8s::splat(2))
        .scalar_collect();

    let lots_of_3s = (&[-123.456f32; 128][..]).simd_iter()
        .simd_map(|v| { f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() -
                        f32s::splat(4.0) - f32s::splat(2.0) })
        .scalar_collect();

    let lots_of_3s_sc = (&[-123.456f32; 128][..]).iter()
        .map(|v| { 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() -
                   4.0 - 2.0 })
        .collect::<Vec<f32>>();

    let mut some_u8s = [0u8; 100];
    let filled_u8s = (&[5u8; 100][..]).simd_iter()
        .simd_map(|vector| vector * u8s::splat(2))
        .scalar_fill(&mut some_u8s);

    println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}", lots_of_84s, lots_of_3s, lots_of_3s_sc, filled_u8s, filled_u8s.len());
}
