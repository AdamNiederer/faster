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
