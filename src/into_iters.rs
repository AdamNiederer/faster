// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::iters::{SIMDIter, SIMDIterator, SIMDObject};
use crate::vecs::*;

/// A trait which transforms a contiguous collection into an owned stream of
/// vectors.
pub trait IntoSIMDIterator {
    type Iter : SIMDIterator;

    /// Return an iterator over this data which will automatically pack
    /// values into SIMD vectors. See `SIMDIterator::simd_map` and
    /// `SIMDIterator::simd_reduce` for more information.
    fn into_simd_iter(self, default: <Self::Iter as SIMDObject>::Vector) -> Self::Iter;
}

/// A trait which transforms a contiguous collection into a slice-backed stream
/// of vectors.
pub trait IntoSIMDRefIterator<'a> {
    type Iter : SIMDIterator;

    /// Return an iterator over this data which will automatically pack
    /// values into SIMD vectors. See `SIMDIterator::simd_map` and
    /// `SIMDIterator::simd_reduce` for more information.
    fn simd_iter(&'a self, default: <Self::Iter as SIMDObject>::Vector) -> Self::Iter;
}

/// A trait which transforms a contiguous collection into a mutable slice-backed
/// stream of vectors.
pub trait IntoSIMDRefMutIterator<'a> {
    type Iter : SIMDIterator;

    /// Return an iterator over this data which will automatically pack
    /// values into SIMD vectors. See `SIMDIterator::simd_map` and
    /// `SIMDIterator::simd_reduce` for more information.
    fn simd_iter_mut(&'a mut self, default: <Self::Iter as SIMDObject>::Vector) -> Self::Iter;
}

macro_rules! impl_array_intos {
    ($($el:ty, $vec:ty),*) => {
        $(
            #[cfg(feature = "std")]
            impl IntoSIMDIterator for Vec<$el> {
                type Iter = SIMDIter<Self>;

                #[inline(always)]
                fn into_simd_iter(self, default: $vec) -> Self::Iter {
                    SIMDIter {
                        data: self,
                        position: 0,
                        default: default,
                    }
                }
            }

            impl<'a> IntoSIMDRefIterator<'a> for &'a [$el] {
                type Iter = SIMDIter<Self>;

                #[inline(always)]
                fn simd_iter(&'a self, default: $vec) -> Self::Iter {
                    SIMDIter {
                        data: self,
                        position: 0,
                        default: default,
                    }
                }
            }

            impl<'a> IntoSIMDRefMutIterator<'a> for &'a mut [$el] {
                type Iter = SIMDIter<Self>;

                #[inline(always)]
                fn simd_iter_mut(&'a mut self, default: $vec) -> Self::Iter {
                    SIMDIter {
                        data: self,
                        position: 0,
                        default: default,
                    }
                }
            }

            impl<'a> IntoSIMDRefMutIterator<'a> for [$el] {
                type Iter = SIMDIter<&'a mut Self>;

                #[inline(always)]
                fn simd_iter_mut(&'a mut self, default: $vec) -> Self::Iter {
                    SIMDIter {
                        data: self,
                        position: 0,
                        default: default,
                    }
                }
            }

            impl<'a> IntoSIMDRefIterator<'a> for [$el] {
                type Iter = SIMDIter<&'a Self>;

                #[inline(always)]
                fn simd_iter(&'a self, default: $vec) -> Self::Iter {
                    SIMDIter {
                        data: self,
                        position: 0,
                        default: default,
                    }
                }
            }
        )*
    }
}

impl_array_intos!(u8, u8s,
                  i8, i8s,
                  u16, u16s,
                  i16, i16s,
                  u32, u32s,
                  i32, i32s,
                  f32, f32s,
                  u64, u64s,
                  i64, i64s,
                  f64, f64s);
