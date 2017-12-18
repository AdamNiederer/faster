// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use vecs::{Packable, Packed};

pub trait PackedIterator : Sized + ExactSizeIterator {
    type Scalar : Packable;
    type Vector : Packed<Scalar = Self::Scalar>;

    #[inline(always)]
    fn width(&self) -> usize {
        Self::Vector::WIDTH
    }

    fn scalar_len(&self) -> usize;
    fn scalar_position(&self) -> usize;

    fn next_vector(&mut self) -> Option<Self::Vector>;

    #[inline(always)]
    fn simd_map<A, B, F>(self, func: F) -> PackedMap<Self, F>
        where F : Fn(Self::Vector) -> A, A : Packed<Scalar = B>, B : Packable {
        PackedMap {
            iter: self,
            func: func
        }
    }

    #[inline(always)]
    fn simd_reduce<A, F>(&mut self, start: A, default: Self::Vector, func: F) -> A
        where F : Fn(&A, &Self::Vector) -> A {
        let mut acc: A;
        if let Some(v) = self.next_vector() {
            acc = func(&start, &v);
            while let Some(v) = self.next_vector() {
                acc = func(&acc, &v);
            }
            if let Some(v) = self.next_partial(default) {
                // eprintln!("hi! {:?}", v);
                acc = func(&acc, &v);
            }
            debug_assert!(self.next_partial(default).is_none());
            acc
        } else if let Some(v) = self.next_partial(default) {
            acc = func(&start, &v);
            while let Some(v) = self.next_partial(default) {
                acc = func(&acc, &v);
            }
            debug_assert!(self.next_partial(default).is_none());
            acc
        } else {
            start
        }
    }
}

#[derive(Debug)]
pub struct PackedIter<'a, T : 'a + Packable> {
    pub position: usize,
    pub data: &'a [T],
}

#[derive(Debug)]
pub struct PackedMap<I, F> {
    pub iter: I,
    pub func: F,
}

impl<'a, T> Iterator for PackedIter<'a, T> where T : Packable {
    type Item = <PackedIter<'a, T> as PackedIterator>::Scalar;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.data.get(self.position).map(|v| { self.position += 1; *v })
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len() - self.position;
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for PackedIter<'a, T>
    where T : Packable {

    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a, T> PackedIterator for PackedIter<'a, T> where T : Packable {
    type Vector = <T as Packable>::Vector;
    type Scalar = T;

    #[inline(always)]
    fn scalar_len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    fn scalar_position(&self) -> usize {
        self.position
    }

    #[inline(always)]
    fn next_vector(&mut self) -> Option<Self::Vector> {
        if self.position + self.width() <= self.scalar_len() {
            let ret = Some(Self::Vector::load(self.data, self.position));
            self.position += Self::Vector::WIDTH;
            ret
        } else {
            None
        }
    }

    fn next_partial(&mut self, default: Self::Vector) -> Option<Self::Vector> where T : Packable {
        if self.position < self.scalar_len() {

            // Workaround broken replace for now.
            let mut store = Vec::with_capacity(default.width());
            for i in 0..default.width() {
                store.push(default.extract(i));
            }
            let mut i = 0;
            while let Some(scl) = self.next() {
                debug_assert!(i < default.width());
                store[i] = scl;
                i += 1;
            }

            let ret = Self::Vector::load(store.as_slice(), 0);

            // // is stdsimd's replace broken?
            // for i in 0..self.scalar_len() - self.position {
            //     eprintln!("hi! {:?} {:?}", self.position + i, self.data[self.position + i]);
            //     ret = ret.replace(i, self.data[self.position + i]);
            //     eprintln!("hi! {:?}", ret.extract(i));
            // }

            self.position = self.scalar_len();
            Some(ret)
        } else {
            None
        }
    }
}

impl<T: PackedIterator> IntoPackedIterator for T {
    type Iter = T;

    #[inline(always)]
    fn into_simd_iter(self) -> T {
        self
    }
}

pub trait IntoPackedIterator {
    type Iter: PackedIterator;

    fn into_simd_iter(self) -> Self::Iter;
}

pub trait IntoPackedRefIterator<'a> {
    type Iter: PackedIterator;

    fn simd_iter(&'a self) -> Self::Iter;
}

pub trait IntoPackedRefMutIterator<'a> {
    type Iter: PackedIterator;

    fn simd_iter_mut(&'a mut self) -> Self::Iter;
}

// Impl ref & ref mut iterators for moved iterator
impl<'a, I: 'a + ?Sized> IntoPackedRefIterator<'a> for I
    where &'a I: IntoPackedIterator {
    type Iter = <&'a I as IntoPackedIterator>::Iter;

    fn simd_iter(&'a self) -> Self::Iter {
        self.into_simd_iter()
    }
}

impl<'a, I: 'a + ?Sized> IntoPackedRefMutIterator<'a> for I
    where &'a mut I: IntoPackedIterator {
    type Iter = <&'a mut I as IntoPackedIterator>::Iter;

    fn simd_iter_mut(&'a mut self) -> Self::Iter {
        self.into_simd_iter()
    }
}

impl<A, B, I, F> Iterator for PackedMap<I, F>
    where I : PackedIterator<Scalar = <I as Iterator>::Item>, <I as Iterator>::Item : Packable, F : Fn(I::Vector) -> A, A : Packed<Scalar = B>, B : Packable {
    type Item = B;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        Some((&self.func)(I::Vector::splat(self.iter.next()?)).coalesce())
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.len() - self.iter.scalar_position() * self.width()) / self.width();
        (remaining, Some(remaining))
    }
}

impl<'a, I, F> ExactSizeIterator for PackedMap<I, F>
    where Self : PackedIterator, I : PackedIterator {
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, A, B, I, F> PackedIterator for PackedMap<I, F>
    where I : PackedIterator<Scalar = <I as Iterator>::Item>, <I as Iterator>::Item : Packable, F : Fn(I::Vector) -> A, A : Packed<Scalar = B>, B : Packable {
    type Vector = A;
    type Scalar = B;


    #[inline(always)]
    fn scalar_len(&self) -> usize {
        self.iter.scalar_len()
    }

    #[inline(always)]
    fn scalar_position(&self) -> usize {
        self.iter.scalar_position()
    }

    #[inline(always)]
    fn next_vector(&mut self) -> Option<Self::Vector> {
        self.iter.next_vector().map(&self.func)
    }

    fn next_partial(&mut self, default: Self::Vector) -> Option<Self::Vector> {
        let mut i = 1;
        if let Some(scl) = self.next() {
            let mut ret = default.clone();
            ret.replace(0, scl);
            while let Some(scl) = self.next() {
                debug_assert!(i < ret.width());
                ret.replace(i, scl);
                i += 1;
            }
            Some(ret)
        } else {
            None
        }
    }
}

pub trait IntoScalar<T> where T : Packable {
    type Scalar : Packable;
    type Vector : Packed<Scalar = Self::Scalar>;
    fn scalar_collect(&mut self) -> Vec<T>;
    fn scalar_fill<'a>(&mut self, fill: &'a mut [T]) -> &'a mut [T];
}

impl<'a, T, I> IntoScalar<T> for I
    where I : PackedIterator<Scalar = T, Item = T>, I::Vector : Packed<Scalar = T>, T : Packable {
    type Scalar = I::Scalar;
    type Vector = I::Vector;

    #[inline(always)]
    fn scalar_collect(&mut self) -> Vec<Self::Scalar> {
        let mut offset = 0;
        let mut ret = Vec::with_capacity(self.len());

        unsafe {
            ret.set_len(self.len());
            while let Some(vec) = self.next_vector() {
                vec.store(ret.as_mut_slice(), offset);
                offset += Self::Vector::WIDTH;
            }
            while let Some(scl) = self.next() {
                ret[offset] = scl;
                offset += 1;
            }
        }
        ret
    }

    #[inline(always)]
    fn scalar_fill<'b>(&mut self, fill: &'b mut [Self::Scalar]) -> &'b mut [Self::Scalar] {
        let mut offset = 0;

        while let Some(vec) = self.next_vector() {
            vec.store(fill, offset);
            offset += Self::Vector::WIDTH;
        }

        while let Some(scl) = self.next() {
            fill[offset] = scl;
            offset += 1;
        }
        fill
    }
}
