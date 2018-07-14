// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
