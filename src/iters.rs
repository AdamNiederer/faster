use std::iter::FromIterator;
use vecs::{SIMD_SIZE, Packable, Packed};

pub trait PackedIterator : Sized {
    type Vector;
    const WIDTH: usize;

    #[inline(always)]
    fn width(&self) -> usize;

    #[inline(always)]
    fn scalar_len(&self) -> usize;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Vector>;
}

#[derive(Debug)]
pub struct PackedIter<'a, T : 'a + Packable> {
    pub position: usize,
    pub data: &'a [T],
}

impl<'a, T> ExactSizeIterator for PackedIter<'a, T>
    where T : Packable {

    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len() / T::SIZE
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

impl<'a, T> PackedIterator for PackedIter<'a, T> where T : Packable {
    const WIDTH: usize = SIMD_SIZE / <T as Packable>::SIZE;
    type Vector = <T as Packable>::Vector;

    #[inline(always)]
    fn width(&self) -> usize {
        Self::WIDTH
    }

    #[inline(always)]
    fn scalar_len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Vector> {
        if self.position + self.width() > self.scalar_len() {
            None
        } else {
            let ret: Option<Self::Vector> = Some(Self::Vector::load(self.data, self.position));
            self.position += T::SIZE;
            ret
        }
    }
}

impl<'a, T> Iterator for PackedIter<'a, T> where T : Packable  {
    type Item = <PackedIter<'a, T> as PackedIterator>::Vector;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        PackedIterator::next(self)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.scalar_len() - self.position * self.width()) / self.width();
        (remaining, Some(remaining))
    }
}

pub trait FromPackedIterator<T> : FromIterator<T> {
    fn from_simd_iter<I, S>(iter: I) -> Self
        where I : ExactSizeIterator<Item = S>, S : Packed<T>;
}

impl<T> FromPackedIterator<T> for Vec<T> {
    #[inline(always)]
    fn from_simd_iter<I, S>(iter: I) -> Vec<T>
        where I : ExactSizeIterator<Item = S>, S : Packed<T> {
        let mut offset = 0;
        let mut ret = Vec::with_capacity(S::WIDTH * iter.len());

        unsafe {
            ret.set_len(S::WIDTH * iter.len());
            for vec in iter {
                let incr = vec.width();
                vec.store(ret.as_mut_slice(), offset);
                offset += incr;
            }
        }

        ret
    }
}

pub trait IntoScalar<T> {
    fn scalar_collect<C : FromPackedIterator<T>>(self) -> C;
}

impl<T, I, S> IntoScalar<T> for I where I : ExactSizeIterator<Item = S>, S : Packed<T> {
    fn scalar_collect<C : FromPackedIterator<T>>(self) -> C {
        FromPackedIterator::from_simd_iter(self)
    }
}
