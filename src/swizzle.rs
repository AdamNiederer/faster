// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use vecs::{Packable, Packed};
use iters::{PackedIter, PackedIterator};
use core_or_std::iter::{Iterator, ExactSizeIterator};
// use stdsimd::simd::*;
// use core_or_std::mem::uninitialized;
// use core_or_std::ptr::write;

pub struct PackedStripe<'a, T> where T : 'a + Packable {
    iter: &'a PackedIter<'a, T>,
    position: usize,
    stride: usize
}

impl<'a, T> Iterator for PackedStripe<'a, T> where T : Packable {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.data.get(self.position).map(|v| { self.position += self.stride; *v })
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.iter.len() - self.position) / self.stride;
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
    #[cfg(not(feature = "no-std"))]
    pub fn stripe(&'a self, count: usize) -> Vec<PackedStripe<'a, T>> {
        (0..count).map(move |i| {
            PackedStripe {
                iter: &self,
                position: self.position + i,
                stride: count
            }
        }).collect::<Vec<PackedStripe<T>>>()
    }

    // TODO: Const generics?
    pub fn stripe_two(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (PackedStripe {
            iter: &self,
            position: self.position,
            stride: 2 },
         PackedStripe {
             iter: &self,
             position: self.position + 1,
             stride: 2 })
    }

    pub fn stripe_three(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (PackedStripe {
            iter: &self,
            position: self.position,
            stride: 3 },
         PackedStripe {
             iter: &self,
             position: self.position + 1,
             stride: 3 },
         PackedStripe {
             iter: &self,
             position: self.position + 2,
             stride: 3 })
    }

    pub fn stripe_four(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (PackedStripe {
            iter: &self,
            position: self.position,
            stride: 4 },
         PackedStripe {
             iter: &self,
             position: self.position + 1,
             stride: 4 },
         PackedStripe {
             iter: &self,
             position: self.position + 2,
             stride: 4 },
         PackedStripe {
             iter: &self,
             position: self.position + 3,
             stride: 4 })
    }

    pub fn stripe_nine(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (PackedStripe {
            iter: &self,
            position: self.position,
            stride: 9 },
         PackedStripe {
             iter: &self,
             position: self.position + 1,
             stride: 9 },
         PackedStripe {
             iter: &self,
             position: self.position + 2,
             stride: 9 },
         PackedStripe {
             iter: &self,
             position: self.position + 3,
             stride: 9 },
         PackedStripe {
             iter: &self,
             position: self.position + 4,
             stride: 9 },
         PackedStripe {
             iter: &self,
             position: self.position + 5,
             stride: 9 },
         PackedStripe {
             iter: &self,
             position: self.position + 6,
             stride: 9 },
         PackedStripe {
             iter: &self,
             position: self.position + 7,
             stride: 9 },
         PackedStripe {
             iter: &self,
             position: self.position + 8,
             stride: 9 })
    }
}

// impl<'a, T> Iterator for PackedStripe<'a, T> where T : Packable {
//     type Item = T;

//     #[inline(always)]
//     fn next(&mut self) -> Option<Self::Item> {
//         let mut ret = self.iter.data[self.position];
//         self.position += self.stride;
//         ret
//     }
// }

macro_rules! impl_stripe {
    ($el:ty, $gather:tt, $offsets:expr) => (
        impl<'a> PackedIterator for PackedStripe<'a, $el> {
            type Scalar = $el;
            type Vector = <$el as Packable>::Vector;

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
            #[cfg(not(target_feature = "avx2"))]
            fn next_vector(&mut self) -> Option<Self::Vector> {
                if self.position + self.width() * self.stride < self.iter.len() {
                    let mut ret = Self::Vector::default();
                    for i in 0..self.width() {
                        ret = ret.replace(i as u32, self.iter.data[self.position + self.stride * i]);
                    }
                    self.position += self.width() * self.stride;
                    Some(ret)
                } else {
                    None
                }
            }

            #[inline(always)]
            #[cfg(not(target_feature = "avx2"))]
            fn next_partial(&mut self, default: Self::Vector) -> Option<Self::Vector> {
                if self.position < self.iter.len() {
                    let mut ret = default.clone();
                    for i in 0..((self.iter.len() - self.position) / self.stride) {
                        ret = ret.replace(i as u32, self.iter.data[self.position + self.stride * i]);
                    }
                    Some(ret)
                } else {
                    None
                }
            }

            // TODO: Masked gathers for avx2 next_partial

            // TODO: Blocked by stdsimd
            // #[inline(always)]
            // #[cfg(target_feature = "avx2")]
            // fn next_vector(&mut self) -> Option<Self::Vector> {
            //     $gather(&self.iter.data[self.position..], $offsets(self.position, self.stride), Self::Scalar::SIZE)
            // }
        }
    )
}

impl_stripe!(u32, "TODO: Blocked by stdsimd _mm256_i32gather_epi32", |pos, stride| { i32x8::new(pos, pos + stride, pos + 2 * stride, pos + 3 * stride, pos + 4 * stride, pos + 5 * stride, pos + 6 * stride, pos + 7 * stride) });
impl_stripe!(i32, "TODO: Blocked by stdsimd _mm256_i32gather_epi32", |pos, stride| { i32x8::new(pos, pos + stride, pos + 2 * stride, pos + 3 * stride, pos + 4 * stride, pos + 5 * stride, pos + 6 * stride, pos + 7 * stride) });
impl_stripe!(f32, "TODO: Blocked by stdsimd _mm256_i32gather_ps", |pos, stride| { i32x8::new(pos, pos + stride, pos + 2 * stride, pos + 3 * stride, pos + 4 * stride, pos + 5 * stride, pos + 6 * stride, pos + 7 * stride) });
impl_stripe!(u64, "TODO: Blocked by stdsimd _mm256_i32gather_epi64", |pos, stride| { i32x4::new(pos, pos + stride, pos + 2 * stride, pos + 3 * stride) });
impl_stripe!(i64, "TODO: Blocked by stdsimd _mm256_i32gather_epi64", |pos, stride| { i32x4::new(pos, pos + stride, pos + 2 * stride, pos + 3 * stride) });
impl_stripe!(f64, "TODO: Blocked by stdsimd _mm256_i32gather_pd", |pos, stride| { i32x4::new(pos, pos + stride, pos + 2 * stride, pos + 3 * stride) });
// TODO: 16- and 8-bit vector polyfills
