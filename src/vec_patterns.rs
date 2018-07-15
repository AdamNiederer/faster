// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This file is machine-generated. See vec_patterns_gen.py for more info.

use crate::vecs::*;

/// Constructors which may be used to instantiate vectors with patterned data.
pub trait Pattern : Packed {
    /// Return a vector whose first `Self::WIDTH / 2` elements are `hi`, and
    /// whose last `Self::WIDTH / 2` elements are `lo`.
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self;

    /// Return a vector containing `hi` at every even index, and lo at every odd
    /// index.
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self;

    /// Return a vector whose first `off` elements are `hi`, and whose last
    /// `Self::WIDTH - off` elements are `lo`.
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: usize) -> Self;

    /// Return a vector whose first `off` elements are memset to 0x00, and whose
    /// last `Self::WIDTH - off` elements are memset to 0xFF.
    fn partition_mask(off: usize) -> Self;

    /// Return a vector made entirely of ones.
    fn ones() -> Self;

    /// Return a vector made entirely of zeroes.
    fn zeroes() -> Self;
}
