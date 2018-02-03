// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use iters::{SIMDIter, SIMDRefIter, SIMDRefMutIter, SIMDIterator};

/// A trait which transforms a contiguous collection into an owned stream of
/// vectors.
pub trait IntoSIMDIterator {
    type Iter: SIMDIterator;

    /// Return an iterator over this data which will automatically pack
    /// values into SIMD vectors. See `SIMDIterator::simd_map` and
    /// `SIMDIterator::simd_reduce` for more information.
    fn into_simd_iter(self) -> Self::Iter;
}

/// A trait which transforms a contiguous collection into a slice-backed stream
/// of vectors.
pub trait IntoSIMDRefIterator<'a> {
    type Iter: SIMDIterator;

    /// Return an iterator over this data which will automatically pack
    /// values into SIMD vectors. See `SIMDIterator::simd_map` and
    /// `SIMDIterator::simd_reduce` for more information.
    fn simd_iter(&'a self) -> Self::Iter;
}

/// A trait which transforms a contiguous collection into a mutable slice-backed
/// stream of vectors.
pub trait IntoSIMDRefMutIterator<'a> {
    type Iter: SIMDIterator;

    /// Return an iterator over this data which will automatically pack
    /// values into SIMD vectors. See `SIMDIterator::simd_map` and
    /// `SIMDIterator::simd_reduce` for more information.
    fn simd_iter_mut(&'a mut self) -> Self::Iter;
}

impl<T : SIMDIterator> IntoSIMDIterator for T {
    type Iter = T;

    #[inline(always)]
    default fn into_simd_iter(self) -> T {
        self
    }
}

macro_rules! impl_array_intos {
    ($($el:ty),*) => {
        $(
            impl IntoSIMDIterator for Vec<$el> {
                type Iter = SIMDIter<$el>;

                #[inline(always)]
                fn into_simd_iter(self) -> Self::Iter {
                    SIMDIter {
                        data: self,
                        position: 0,
                    }
                }
            }

            impl<'a> IntoSIMDRefIterator<'a> for [$el] {
                type Iter = SIMDRefIter<'a, $el>;

                #[inline(always)]
                fn simd_iter(&'a self) -> Self::Iter {
                    SIMDRefIter {
                        data: self,
                        position: 0,
                    }
                }
            }

            impl<'a> IntoSIMDRefMutIterator<'a> for [$el] {
                type Iter = SIMDRefMutIter<'a, $el>;

                #[inline(always)]
                fn simd_iter_mut(&'a mut self) -> Self::Iter {
                    SIMDRefMutIter {
                        data: self,
                        position: 0,
                    }
                }
            }
        )*
    }
}

impl_array_intos!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64);
