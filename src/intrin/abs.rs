// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


pub trait Abs {
    type Out;
    /// Return a vector the absolute value of the elements of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i32s(-2).abs(), u32s(2));
    /// assert_eq!(i8s(-256).abs(), u8s(256));
    /// # }
    /// ```
    fn abs(&self) -> Self::Out;
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn abs_i8s() {
        for i in -128..127 {
            assert_eq!(i8s(i).abs().extract(0), (i as i64).abs() as u8);
        }
    }

    #[test]
    fn abs_i16s() {
        for i in -32768..32767 {
            assert_eq!(i16s(i).abs().extract(0), (i as i64).abs() as u16);
        }
    }

    #[test]
    fn abs_i32s() {
        for i in -65536..65536 {
            assert_eq!(i32s(i).abs().extract(0), (i as i64).abs() as u32);
        }
    }

    #[test]
    fn abs_i64s() {
        for i in -65536..65536 {
            assert_eq!(i64s(i).abs().extract(0), (i as i64).abs() as u64);
        }
    }

    #[test]
    fn abs_f32s() {
        let mut i = -1024.0;
        while i < 1024.0 {
            // This test has some pretty significant float error if done on x86
            assert_eq!(f32s(i).abs().extract(0), i.abs());
            i += 1.0
        }
    }

    #[test]
    fn abs_f64s() {
        let mut i = -1024.0;
        while i < 1024.0 {
            // This test has some pretty significant float error if done on x86
            assert_eq!(f64s(i).abs().extract(0), i.abs());
            i += 1.0
        }
    }
}
