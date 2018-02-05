// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused_imports)]

use vecs::*;
use iters::{SIMDIterable, SIMDIterator, SIMDArray, SIMDObject};
use core_or_std::iter::{Iterator, ExactSizeIterator};

// For AVX2 gathers
use core_or_std::mem::transmute;
use stdsimd::vendor::*;
use stdsimd::simd::{__m256i, __m128i};
use intrin::Transmute;

// TODO: SIMDArray emulation for PackedStripe

/// A slice-backed iterator which packs every nth element of its constituent
/// elements into a vector.
pub struct PackedStripe<'a, A> where A : 'a + SIMDArray {
    iter: &'a A,
    pos: usize,
    base: usize,
    stride: usize,
    default: <A as SIMDObject>::Vector
}

impl<'a, A> Iterator for PackedStripe<'a, A> where A : 'a + SIMDArray {
    type Item = <A as SIMDObject>::Vector;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + self.stride * self.width() < self.iter.scalar_len() {
            let mut ret = <Self as SIMDObject>::Vector::default();
            for i in 0..self.width() {
                ret = ret.replace(i, unsafe {
                    self.iter.load_scalar_unchecked(self.pos + self.stride * i)
                });
            }
            self.pos += self.stride * self.width();
            Some(ret)
        } else {
            None
        }
    }
}

impl<'a, A> ExactSizeIterator for PackedStripe<'a, A> where A : SIMDArray {
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.vector_len() / self.stride
    }
}

pub trait Stripe<A> where A : SIMDArray {
    /// Return a vec of iterators which pack every `count`th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, `count`th, `count * 2`th...
    /// elements, while the 2nd iterator will pack the 1st, `count + 1`th,
    /// `count * 2 + 1`th... elements.
    #[cfg(not(feature = "no-std"))]
    fn stripe(&self, count: usize, default: &[<A as SIMDObject>::Vector]) -> Vec<PackedStripe<A>>;

    /// Return a tuple of iterators which pack every 2nd element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 2nd, 4th... elements, while the 2nd
    /// iterator will pack the 1st, 3rd, 5th... elements.
    fn stripe_two(&self, default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector)) -> (PackedStripe<A>, PackedStripe<A>);

    /// Return a tuple of iterators which pack every 3rd element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 3rd, 6th... elements, while the 2nd
    /// iterator will pack the 1st, 4th, 7th... elements.
    fn stripe_three(&self, default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector , <A as SIMDObject>::Vector)) -> (PackedStripe<A>, PackedStripe<A> , PackedStripe<A>);

    /// Return a tuple of iterators which pack every 4th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 4th, 8th... elements, while the 2nd
    /// iterator will pack the 1st, 5th, 9th... elements.
    fn stripe_four(&self, default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector)) -> (PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>);

    /// Return a tuple of iterators which pack every 9th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 9th, 18th... elements, while the 2nd
    /// iterator will pack the 1st, 10th, 19th... elements.
    fn stripe_nine(&self, default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector)) -> (PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>);
}

impl<A> Stripe<A> for A where A : SIMDArray {

    #[inline(always)]
    #[cfg(not(feature = "no-std"))]
    fn stripe(&self, count: usize, default: &[<A as SIMDObject>::Vector]) -> Vec<PackedStripe<A>> {
        assert!(default.len() == count);
        (0..count).map(move |offset| {
            PackedStripe {
                iter: self,
                pos: offset,
                base: offset,
                stride: count,
                default: unsafe { *default.get_unchecked(offset) }
            }
        }).collect::<Vec<PackedStripe<A>>>()
    }

    #[inline(always)]
    fn stripe_two(&self, default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector)) -> (PackedStripe<A>, PackedStripe<A>) {
        (
            PackedStripe {
                iter: self,
                pos: 0,
                base: 0,
                stride: 2,
                default: default.0
            },
            PackedStripe {
                iter: self,
                pos: 1,
                base: 1,
                stride: 2,
                default: default.1
            }
        )
    }

    #[inline(always)]
    fn stripe_three(&self, default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector , <A as SIMDObject>::Vector)) -> (PackedStripe<A>, PackedStripe<A> , PackedStripe<A>) {
        (
            PackedStripe {
                iter: self,
                pos: 0,
                base: 0,
                stride: 3,
                default: default.0
            },
            PackedStripe {
                iter: self,
                pos: 1,
                base: 1,
                stride: 3,
                default: default.1
            },
            PackedStripe {
                iter: self,
                pos: 2,
                base: 2,
                stride: 3,
                default: default.2
            }
        )
    }

    #[inline(always)]
    fn stripe_four(&self, default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector)) -> (PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>) {
        (
            PackedStripe {
                iter: self,
                pos: 0,
                base: 0,
                stride: 4,
                default: default.0
            },
            PackedStripe {
                iter: self,
                pos: 1,
                base: 1,
                stride: 4,
                default: default.1
            },
            PackedStripe {
                iter: self,
                pos: 2,
                base: 2,
                stride: 4,
                default: default.2
            },
            PackedStripe {
                iter: self,
                pos: 3,
                base: 3,
                stride: 4,
                default: default.3
            }
        )
    }

    #[inline(always)]
    fn stripe_nine(&self, default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector, <A as SIMDObject>::Vector)) -> (PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>, PackedStripe<A>) {
        (
            PackedStripe {
                iter: self,
                pos: 0,
                base: 0,
                stride: 9,
                default: default.0
            },
            PackedStripe {
                iter: self,
                pos: 1,
                base: 1,
                stride: 9,
                default: default.1
            },
            PackedStripe {
                iter: self,
                pos: 2,
                base: 2,
                stride: 9,
                default: default.2
            },
            PackedStripe {
                iter: self,
                pos: 3,
                base: 3,
                stride: 9,
                default: default.3
            },
            PackedStripe {
                iter: self,
                pos: 4,
                base: 4,
                stride: 9,
                default: default.4
            },
            PackedStripe {
                iter: self,
                pos: 5,
                base: 5,
                stride: 9,
                default: default.5
            },
            PackedStripe {
                iter: self,
                pos: 6,
                base: 6,
                stride: 9,
                default: default.6
            },
            PackedStripe {
                iter: self,
                pos: 7,
                base: 7,
                stride: 9,
                default: default.7
            },
            PackedStripe {
                iter: self,
                pos: 8,
                base: 8,
                stride: 9,
                default: default.8
            }
        )
    }
}

impl<'a, A> SIMDObject for PackedStripe<'a, A> where A : SIMDArray {
    type Scalar = <A as SIMDObject>::Scalar;
    type Vector = <A as SIMDObject>::Vector;

    #[inline(always)]
    fn width(&self) -> usize {
        self.iter.width()
    }
}

impl<'a, A> SIMDIterable for PackedStripe<'a, A> where A : SIMDArray {
    #[inline(always)]
    fn scalar_pos(&self) -> usize {
        self.pos / self.stride
    }

    #[inline(always)]
    fn vector_pos(&self) -> usize {
        self.pos / (self.stride * self.width())
    }

    #[inline(always)]
    fn scalar_inc(&mut self) {
        self.pos += self.stride
    }

    #[inline(always)]
    fn vector_inc(&mut self) {
        self.pos += self.stride * self.width()
    }

    #[inline(always)]
    fn finalize(&mut self) {
        self.pos = self.iter.scalar_len()
    }

    #[inline(always)]
    fn default(&self) -> Self::Vector {
        self.default
    }
}

impl<'a, A> SIMDIterator for PackedStripe<'a, A> where A : SIMDArray {
    #[inline(always)]
    fn end(&mut self) -> Option<(Self::Vector, usize)> {
        if self.pos < self.iter.scalar_len() {
            // TODO: Can we simplify this math?
            let mut ret = self.default().clone();
            // Crappy integer division equivalent to ceil(self.iter.len() - self.pos / self.stride)
            let empty_amt = self.width() - ((self.iter.scalar_len() - self.pos - 1) / self.stride + 1);
            // Right-align the partial vector to maintain compat with SIMDRefIter
            for i in empty_amt..self.width() {
                ret = ret.replace(i, unsafe {
                    self.iter.load_scalar_unchecked(self.pos + self.stride * (i - empty_amt))
                });
            }
            Some((ret, empty_amt))
        } else {
            None
        }
    }
}
