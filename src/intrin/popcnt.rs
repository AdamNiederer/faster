// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vecs::*;

pub trait Popcnt : Packed {
    fn count_ones(&self) -> usize;

    #[inline(always)]
    fn count_zeroes(&self) -> usize {
        (Self::WIDTH * Self::Scalar::SIZE) - self.count_ones()
    }
}

// Only used in some architectures. Might produce `unused` warning on others.
#[allow(unused_macros)]
macro_rules! impl_popcnt {
    ($($vec:ty, $fn:ident),*) => {
        $(
            impl Popcnt for $vec {
                #[inline(always)]
                #[allow(unused_unsafe)]
                fn count_ones(&self) -> usize {
                    unsafe { $fn(self.be_u8s()) }
                }
            }
        )*
    }
}

// Only used in some architectures. Might produce `unused` warning on others.
#[allow(unused_macros)]
macro_rules! test_popcnt {
    (($($el:tt),*), ($($vec:tt),*), ($($fn:tt),*)) => (
        $(
            #[test]
            fn $fn() {
                assert_eq!($vec::splat(1i8 as $el).count_ones(), $vec::WIDTH);
                assert_eq!($vec::splat(1i8 as $el).count_zeroes()
                           + $vec::splat(1i8 as $el).count_ones(),
                           $vec::WIDTH * <<$vec as Packed>::Scalar as Packable>::SIZE);
            }
        )*
    )
}
