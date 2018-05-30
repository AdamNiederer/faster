// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};

macro_rules! impl_cast {
    ($trait:path, $from:ty, $to:ty, $name:ident, $rsname:ident) => (
        impl $trait for $from {
            type Cast = $to;

            #[inline(always)]
            fn $name(self) -> Self::Cast {
                self.$rsname()
            }
        }
    );
}

pub trait Asi8s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to i8s.
    fn as_i8s(self) -> Self::Cast;
}

pub trait Asu8s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to u8s.
    fn as_u8s(self) -> Self::Cast;
}

pub trait Asi16s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to i16s.
    fn as_i16s(self) -> Self::Cast;
}

pub trait Asu16s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to u16s.
    fn as_u16s(self) -> Self::Cast;
}

pub trait Asf32s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to f32s.
    fn as_f32s(self) -> Self::Cast;
}

pub trait Asi32s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to i32s.
    fn as_i32s(self) -> Self::Cast;
}

pub trait Asu32s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to u32s.
    fn as_u32s(self) -> Self::Cast;
}

pub trait Asf64s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to f64s.
    fn as_f64s(self) -> Self::Cast;
}

pub trait Asi64s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to i64s.
    fn as_i64s(self) -> Self::Cast;
}

pub trait Asu64s {
    type Cast;

    /// Return a vector containing all elements of `self` cast to u64s.
    fn as_u64s(self) -> Self::Cast;
}

impl_cast!(Asu8s, i8x16, u8x16, as_u8s, as_u8x16);
impl_cast!(Asi8s, u8x16, i8x16, as_i8s, as_i8x16);

impl_cast!(Asu8s, i8x32, u8x32, as_u8s, as_u8x32);
impl_cast!(Asi8s, u8x32, i8x32, as_i8s, as_i8x32);

impl_cast!(Asu8s, i8x64, u8x64, as_u8s, as_u8x64);
impl_cast!(Asi8s, u8x64, i8x64, as_i8s, as_i8x64);

impl_cast!(Asu16s, i16x8, u16x8, as_u16s, as_u16x8);
impl_cast!(Asi16s, u16x8, i16x8, as_i16s, as_i16x8);

impl_cast!(Asu16s, i16x16, u16x16, as_u16s, as_u16x16);
impl_cast!(Asi16s, u16x16, i16x16, as_i16s, as_i16x16);

impl_cast!(Asu16s, i16x32, u16x32, as_u16s, as_u16x32);
impl_cast!(Asi16s, u16x32, i16x32, as_i16s, as_i16x32);

impl_cast!(Asu32s, i32x4, u32x4, as_u32s, as_u32x4);
impl_cast!(Asu32s, f32x4, u32x4, as_u32s, as_u32x4);
impl_cast!(Asi32s, f32x4, i32x4, as_i32s, as_i32x4);
impl_cast!(Asi32s, u32x4, i32x4, as_i32s, as_i32x4);
impl_cast!(Asf32s, u32x4, f32x4, as_f32s, as_f32x4);
impl_cast!(Asf32s, i32x4, f32x4, as_f32s, as_f32x4);

impl_cast!(Asu32s, i32x8, u32x8, as_u32s, as_u32x8);
impl_cast!(Asu32s, f32x8, u32x8, as_u32s, as_u32x8);
impl_cast!(Asi32s, f32x8, i32x8, as_i32s, as_i32x8);
impl_cast!(Asi32s, u32x8, i32x8, as_i32s, as_i32x8);
impl_cast!(Asf32s, u32x8, f32x8, as_f32s, as_f32x8);
impl_cast!(Asf32s, i32x8, f32x8, as_f32s, as_f32x8);

impl_cast!(Asu32s, i32x16, u32x16, as_u32s, as_u32x16);
impl_cast!(Asu32s, f32x16, u32x16, as_u32s, as_u32x16);
impl_cast!(Asi32s, f32x16, i32x16, as_i32s, as_i32x16);
impl_cast!(Asi32s, u32x16, i32x16, as_i32s, as_i32x16);
impl_cast!(Asf32s, u32x16, f32x16, as_f32s, as_f32x16);
impl_cast!(Asf32s, i32x16, f32x16, as_f32s, as_f32x16);

impl_cast!(Asu64s, i64x2, u64x2, as_u64s, as_u64x2);
impl_cast!(Asu64s, f64x2, u64x2, as_u64s, as_u64x2);
impl_cast!(Asi64s, f64x2, i64x2, as_i64s, as_i64x2);
impl_cast!(Asi64s, u64x2, i64x2, as_i64s, as_i64x2);
impl_cast!(Asf64s, u64x2, f64x2, as_f64s, as_f64x2);
impl_cast!(Asf64s, i64x2, f64x2, as_f64s, as_f64x2);

impl_cast!(Asu64s, i64x4, u64x4, as_u64s, as_u64x4);
impl_cast!(Asu64s, f64x4, u64x4, as_u64s, as_u64x4);
impl_cast!(Asi64s, f64x4, i64x4, as_i64s, as_i64x4);
impl_cast!(Asi64s, u64x4, i64x4, as_i64s, as_i64x4);
impl_cast!(Asf64s, u64x4, f64x4, as_f64s, as_f64x4);
impl_cast!(Asf64s, i64x4, f64x4, as_f64s, as_f64x4);

impl_cast!(Asu64s, i64x8, u64x8, as_u64s, as_u64x8);
impl_cast!(Asu64s, f64x8, u64x8, as_u64s, as_u64x8);
impl_cast!(Asi64s, f64x8, i64x8, as_i64s, as_i64x8);
impl_cast!(Asi64s, u64x8, i64x8, as_i64s, as_i64x8);
impl_cast!(Asf64s, u64x8, f64x8, as_f64s, as_f64x8);
impl_cast!(Asf64s, i64x8, f64x8, as_f64s, as_f64x8);
