// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait SaturatingHSub {
    /// Return a vector containing the interleaved differences of elements in
    /// `self` and `other`, using saturating subtraction. The returned vector
    /// will begin with the difference of the first two elements in `self`, and
    /// end with the difference of the last two elements in `other`
    fn saturating_hsub(&self, other: Self) -> Self;
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn saturating_hsub_i8s() {
        assert_eq!(i8s(1).saturating_hsub(i8s(2)), i8s::interleave(0, 0));
        assert_eq!(i8s::interleave(1, 2).saturating_hsub(i8s::interleave(3, 4)), i8s::interleave(-1, -1));
        assert_eq!(i8s::interleave(-100, 100).saturating_hsub(i8s::interleave(100, -100)), i8s::interleave(i8::min_value(), i8::max_value()));
    }

    #[test]
    fn saturating_hsub_i16s() {
        assert_eq!(i16s(1).saturating_hsub(i16s(2)), i16s::interleave(0, 0));
        assert_eq!(i16s::interleave(1, 2).saturating_hsub(i16s::interleave(3, 4)), i16s::interleave(-1, -1));
        assert_eq!(i16s::interleave(-30000, 30000).saturating_hsub(i16s::interleave(30000, -30000)), i16s::interleave(i16::min_value(), i16::max_value()));
    }

    #[test]
    fn saturating_hsub_i32s() {
        assert_eq!(i32s(1).saturating_hsub(i32s(2)), i32s::interleave(0, 0));
        assert_eq!(i32s::interleave(1, 2).saturating_hsub(i32s::interleave(3, 4)), i32s::interleave(-1, -1));
        assert_eq!(i32s::interleave(-2_000_000_000, 2_000_000_000).saturating_hsub(i32s::interleave(2_000_000_000, -2_000_000_000)), i32s::interleave(i32::min_value(), i32::max_value()));
    }

    #[test]
    fn saturating_hsub_i64s() {
        assert_eq!(i64s(1).saturating_hsub(i64s(2)), i64s::interleave(0, 0));
        assert_eq!(i64s::interleave(1, 2).saturating_hsub(i64s::interleave(3, 4)), i64s::interleave(-1, -1));
        assert_eq!(i64s::interleave(-9_000_000_000_000_000_000, 9_000_000_000_000_000_000).saturating_hsub(i64s::interleave(9_000_000_000_000_000_000, -9_000_000_000_000_000_000)), i64s::interleave(i64::min_value(), i64::max_value()));
    }

    #[test]
    fn saturating_hsub_u8s() {
        assert_eq!(u8s(1).saturating_hsub(u8s(2)), u8s::interleave(0, 0));
        assert_eq!(u8s::interleave(1, 2).saturating_hsub(u8s::interleave(3, 4)), u8s::interleave(0, 0));
        assert_eq!(u8s::interleave(2, 1).saturating_hsub(u8s::interleave(4, 3)), u8s::interleave(1, 1));
    }

    #[test]
    fn saturating_hsub_u16s() {
        assert_eq!(u16s(1).saturating_hsub(u16s(2)), u16s::interleave(0, 0));
        assert_eq!(u16s::interleave(1, 2).saturating_hsub(u16s::interleave(3, 4)), u16s::interleave(0, 0));
        assert_eq!(u16s::interleave(2, 1).saturating_hsub(u16s::interleave(4, 3)), u16s::interleave(1, 1));
    }

    #[test]
    fn saturating_hsub_u32s() {
        assert_eq!(u32s(1).saturating_hsub(u32s(2)), u32s::interleave(0, 0));
        assert_eq!(u32s::interleave(1, 2).saturating_hsub(u32s::interleave(3, 4)), u32s::interleave(0, 0));
        assert_eq!(u32s::interleave(2, 1).saturating_hsub(u32s::interleave(4, 3)), u32s::interleave(1, 1));
    }

    #[test]
    fn saturating_hsub_u64s() {
        assert_eq!(u64s(1).saturating_hsub(u64s(2)), u64s::interleave(0, 0));
        assert_eq!(u64s::interleave(1, 2).saturating_hsub(u64s::interleave(3, 4)), u64s::interleave(0, 0));
        assert_eq!(u64s::interleave(2, 1).saturating_hsub(u64s::interleave(4, 3)), u64s::interleave(1, 1));
    }
}
