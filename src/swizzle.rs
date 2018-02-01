// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused_imports)]

use vecs::*;
use iters::{PackedIter, PackedIterator};
use core_or_std::iter::{Iterator, ExactSizeIterator};

// For AVX2 gathers
use core_or_std::mem::transmute;
use stdsimd::vendor::*;
use stdsimd::simd::{__m256i, __m128i};
use intrin::PackedTransmute;

/// A slice-backed iterator which packs every nth element of its constituent
/// elements into a vector.
pub struct PackedStripe<'a, T> where T : Packable + 'a {
    iter: &'a PackedIter<'a, T>,
    base: usize,
    stride: usize
}

impl<'a, T> Iterator for PackedStripe<'a, T> where T : 'a + Packable {
    type Item = T;

    #[inline(never)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.data.get(self.base).map(|v| { self.base += self.stride; *v })
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.iter.len() - self.base) / self.stride;
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for PackedStripe<'a, T>
    where T : Packable {

    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len() / self.stride
    }
}

impl<'a, T> PackedIter<'a, T> where T : Packable {
    /// Return a vec of iterators which pack every `count`th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, `count`th, `count * 2`th...
    /// elements, while the 2nd iterator will pack the 1st, `count + 1`th,
    /// `count * 2 + 1`th... elements.
    #[cfg(not(feature = "no-std"))]
    pub fn stripe(&'a self, count: usize) -> Vec<PackedStripe<'a, T>> {
        (0..count).map(move |offset| {
            PackedStripe {
                iter: self,
                base: offset,
                stride: count
            }
        }).collect::<Vec<PackedStripe<T>>>()
    }

    /// Return a tuple of iterators which pack every 2nd element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 2nd, 4th... elements, while the 2nd
    /// iterator will pack the 1st, 3rd, 5th... elements.
    pub fn stripe_two(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (
            PackedStripe {
                iter: self,
                base: 0,
                stride: 2
            },
            PackedStripe {
                iter: self,
                base: 1,
                stride: 2
            }
        )
    }

    /// Return a tuple of iterators which pack every 3rd element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 3rd, 6th... elements, while the 2nd
    /// iterator will pack the 1st, 4th, 7th... elements.
    pub fn stripe_three(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T> , PackedStripe<'a, T>) {
        (
            PackedStripe {
                iter: self,
                base: 0,
                stride: 3
            },
            PackedStripe {
                iter: self,
                base: 1,
                stride: 3
            },
            PackedStripe {
                iter: self,
                base: 2,
                stride: 3
            }
        )
    }

    /// Return a tuple of iterators which pack every 4th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 4th, 8th... elements, while the 2nd
    /// iterator will pack the 1st, 5th, 9th... elements.
    pub fn stripe_four(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (
            PackedStripe {
                iter: self,
                base: 0,
                stride: 4
            },
            PackedStripe {
                iter: self,
                base: 1,
                stride: 4
            },
            PackedStripe {
                iter: self,
                base: 2,
                stride: 4
            },
            PackedStripe {
                iter: self,
                base: 3,
                stride: 4
            }
        )
    }

    /// Return a tuple of iterators which pack every 9th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 9th, 18th... elements, while the 2nd
    /// iterator will pack the 1st, 10th, 19th... elements.
    pub fn stripe_nine(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (
            PackedStripe {
                iter: self,
                base: 0,
                stride: 9
            },
            PackedStripe {
                iter: self,
                base: 1,
                stride: 9
            },
            PackedStripe {
                iter: self,
                base: 2,
                stride: 9
            },
            PackedStripe {
                iter: self,
                base: 3,
                stride: 9
            },
            PackedStripe {
                iter: self,
                base: 4,
                stride: 9
            },
            PackedStripe {
                iter: self,
                base: 5,
                stride: 9
            },
            PackedStripe {
                iter: self,
                base: 6,
                stride: 9
            },
            PackedStripe {
                iter: self,
                base: 7,
                stride: 9
            },
            PackedStripe {
                iter: self,
                base: 8,
                stride: 9
            }
        )
    }
}

impl<'a, T> PackedIterator for PackedStripe<'a, T> where T : Packable {
    type Scalar = T;
    type Vector = <T as Packable>::Vector;

    #[inline(always)]
    fn width(&self) -> usize {
        Self::Vector::WIDTH
    }

    #[inline(always)]
    fn scalar_len(&self) -> usize {
        self.iter.scalar_len() / self.stride
    }

    #[inline(always)]
    fn scalar_position(&self) -> usize {
        self.iter.scalar_position() // TODO: Don't just pass through the original position
    }

    #[inline(always)]
    fn next_vector(&mut self) -> Option<Self::Vector> {
        if self.base + self.stride * self.width() < self.iter.len() {
            let mut ret = Self::Vector::default();
            for i in 0..self.width() {
                ret = ret.replace(i, self.iter.data[self.base + self.stride * i]);
            }
            self.base += self.stride * self.width();
            Some(ret)
        } else {
            None
        }
    }

    #[inline(always)]
    fn next_partial(&mut self, default: Self::Vector) -> Option<(Self::Vector, usize)> {
        if self.base < self.iter.len() {
            let mut ret = default.clone();
            let fill_amt = (self.iter.len() - self.base) / self.stride;
            // Right-align the partial vector to maintain compat with PackedIter
            for i in (self.width() - fill_amt)..self.width() {
                ret = ret.replace(i, self.iter.data[self.base + self.stride * i]);
            }
            Some((ret, self.width() - fill_amt))
        } else {
            None
        }
    }
}
