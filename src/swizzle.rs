// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use vecs::*;
use iters::{PackedIter, PackedIterator};
use core_or_std::iter::{Iterator, ExactSizeIterator};

pub struct PackedStripe<'a, T> where T : 'a + Packable {
    iter: &'a PackedIter<'a, T>,
    offsets: i32x8
}

impl<'a, T> PackedStripe<'a, T> where T : 'a + Packable {

    #[inline(always)]
    fn pos(&self) -> usize {
        self.offsets.extract(0) as usize
    }

    #[inline(always)]
    fn stride(&self) -> usize {
        (self.offsets.extract(1) - self.offsets.extract(0)) as usize
    }
}

impl<'a, T> Iterator for PackedStripe<'a, T> where T : 'a + Packable {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Why does self.pos() not work here?
        self.iter.data.get(self.pos() as usize).map(|v| { self.offsets += i32x8::splat(self.stride() as i32); *v })
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.iter.len() - self.pos()) / self.stride();
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for PackedStripe<'a, T>
    where T : Packable {

    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len() / self.stride()
    }
}

impl<'a, T> PackedIter<'a, T> where T : Packable {
    #[cfg(not(feature = "no-std"))]
    pub fn stripe(&'a self, count: usize) -> Vec<PackedStripe<'a, T>> {
        (0..count).map(move |i| {
            PackedStripe {
                iter: &self,
                offsets: i32x8::new((self.position + i) as i32,
                                    (self.position + i + count) as i32,
                                    (self.position + i + count * 2) as i32,
                                    (self.position + i + count * 3) as i32,
                                    (self.position + i + count * 4) as i32,
                                    (self.position + i + count * 5) as i32,
                                    (self.position + i + count * 6) as i32,
                                    (self.position + i + count * 7) as i32)
            }
        }).collect::<Vec<PackedStripe<T>>>()
    }

    // TODO: Const generics?
    pub fn stripe_two(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32, self.position as i32 + 2, self.position as i32 + 4, self.position as i32 + 6,
                                    self.position as i32 + 8, self.position as i32 + 10, self.position as i32 + 12, self.position as i32 + 14)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 1, self.position as i32 + 3, self.position as i32 + 5, self.position as i32 + 7,
                                    self.position as i32 + 9, self.position as i32 + 11, self.position as i32 + 13, self.position as i32 + 15)
            }
        )
    }

    pub fn stripe_three(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 0, self.position as i32 + 3, self.position as i32 + 6, self.position as i32 + 9,
                                    self.position as i32 + 12, self.position as i32 + 15, self.position as i32 + 18, self.position as i32 + 21)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 1, self.position as i32 + 4, self.position as i32 + 7, self.position as i32 + 10,
                                    self.position as i32 + 13, self.position as i32 + 16, self.position as i32 + 19, self.position as i32 + 22)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 2, self.position as i32 + 5, self.position as i32 + 8, self.position as i32 + 11,
                                    self.position as i32 + 14, self.position as i32 + 17, self.position as i32 + 20, self.position as i32 + 23)
            }
        )
    }

    pub fn stripe_four(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 0, self.position as i32 + 4, self.position as i32 + 8, self.position as i32 + 12,
                                    self.position as i32 + 16, self.position as i32 + 20, self.position as i32 + 24, self.position as i32 + 28)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 1, self.position as i32 + 5, self.position as i32 + 9, self.position as i32 + 13,
                                    self.position as i32 + 17, self.position as i32 + 21, self.position as i32 + 25, self.position as i32 + 29)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 2, self.position as i32 + 6, self.position as i32 + 10, self.position as i32 + 14,
                                    self.position as i32 + 18, self.position as i32 + 22, self.position as i32 + 26, self.position as i32 + 30)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 3, self.position as i32 + 7, self.position as i32 + 11, self.position as i32 + 15,
                                    self.position as i32 + 19, self.position as i32 + 23, self.position as i32 + 27, self.position as i32 + 31)
            }
        )
    }

    pub fn stripe_nine(&'a self) -> (PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>, PackedStripe<'a, T>) {
        (
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 0, self.position as i32 + 9, self.position as i32 + 18, self.position as i32 + 27,
                                    self.position as i32 + 36, self.position as i32 + 45, self.position as i32 + 54, self.position as i32 + 63)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 1, self.position as i32 + 10, self.position as i32 + 19, self.position as i32 + 28,
                                    self.position as i32 + 37, self.position as i32 + 46, self.position as i32 + 55, self.position as i32 + 64)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 2, self.position as i32 + 11, self.position as i32 + 20, self.position as i32 + 29,
                                    self.position as i32 + 38, self.position as i32 + 47, self.position as i32 + 56, self.position as i32 + 65)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 3, self.position as i32 + 12, self.position as i32 + 21, self.position as i32 + 30,
                                    self.position as i32 + 39, self.position as i32 + 48, self.position as i32 + 57, self.position as i32 + 66)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 4, self.position as i32 + 13, self.position as i32 + 22, self.position as i32 + 31,
                                    self.position as i32 + 40, self.position as i32 + 49, self.position as i32 + 58, self.position as i32 + 67)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 5, self.position as i32 + 14, self.position as i32 + 23, self.position as i32 + 32,
                                    self.position as i32 + 41, self.position as i32 + 50, self.position as i32 + 59, self.position as i32 + 68)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 6, self.position as i32 + 15, self.position as i32 + 24, self.position as i32 + 33,
                                    self.position as i32 + 42, self.position as i32 + 51, self.position as i32 + 60, self.position as i32 + 69)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 7, self.position as i32 + 16, self.position as i32 + 25, self.position as i32 + 34,
                                    self.position as i32 + 43, self.position as i32 + 52, self.position as i32 + 61, self.position as i32 + 70)
            },
            PackedStripe {
                iter: &self,
                offsets: i32x8::new(self.position as i32 + 8, self.position as i32 + 17, self.position as i32 + 26, self.position as i32 + 35,
                                    self.position as i32 + 44, self.position as i32 + 53, self.position as i32 + 62, self.position as i32 + 71)
            }
        )
    }
}

macro_rules! impl_stripe {
    ($el:ty, $ptr:ty, $gather:tt, $offlim:expr, $cvt:expr, $cvt2:expr) => (
        impl<'a> PackedIterator for PackedStripe<'a, $el> {
            type Scalar = $el;
            type Vector = <$el as Packable>::Vector;

            #[inline(always)]
            fn width(&self) -> usize {
                Self::Vector::WIDTH
            }

            #[inline(always)]
            fn scalar_len(&self) -> usize {
                self.iter.scalar_len() / self.stride()
            }

            #[inline(always)]
            fn scalar_position(&self) -> usize {
                self.iter.scalar_position() // TODO: Don't just pass through the original position
            }

            #[inline(always)]
            #[cfg(not(target_feature = "avx2"))]
            fn next_vector(&mut self) -> Option<Self::Vector> {
                if self.offsets.extract($offlim) < self.iter.len() as i32 {
                    let mut ret = Self::Vector::default();
                    for i in 0..(($offlim + 1).min(Self::Vector::WIDTH) as u32) {
                        ret = ret.replace(i as u32, self.iter.data[self.offsets.extract(i) as usize]);
                    }
                    self.offsets += i32x8::splat((self.offsets.extract($offlim) as usize - self.pos() + self.stride() + 1) as i32);
                    Some(ret)
                } else {
                    None
                }
            }

            #[inline(always)]
            // #[cfg(not(target_feature = "avx2"))]
            fn next_partial(&mut self, default: Self::Vector) -> Option<Self::Vector> {
                if self.pos() < self.iter.len() {
                    let mut ret = default.clone();
                    for i in 0..((self.iter.len() - self.pos()) / self.stride()) {
                        ret = ret.replace(i as u32, self.iter.data[self.offsets.extract(i as u32) as usize]);
                    }
                    Some(ret)
                } else {
                    None
                }
            }

            #[inline(always)]
            #[cfg(target_feature = "avx2")]
            fn next_vector(&mut self) -> Option<Self::Vector> {
                if self.offsets.extract($offlim) < self.iter.len() as i32 {
                    unsafe {
                        let ret = Some($cvt2($gather(self.iter.data[self.pos()..].as_ptr() as *const $ptr,
                                                     $cvt(self.offsets), Self::Scalar::SIZE as i8)));
                        self.offsets += i32x8::splat((self.offsets.extract($offlim) as usize - self.pos() + self.stride() + 1) as i32);
                        ret
                    }
                } else {
                    None
                }
            }
        }
    )
}

impl_stripe!(u32, i32, _mm256_i32gather_epi32, 7, |v| v, |v: i32x8| v.be_u32s());
impl_stripe!(i32, i32, _mm256_i32gather_epi32, 7, |v| v, |v| v);
impl_stripe!(f32, f32, _mm256_i32gather_ps, 7, |v| v, |v| v);
impl_stripe!(u64, i64, _mm256_i32gather_epi64, 3, |v| transmute::<__m128i, i32x4>(_mm256_castsi256_si128(transmute::<i32x8, __m256i>(v))),  |v: i64x4| v.be_u64s());
impl_stripe!(i64, i64, _mm256_i32gather_epi64, 3, |v| transmute::<__m128i, i32x4>(_mm256_castsi256_si128(transmute::<i32x8, __m256i>(v))), |v| v);
impl_stripe!(f64, f64, _mm256_i32gather_pd, 3, |v| transmute::<__m128i, i32x4>(_mm256_castsi256_si128(transmute::<i32x8, __m256i>(v))), |v| v);
// TODO: 16- and 8-bit vector polyfills
