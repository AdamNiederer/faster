// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait HSub {
    /// Return a vector containing the interleaved differences of elements in
    /// `self` and `other`. The returned vector will begin with the difference
    /// of the first two elements in `self`, and end with the difference of the
    /// last two elements in `other`
    fn hsub(&self, other: Self) -> Self;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn hsub_i8s() {
        assert_eq!(i8s(1).hsub(i8s(2)), i8s::interleave(0, 0));
        assert_eq!(
            i8s::interleave(1, 2).hsub(i8s::interleave(3, 4)),
            i8s::interleave(-1, -1)
        );
    }

    #[test]
    fn hsub_i16s() {
        assert_eq!(i16s(1).hsub(i16s(2)), i16s::interleave(0, 0));
        assert_eq!(
            i16s::interleave(1, 2).hsub(i16s::interleave(3, 4)),
            i16s::interleave(-1, -1)
        );
    }

    #[test]
    fn hsub_i32s() {
        assert_eq!(i32s(1).hsub(i32s(2)), i32s::interleave(0, 0));
        assert_eq!(
            i32s::interleave(1, 2).hsub(i32s::interleave(3, 4)),
            i32s::interleave(-1, -1)
        );
    }

    #[test]
    fn hsub_i64s() {
        assert_eq!(i64s(1).hsub(i64s(2)), i64s::interleave(0, 0));
        assert_eq!(
            i64s::interleave(1, 2).hsub(i64s::interleave(3, 4)),
            i64s::interleave(-1, -1)
        );
    }

    #[test]
    fn hsub_u8s() {
        assert_eq!(u8s(1).hsub(u8s(2)), u8s::interleave(0, 0));
        assert_eq!(
            u8s::interleave(2, 1).hsub(u8s::interleave(4, 3)),
            u8s::interleave(1, 1)
        );
    }

    #[test]
    fn hsub_u16s() {
        assert_eq!(u16s(1).hsub(u16s(2)), u16s::interleave(0, 0));
        assert_eq!(
            u16s::interleave(2, 1).hsub(u16s::interleave(4, 3)),
            u16s::interleave(1, 1)
        );
    }

    #[test]
    fn hsub_u32s() {
        assert_eq!(u32s(1).hsub(u32s(2)), u32s::interleave(0, 0));
        assert_eq!(
            u32s::interleave(2, 1).hsub(u32s::interleave(4, 3)),
            u32s::interleave(1, 1)
        );
    }

    #[test]
    fn hsub_u64s() {
        assert_eq!(u64s(1).hsub(u64s(2)), u64s::interleave(0, 0));
        assert_eq!(
            u64s::interleave(2, 1).hsub(u64s::interleave(4, 3)),
            u64s::interleave(1, 1)
        );
    }

    #[test]
    fn hsub_f32s() {
        assert_eq!(f32s(1.0).hsub(f32s(2.0)), f32s::interleave(0.0, 0.0));
        assert_eq!(
            f32s::interleave(1.0, 2.0).hsub(f32s::interleave(3.0, 4.0)),
            f32s::interleave(-1.0, -1.0)
        );
    }

    #[test]
    fn hsub_f64s() {
        assert_eq!(f64s(1.0).hsub(f64s(2.0)), f64s::interleave(0.0, 0.0));
        assert_eq!(
            f64s::interleave(1.0, 2.0).hsub(f64s::interleave(3.0, 4.0)),
            f64s::interleave(-1.0, -1.0)
        );
    }
}
