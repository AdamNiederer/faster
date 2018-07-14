// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

macro_rules! rust_fallback_eq {
    (impl $trait:tt for $type:tt where $feat:tt {
        $($newfn:ident, $rustfn:ident => $mask:tt, $maskel:tt, $mmfn:tt ( $($mmfnargs:expr),* ), [$($n:expr),+]);*;}) => (
        impl $trait for $type {
            $(
                type Out = $mask;

                #[inline(always)]
                #[cfg(target_feature = $feat)]
                fn $newfn(&self, other: Self) -> $mask {
                    use crate::std::mem::transmute;
                    unsafe { transmute($mmfn(transmute(*self), transmute(other), $($mmfnargs),*)) }
                }

                #[inline(always)]
                #[cfg(not(target_feature = $feat))]
                fn $newfn(&self, other: Self) -> Self::Out {
                    use crate::std::mem::transmute;
                    unsafe {
                        Self::Out::new($(transmute(if self.extract($n).$rustfn(&other.extract($n)) {
                            $maskel::max_value()
                        } else {
                            $maskel::min_value()
                        })),*)
                    }
                }
            )*
        }
    );
}
