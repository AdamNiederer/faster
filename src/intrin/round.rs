// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Round {
    /// Return a vector with all elements of `self` rounded to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).round(), f32s(3.0));
    /// assert_eq!(f32s(2.4).round(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).round(), f32s(-3.0));
    /// assert_eq!(f32s(-2.4).round(), f32s(-2.0));
    /// # }
    /// ```
    fn round(&self) -> Self;

    /// Return a vector with all elements of `self` rounded up to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).ceil(), f32s(3.0));
    /// assert_eq!(f32s(2.4).ceil(), f32s(3.0));
    /// assert_eq!(f32s(-2.7).ceil(), f32s(-2.0));
    /// assert_eq!(f32s(-2.4).ceil(), f32s(-2.0));
    /// # }
    /// ```
    fn ceil(&self) -> Self;

    /// Return a vector with all elements of `self` rounded down to the nearest
    /// integer.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).floor(), f32s(2.0));
    /// assert_eq!(f32s(2.4).floor(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).floor(), f32s(-3.0));
    /// assert_eq!(f32s(-2.4).floor(), f32s(-3.0));
    /// # }
    /// ```
    fn floor(&self) -> Self;

    /// Return a vector with all elements of `self` truncated. Effectively
    /// rounds the elements towards zero.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(f32s(2.7).trunc(), f32s(2.0));
    /// assert_eq!(f32s(2.4).trunc(), f32s(2.0));
    /// assert_eq!(f32s(-2.7).trunc(), f32s(-2.0));
    /// assert_eq!(f32s(-2.4).trunc(), f32s(-2.0));
    /// # }
    /// ```
    fn trunc(&self) -> Self;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn round_f32s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f32s(i), f32s(i).round());
            assert_eq!(f32s(i), f32s(i + 0.4).round());
            assert_eq!(f32s(i + 1.0), f32s(i + 0.8).round());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f32s(i), f32s(i).round());
            assert_eq!(f32s(i), f32s(i - 0.4).round());
            assert_eq!(f32s(i - 1.0), f32s(i - 0.8).round());
            i -= 1.0;
        }

        assert_eq!(f32s(9.8).round(), f32s(10.0));
        assert_eq!(f32s(9.1).round(), f32s(9.0));
        assert_eq!(f32s(9.0).round(), f32s(9.0));
    }

    #[test]
    fn trunc_f32s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f32s(i), f32s(i).trunc());
            assert_eq!(f32s(i), f32s(i + 0.4).trunc());
            assert_eq!(f32s(i), f32s(i + 0.8).trunc());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f32s(i), f32s(i).trunc());
            assert_eq!(f32s(i), f32s(i - 0.4).trunc());
            assert_eq!(f32s(i), f32s(i - 0.8).trunc());
            i -= 1.0;
        }

        assert_eq!(f32s(9.8).trunc(), f32s(9.0));
        assert_eq!(f32s(9.1).trunc(), f32s(9.0));
        assert_eq!(f32s(9.0).trunc(), f32s(9.0));
    }

    #[test]
    fn ceil_f32s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f32s(i), f32s(i).ceil());
            assert_eq!(f32s(i + 1.0), f32s(i + 0.4).ceil());
            assert_eq!(f32s(i + 1.0), f32s(i + 0.8).ceil());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f32s(i), f32s(i).ceil());
            assert_eq!(f32s(i), f32s(i - 0.4).ceil());
            assert_eq!(f32s(i), f32s(i - 0.8).ceil());
            i -= 1.0;
        }

        assert_eq!(f32s(9.8).ceil(), f32s(10.0));
        assert_eq!(f32s(9.1).ceil(), f32s(10.0));
        assert_eq!(f32s(9.0).ceil(), f32s(9.0));
    }

    #[test]
    fn floor_f32s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f32s(i), f32s(i).floor());
            assert_eq!(f32s(i), f32s(i + 0.4).floor());
            assert_eq!(f32s(i), f32s(i + 0.8).floor());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f32s(i), f32s(i).floor());
            assert_eq!(f32s(i - 1.0), f32s(i - 0.4).floor());
            assert_eq!(f32s(i - 1.0), f32s(i - 0.8).floor());
            i -= 1.0;
        }

        assert_eq!(f32s(9.8).floor(), f32s(9.0));
        assert_eq!(f32s(9.1).floor(), f32s(9.0));
        assert_eq!(f32s(9.0).floor(), f32s(9.0));
    }

    #[test]
    fn round_f64s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f64s(i), f64s(i).round());
            assert_eq!(f64s(i), f64s(i + 0.4).round());
            assert_eq!(f64s(i + 1.0), f64s(i + 0.8).round());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f64s(i), f64s(i).round());
            assert_eq!(f64s(i), f64s(i - 0.4).round());
            assert_eq!(f64s(i - 1.0), f64s(i - 0.8).round());
            i -= 1.0;
        }

        assert_eq!(f64s(9.8).round(), f64s(10.0));
        assert_eq!(f64s(9.1).round(), f64s(9.0));
        assert_eq!(f64s(9.0).round(), f64s(9.0));
    }

    #[test]
    fn trunc_f64s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f64s(i), f64s(i).trunc());
            assert_eq!(f64s(i), f64s(i + 0.4).trunc());
            assert_eq!(f64s(i), f64s(i + 0.8).trunc());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f64s(i), f64s(i).trunc());
            assert_eq!(f64s(i), f64s(i - 0.4).trunc());
            assert_eq!(f64s(i), f64s(i - 0.8).trunc());
            i -= 1.0;
        }

        assert_eq!(f64s(9.8).trunc(), f64s(9.0));
        assert_eq!(f64s(9.1).trunc(), f64s(9.0));
        assert_eq!(f64s(9.0).trunc(), f64s(9.0));
    }

    #[test]
    fn ceil_f64s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f64s(i), f64s(i).ceil());
            assert_eq!(f64s(i + 1.0), f64s(i + 0.4).ceil());
            assert_eq!(f64s(i + 1.0), f64s(i + 0.8).ceil());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f64s(i), f64s(i).ceil());
            assert_eq!(f64s(i), f64s(i - 0.4).ceil());
            assert_eq!(f64s(i), f64s(i - 0.8).ceil());
            i -= 1.0;
        }

        assert_eq!(f64s(9.8).ceil(), f64s(10.0));
        assert_eq!(f64s(9.1).ceil(), f64s(10.0));
        assert_eq!(f64s(9.0).ceil(), f64s(9.0));
    }

    #[test]
    fn floor_f64s() {
        let mut i = 0.0;
        while i < 32.0 {
            assert_eq!(f64s(i), f64s(i).floor());
            assert_eq!(f64s(i), f64s(i + 0.4).floor());
            assert_eq!(f64s(i), f64s(i + 0.8).floor());
            i += 1.0;
        }

        let mut i = 0.0;
        while i > -32.0 {
            assert_eq!(f64s(i), f64s(i).floor());
            assert_eq!(f64s(i - 1.0), f64s(i - 0.4).floor());
            assert_eq!(f64s(i - 1.0), f64s(i - 0.8).floor());
            i -= 1.0;
        }

        assert_eq!(f64s(9.8).floor(), f64s(9.0));
        assert_eq!(f64s(9.1).floor(), f64s(9.0));
        assert_eq!(f64s(9.0).floor(), f64s(9.0));
    }
}
