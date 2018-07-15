// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


pub trait HAdd {
    /// Return a vector containing the interleaved sums of elements in `self`
    /// and `other`. The returned vector will begin with the sum of the first
    /// two elements in `self`, and end with the sum of the last two elements in
    /// `other`
    fn hadd(&self, other: Self) -> Self;
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn hadd_i8s() {
        assert_eq!(i8s(1).hadd(i8s(2)), i8s::interleave(2, 4));
        assert_eq!(i8s::interleave(1, 2).hadd(i8s::interleave(3, 4)), i8s::interleave(3, 7));
    }

    #[test]
    fn hadd_i16s() {
        assert_eq!(i16s(1).hadd(i16s(2)), i16s::interleave(2, 4));
        assert_eq!(i16s::interleave(1, 2).hadd(i16s::interleave(3, 4)), i16s::interleave(3, 7));
    }

    #[test]
    fn hadd_i32s() {
        assert_eq!(i32s(1).hadd(i32s(2)), i32s::interleave(2, 4));
        assert_eq!(i32s::interleave(1, 2).hadd(i32s::interleave(3, 4)), i32s::interleave(3, 7));
    }

    #[test]
    fn hadd_i64s() {
        assert_eq!(i64s(1).hadd(i64s(2)), i64s::interleave(2, 4));
        assert_eq!(i64s::interleave(1, 2).hadd(i64s::interleave(3, 4)), i64s::interleave(3, 7));
    }

    #[test]
    fn hadd_u8s() {
        assert_eq!(u8s(1).hadd(u8s(2)), u8s::interleave(2, 4));
        assert_eq!(u8s::interleave(1, 2).hadd(u8s::interleave(3, 4)), u8s::interleave(3, 7));
    }

    #[test]
    fn hadd_u16s() {
        assert_eq!(u16s(1).hadd(u16s(2)), u16s::interleave(2, 4));
        assert_eq!(u16s::interleave(1, 2).hadd(u16s::interleave(3, 4)), u16s::interleave(3, 7));
    }

    #[test]
    fn hadd_u32s() {
        assert_eq!(u32s(1).hadd(u32s(2)), u32s::interleave(2, 4));
        assert_eq!(u32s::interleave(1, 2).hadd(u32s::interleave(3, 4)), u32s::interleave(3, 7));
    }

    #[test]
    fn hadd_u64s() {
        assert_eq!(u64s(1).hadd(u64s(2)), u64s::interleave(2, 4));
        assert_eq!(u64s::interleave(1, 2).hadd(u64s::interleave(3, 4)), u64s::interleave(3, 7));
    }

    #[test]
    fn hadd_f32s() {
        assert_eq!(f32s(1.0).hadd(f32s(2.0)), f32s::interleave(2.0, 4.0));
        assert_eq!(f32s::interleave(1.0, 2.0).hadd(f32s::interleave(3.0, 4.0)), f32s::interleave(3.0, 7.0));
    }

    #[test]
    fn hadd_f64s() {
        assert_eq!(f64s(1.0).hadd(f64s(2.0)), f64s::interleave(2.0, 4.0));
        assert_eq!(f64s::interleave(1.0, 2.0).hadd(f64s::interleave(3.0, 4.0)), f64s::interleave(3.0, 7.0));
    }
}
