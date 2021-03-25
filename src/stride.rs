// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused_imports)]

use crate::arch::current::vecs::*;
use crate::core::iter::{ExactSizeIterator, FromIterator, Iterator};
use crate::iters::{SIMDArray, SIMDIterable, SIMDIterator, SIMDObject, SIMDSized, UnsafeIterator};
use crate::vecs::*;

// For AVX2 gathers
use crate::core::mem::transmute;
use crate::intrin::transmute::*;

/// A slice-backed iterator which packs every nth element of its constituent
/// elements into a vector.
#[derive(Clone)]
pub struct PackedStride<'a, A>
where
    A: 'a + SIMDArray,
{
    iter: &'a A,
    pos: usize,
    base: usize, // TODO: Can we get rid of this?
    stride: usize,
    default: <A as SIMDObject>::Vector,
}

impl<'a, A> Iterator for PackedStride<'a, A>
where
    A: 'a + SIMDArray,
{
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
            let width = self.width(); // Appease borrow checker
            self.advance(width);
            Some(ret)
        } else {
            None
        }
    }
}

impl<'a, A> ExactSizeIterator for PackedStride<'a, A>
where
    A: SIMDArray,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.vector_len() / self.stride
    }
}

pub trait Stride<A>
where
    A: SIMDArray,
{
    /// Return a vec of iterators which pack every `count`th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, `count`th, `count * 2`th...
    /// elements, while the 2nd iterator will pack the 1st, `count + 1`th,
    /// `count * 2 + 1`th... elements.
    ///
    /// If you want to collect into something else than `Vec`, you can use the
    /// [`stride_into`](#method.stride_into).
    #[inline(always)]
    #[cfg(feature = "std")]
    fn stride(&self, count: usize, default: &[<A as SIMDObject>::Vector]) -> Vec<PackedStride<A>> {
        self.stride_into(count, default)
    }

    /// Return a collection of iterators which pack every `count`th element into an iterator.
    ///
    /// This works the same way as [`stride`](#method.stride). However, it is possible to choose
    /// the collection into which the result is put.
    ///
    /// This can be beneficial in two cases:
    /// * In no-std environment where `Vec` is not available, but one could use something like
    ///   [arrayvec](https://crates.io/crates/arrayvec).
    /// * To avoid heap allocations in small cases and falling back on them in large cases with
    ///   [smallvec](https://crates.io/crates/smallvec).
    fn stride_into<'s, C>(&'s self, count: usize, default: &[<A as SIMDObject>::Vector]) -> C
    where
        C: FromIterator<PackedStride<'s, A>>,
        A: 's;

    /// Return a tuple of iterators which pack every 2nd element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 2nd, 4th... elements, while the 2nd
    /// iterator will pack the 1st, 3rd, 5th... elements.
    fn stride_two(
        &self,
        default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector),
    ) -> (PackedStride<A>, PackedStride<A>);

    /// Return a tuple of iterators which pack every 3rd element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 3rd, 6th... elements, while the 2nd
    /// iterator will pack the 1st, 4th, 7th... elements.
    fn stride_three(
        &self,
        default: (
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
        ),
    ) -> (PackedStride<A>, PackedStride<A>, PackedStride<A>);

    /// Return a tuple of iterators which pack every 4th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 4th, 8th... elements, while the 2nd
    /// iterator will pack the 1st, 5th, 9th... elements.
    fn stride_four(
        &self,
        default: (
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
        ),
    ) -> (
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
    );

    /// Return a tuple of iterators which pack every 9th element into an
    /// iterator. The nth iterator of the tuple is offset by n - 1. Therefore,
    /// the 1st iterator will pack the 0th, 9th, 18th... elements, while the 2nd
    /// iterator will pack the 1st, 10th, 19th... elements.
    fn stride_nine(
        &self,
        default: (
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
        ),
    ) -> (
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
    );
}

impl<A> Stride<A> for A
where
    A: SIMDArray,
{
    #[inline(always)]
    fn stride_into<'s, C>(&'s self, count: usize, default: &[<A as SIMDObject>::Vector]) -> C
    where
        C: FromIterator<PackedStride<'s, A>>,
        A: 's,
    {
        assert!(default.len() == count);
        (0..count)
            .map(move |offset| PackedStride {
                iter: self,
                pos: offset,
                base: offset,
                stride: count,
                default: unsafe { *default.get_unchecked(offset) },
            })
            .collect()
    }

    #[inline(always)]
    fn stride_two(
        &self,
        default: (<A as SIMDObject>::Vector, <A as SIMDObject>::Vector),
    ) -> (PackedStride<A>, PackedStride<A>) {
        (
            PackedStride {
                iter: self,
                pos: 0,
                base: 0,
                stride: 2,
                default: default.0,
            },
            PackedStride {
                iter: self,
                pos: 1,
                base: 1,
                stride: 2,
                default: default.1,
            },
        )
    }

    #[inline(always)]
    fn stride_three(
        &self,
        default: (
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
        ),
    ) -> (PackedStride<A>, PackedStride<A>, PackedStride<A>) {
        (
            PackedStride {
                iter: self,
                pos: 0,
                base: 0,
                stride: 3,
                default: default.0,
            },
            PackedStride {
                iter: self,
                pos: 1,
                base: 1,
                stride: 3,
                default: default.1,
            },
            PackedStride {
                iter: self,
                pos: 2,
                base: 2,
                stride: 3,
                default: default.2,
            },
        )
    }

    #[inline(always)]
    fn stride_four(
        &self,
        default: (
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
        ),
    ) -> (
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
    ) {
        (
            PackedStride {
                iter: self,
                pos: 0,
                base: 0,
                stride: 4,
                default: default.0,
            },
            PackedStride {
                iter: self,
                pos: 1,
                base: 1,
                stride: 4,
                default: default.1,
            },
            PackedStride {
                iter: self,
                pos: 2,
                base: 2,
                stride: 4,
                default: default.2,
            },
            PackedStride {
                iter: self,
                pos: 3,
                base: 3,
                stride: 4,
                default: default.3,
            },
        )
    }

    #[inline(always)]
    fn stride_nine(
        &self,
        default: (
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
            <A as SIMDObject>::Vector,
        ),
    ) -> (
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
        PackedStride<A>,
    ) {
        (
            PackedStride {
                iter: self,
                pos: 0,
                base: 0,
                stride: 9,
                default: default.0,
            },
            PackedStride {
                iter: self,
                pos: 1,
                base: 1,
                stride: 9,
                default: default.1,
            },
            PackedStride {
                iter: self,
                pos: 2,
                base: 2,
                stride: 9,
                default: default.2,
            },
            PackedStride {
                iter: self,
                pos: 3,
                base: 3,
                stride: 9,
                default: default.3,
            },
            PackedStride {
                iter: self,
                pos: 4,
                base: 4,
                stride: 9,
                default: default.4,
            },
            PackedStride {
                iter: self,
                pos: 5,
                base: 5,
                stride: 9,
                default: default.5,
            },
            PackedStride {
                iter: self,
                pos: 6,
                base: 6,
                stride: 9,
                default: default.6,
            },
            PackedStride {
                iter: self,
                pos: 7,
                base: 7,
                stride: 9,
                default: default.7,
            },
            PackedStride {
                iter: self,
                pos: 8,
                base: 8,
                stride: 9,
                default: default.8,
            },
        )
    }
}

impl<'a, A> SIMDObject for PackedStride<'a, A>
where
    A: SIMDArray,
{
    type Scalar = <A as SIMDObject>::Scalar;
    type Vector = <A as SIMDObject>::Vector;

    #[inline(always)]
    fn width(&self) -> usize {
        self.iter.width()
    }
}

impl<'a, A> SIMDArray for PackedStride<'a, A>
where
    A: SIMDArray,
{
    #[inline(always)]
    fn load(&self, offset: usize) -> Self::Vector {
        assert!(self.base + self.stride * (offset + (self.width() - 1)) < self.iter.scalar_len());
        unsafe { self.load_unchecked(offset) }
    }

    #[inline(always)]
    unsafe fn load_unchecked(&self, offset: usize) -> Self::Vector {
        debug_assert!(offset + self.stride * (self.width() - 1) < self.iter.scalar_len());
        let mut ret = <Self as SIMDObject>::Vector::default();

        for i in 0..self.width() {
            ret = ret.replace(
                i,
                self.iter
                    .load_scalar_unchecked(self.base + self.stride * (offset + i)),
            );
        }
        ret
    }

    #[inline(always)]
    fn load_scalar(&self, offset: usize) -> Self::Scalar {
        self.iter.load_scalar(self.base + offset * self.stride)
    }

    #[inline(always)]
    unsafe fn load_scalar_unchecked(&self, offset: usize) -> Self::Scalar {
        self.iter
            .load_scalar_unchecked(self.base + offset * self.stride)
    }
}

impl<'a, A> SIMDSized for PackedStride<'a, A>
where
    A: SIMDArray,
{
    #[inline(always)]
    fn scalar_len(&self) -> usize {
        self.iter.scalar_len() / self.stride
    }
}

impl<'a, A> SIMDIterable for PackedStride<'a, A>
where
    A: SIMDArray,
{
    #[inline(always)]
    fn scalar_pos(&self) -> usize {
        (self.pos - self.base) / self.stride
    }

    #[inline(always)]
    fn vector_pos(&self) -> usize {
        (self.pos - self.base) / (self.stride * self.width())
    }

    #[inline(always)]
    fn advance(&mut self, amount: usize) {
        self.pos += amount * self.stride;
    }

    #[inline(always)]
    fn default(&self) -> Self::Vector {
        self.default
    }
}

#[cfg(test)]
mod tests {
    use super::super::zip::*;
    use super::super::*;

    use super::*;

    #[test]
    fn scalar_load() {
        let x = [1u8, 2, 3, 4];
        let y = &x[..];
        let (a, b) = y.stride_two((u8s(0), u8s(0)));
        assert_eq!(a.load_scalar(0), 1);
        assert_eq!(a.load_scalar(1), 3);
        assert_eq!(b.load_scalar(0), 2);
        assert_eq!(b.load_scalar(1), 4);
    }

    #[test]
    fn vector_load() {
        let x = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let y = &x[..];
        let (a, b) = y.stride_two((u64s(0), u64s(0)));
        assert_eq!(a.load(0).extract(0), 1);
        assert_eq!(a.load(1).extract(0), 3);
        assert_eq!(b.load(0).extract(0), 2);
        assert_eq!(b.load(1).extract(0), 4);
    }

    #[test]
    fn vector_iter() {
        let x = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let y = &x[..];
        let (a, b) = y.stride_two((u64s(1), u64s(2)));

        for vec in a {
            assert!(vec.scalar_reduce(true, |acc, s| acc && s % 2 == 1));
        }

        for vec in b {
            assert!(vec.scalar_reduce(true, |acc, s| acc && s % 2 == 0));
        }
    }
}
