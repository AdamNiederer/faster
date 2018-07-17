// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


macro_rules! rust_fallback_impl {
    (impl $trait:tt for $type:tt where $feat:tt {
        $($rustfn:ident => $mmfn:tt  ( $($mmfnargs:expr),* ), [$($n:expr),+]);*;}) => (
        impl $trait for $type {
            $(
                #[inline(always)]
                #[cfg(target_feature = $feat)]
                fn $rustfn(&self) -> Self {
                    optimized!();
                    unsafe { $mmfn(*self, $($mmfnargs),*) }
                }

                #[inline(always)]
                #[cfg(not(target_feature = $feat))]
                fn $rustfn(&self) -> Self {
                    fallback!();
                    Self::new($(self.extract($n).$rustfn(),)*)
                }
            )*
        }
    );
}

macro_rules! rust_fallback_impl_binary {
    (impl $trait:tt for $type:tt where $feat:tt {
        $($rustfn:ident => $mmfn:tt  ( $($mmfnargs:expr),* ), [$($n:expr),+]);*;}) => (
        impl $trait for $type {
            $(
                #[inline(always)]
                #[cfg(target_feature = $feat)]
                fn $rustfn(&self, other: Self) -> Self {
                    use crate::std::mem::transmute;
                    optimized!();
                    unsafe { transmute($mmfn(transmute(*self), transmute(other), $($mmfnargs),*)) }
                }

                #[inline(always)]
                #[cfg(not(target_feature = $feat))]
                fn $rustfn(&self, other: Self) -> Self {
                    fallback!();
                    Self::new($(self.extract($n).$rustfn(other.extract($n)),)*)
                }
            )*
        }
    );
}

macro_rules! hop {
    ($name:ident, $fn:path, $($a:expr, $b:expr),*) => {
        #[inline(always)]
        fn $name(&self, other: Self) -> Self {
            fallback!();
            Self::new($($fn(self.extract($a), self.extract($b)),
                        $fn(other.extract($a), other.extract($b))),*)
        }
    }
}
