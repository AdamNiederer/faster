// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait SaturatingHAdd {
    /// Return a vector containing the interleaved sums of elements in `self`
    /// and `other`, using saturating addition. The returned vector will begin
    /// with the sum of the first two elements in `self`, and end with the sum
    /// of the last two elements in `other`
    fn saturating_hadd(&self, other: Self) -> Self;
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn saturating_hadd_i8s() {
        assert_eq!(i8s(1).saturating_hadd(i8s(2)), i8s::interleave(2, 4));
        assert_eq!(i8s::interleave(1, 2).saturating_hadd(i8s::interleave(3, 4)), i8s::interleave(3, 7));
        assert_eq!(i8s::interleave(-100, -100).saturating_hadd(i8s::interleave(100, 100)), i8s::interleave(i8::min_value(), i8::max_value()));
    }

    #[test]
    fn saturating_hadd_i16s() {
        assert_eq!(i16s(1).saturating_hadd(i16s(2)), i16s::interleave(2, 4));
        assert_eq!(i16s::interleave(1, 2).saturating_hadd(i16s::interleave(3, 4)), i16s::interleave(3, 7));
        assert_eq!(i16s::interleave(-30000, -30000).saturating_hadd(i16s::interleave(30000, 30000)), i16s::interleave(i16::min_value(), i16::max_value()));
    }

    #[test]
    fn saturating_hadd_i32s() {
        assert_eq!(i32s(1).saturating_hadd(i32s(2)), i32s::interleave(2, 4));
        assert_eq!(i32s::interleave(1, 2).saturating_hadd(i32s::interleave(3, 4)), i32s::interleave(3, 7));
        assert_eq!(i32s::interleave(-2_000_000_000, -2_000_000_000).saturating_hadd(i32s::interleave(2_000_000_000, 2_000_000_000)), i32s::interleave(i32::min_value(), i32::max_value()));
    }

    #[test]
    fn saturating_hadd_i64s() {
        assert_eq!(i64s(1).saturating_hadd(i64s(2)), i64s::interleave(2, 4));
        assert_eq!(i64s::interleave(1, 2).saturating_hadd(i64s::interleave(3, 4)), i64s::interleave(3, 7));
        assert_eq!(i64s::interleave(-9_000_000_000_000_000_000, -9_000_000_000_000_000_000).saturating_hadd(i64s::interleave(9_000_000_000_000_000_000, 9_000_000_000_000_000_000)), i64s::interleave(i64::min_value(), i64::max_value()));
    }

    #[test]
    fn saturating_hadd_u8s() {
        assert_eq!(u8s(1).saturating_hadd(u8s(2)), u8s::interleave(2, 4));
        assert_eq!(u8s::interleave(1, 2).saturating_hadd(u8s::interleave(3, 4)), u8s::interleave(3, 7));
        assert_eq!(u8s(200).saturating_hadd(u8s(200)), u8s(u8::max_value()));
    }

    #[test]
    fn saturating_hadd_u16s() {
        assert_eq!(u16s(1).saturating_hadd(u16s(2)), u16s::interleave(2, 4));
        assert_eq!(u16s::interleave(1, 2).saturating_hadd(u16s::interleave(3, 4)), u16s::interleave(3, 7));
        assert_eq!(u16s(60000).saturating_hadd(u16s(60000)), u16s(u16::max_value()));
    }

    #[test]
    fn saturating_hadd_u32s() {
        assert_eq!(u32s(1).saturating_hadd(u32s(2)), u32s::interleave(2, 4));
        assert_eq!(u32s::interleave(1, 2).saturating_hadd(u32s::interleave(3, 4)), u32s::interleave(3, 7));
        assert_eq!(u32s(4_000_000_000).saturating_hadd(u32s(4_000_000_000)), u32s(u32::max_value()));
    }

    #[test]
    fn saturating_hadd_u64s() {
        assert_eq!(u64s(1).saturating_hadd(u64s(2)), u64s::interleave(2, 4));
        assert_eq!(u64s::interleave(1, 2).saturating_hadd(u64s::interleave(3, 4)), u64s::interleave(3, 7));
        assert_eq!(u64s(18_000_000_000_000_000_000).saturating_hadd(u64s(18_000_000_000_000_000_000)), u64s(u64::max_value()));
    }
}
