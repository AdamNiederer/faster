// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This file is machine-generated. See vec-patterns-gen.py for more info.

use vecs::*;

pub trait PackedPattern : Packed {
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self;
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self;
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self;
}

impl PackedPattern for u8x64 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            17 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            18 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            19 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            20 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            21 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            22 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            23 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            24 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            25 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            26 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            27 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            28 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            29 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            30 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            31 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            32 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            33 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            34 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            35 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            36 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            37 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            38 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            39 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            40 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            41 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            42 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            43 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            44 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            45 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            46 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            47 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            48 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            49 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            50 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            51 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            52 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            53 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            54 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            55 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            56 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            57 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            58 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            59 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            60 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            61 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            62 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            63 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            64 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u8x32 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            17 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            18 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            19 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            20 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            21 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            22 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            23 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            24 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            25 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            26 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            27 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            28 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            29 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            30 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            31 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            32 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u8x16 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i8x64 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            17 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            18 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            19 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            20 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            21 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            22 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            23 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            24 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            25 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            26 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            27 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            28 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            29 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            30 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            31 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            32 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            33 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            34 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            35 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            36 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            37 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            38 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            39 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            40 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            41 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            42 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            43 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            44 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            45 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            46 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            47 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            48 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            49 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            50 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            51 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            52 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            53 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            54 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            55 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            56 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            57 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            58 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            59 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            60 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            61 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            62 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            63 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            64 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i8x32 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            17 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            18 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            19 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            20 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            21 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            22 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            23 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            24 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            25 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            26 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            27 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            28 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            29 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            30 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            31 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            32 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i8x16 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u16x32 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            17 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            18 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            19 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            20 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            21 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            22 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            23 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            24 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            25 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            26 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            27 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            28 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            29 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            30 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            31 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            32 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u16x16 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u16x8 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i16x32 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            17 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            18 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            19 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            20 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            21 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            22 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            23 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            24 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            25 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            26 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            27 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            28 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            29 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            30 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            31 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            32 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i16x16 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i16x8 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u32x16 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u32x8 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u32x4 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo),
            3 => Self::new(hi, hi, hi, lo),
            4 => Self::new(hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i32x16 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i32x8 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i32x4 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo),
            3 => Self::new(hi, hi, hi, lo),
            4 => Self::new(hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for f32x16 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo, lo),
            9 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo, lo),
            10 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo, lo),
            11 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo, lo),
            12 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo, lo),
            13 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo, lo),
            14 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo, lo),
            15 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, lo),
            16 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for f32x8 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for f32x4 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo),
            3 => Self::new(hi, hi, hi, lo),
            4 => Self::new(hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u64x8 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u64x4 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo),
            3 => Self::new(hi, hi, hi, lo),
            4 => Self::new(hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for u64x2 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo),
            1 => Self::new(hi, lo),
            2 => Self::new(hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i64x8 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i64x4 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo),
            3 => Self::new(hi, hi, hi, lo),
            4 => Self::new(hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for i64x2 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo),
            1 => Self::new(hi, lo),
            2 => Self::new(hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for f64x8 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, hi, hi, lo, lo, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo, hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo, lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo, lo, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo, lo, lo, lo, lo),
            3 => Self::new(hi, hi, hi, lo, lo, lo, lo, lo),
            4 => Self::new(hi, hi, hi, hi, lo, lo, lo, lo),
            5 => Self::new(hi, hi, hi, hi, hi, lo, lo, lo),
            6 => Self::new(hi, hi, hi, hi, hi, hi, lo, lo),
            7 => Self::new(hi, hi, hi, hi, hi, hi, hi, lo),
            8 => Self::new(hi, hi, hi, hi, hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for f64x4 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, hi, lo, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo, hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo, lo, lo),
            1 => Self::new(hi, lo, lo, lo),
            2 => Self::new(hi, hi, lo, lo),
            3 => Self::new(hi, hi, hi, lo),
            4 => Self::new(hi, hi, hi, hi),
            _ => unreachable!()
        }
    }
}

impl PackedPattern for f64x2 {
    #[inline(always)]
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo)
    }

    #[inline(always)]
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {
        Self::new(hi, lo)
    }

    #[inline(always)]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: u8) -> Self {
        assert!(off <= Self::WIDTH as u8);
        match off {
            0 => Self::new(lo, lo),
            1 => Self::new(hi, lo),
            2 => Self::new(hi, hi),
            _ => unreachable!()
        }
    }
}
