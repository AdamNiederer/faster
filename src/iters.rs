use std::marker::PhantomData;
use typenum::{Unsigned};

pub trait PackedIterator : Sized {
    type Item;
    fn width(&self) -> usize;
}

// T : Scalar type, S : SIMD Vector, N : SIMD Width
pub struct PackedIter<'a, T : 'a, S, N> {
    pub data: &'a [T],
    pub position: usize,
    pub __simd_data: PhantomData<(S, N)>,
}

impl<'a, T, S, N> PackedIterator for PackedIter<'a, T, S, N>
    where N : Unsigned {
    type Item = S;

    #[inline(always)]
    fn width(&self) -> usize {
        N::to_usize()
    }
}

impl<T: PackedIterator> IntoPackedIterator for T {
    type Iter = T;
    type Item = T::Item;

    #[inline(always)]
    fn into_simd_iter(self) -> T {
        self
    }
}

pub trait IntoPackedIterator {
    type Iter: PackedIterator<Item = Self::Item>;
    type Item;

    fn into_simd_iter(self) -> Self::Iter;
}

pub trait IntoPackedRefIterator<'a> {
    type Iter: PackedIterator<Item = Self::Item>;
    type Item: 'a;

    fn simd_iter(&'a self) -> Self::Iter;
}

pub trait IntoPackedRefMutIterator<'a> {
    type Iter: PackedIterator<Item = Self::Item>;
    type Item: 'a;

    fn simd_iter_mut(&'a mut self) -> Self::Iter;
}

// Impl ref & ref mut iterators for moved iterator
impl<'a, I: 'a + ?Sized> IntoPackedRefIterator<'a> for I
    where &'a I: IntoPackedIterator {
    type Iter = <&'a I as IntoPackedIterator>::Iter;
    type Item = <&'a I as IntoPackedIterator>::Item;

    fn simd_iter(&'a self) -> Self::Iter {
        self.into_simd_iter()
    }
}

impl<'a, I: 'a + ?Sized> IntoPackedRefMutIterator<'a> for I
    where &'a mut I: IntoPackedIterator {
    type Iter = <&'a mut I as IntoPackedIterator>::Iter;
    type Item = <&'a mut I as IntoPackedIterator>::Item;

    fn simd_iter_mut(&'a mut self) -> Self::Iter {
        self.into_simd_iter()
    }
}
