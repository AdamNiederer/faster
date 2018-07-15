// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Sqrt {
    /// Return a vector the containing square roots of the elements of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(rust_2018_preview, stdsimd)]
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(4.0).sqrt(), f32s(2.0));
    /// assert_eq!(f64s(9.0).sqrt(), f64s(3.0));
    /// # }
    /// ```
    fn sqrt(&self) -> Self;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn sqrt_f64s() {
        assert_eq!(f64s(1.0).sqrt(), f64s(1.0));
        assert!(f64s(9.0).sqrt().max(f64s(2.999)) == f64s(9.0).sqrt());
        assert!(f64s(9.0).sqrt().min(f64s(3.001)) == f64s(9.0).sqrt());
    }

    #[test]
    fn sqrt_f32s() {
        assert_eq!(f32s(1.0).sqrt(), f32s(1.0));
        assert!(f32s(9.0).sqrt().max(f32s(2.999)) == f32s(9.0).sqrt());
        assert!(f32s(9.0).sqrt().min(f32s(3.001)) == f32s(9.0).sqrt());
    }
}
