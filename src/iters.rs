use std::iter::FromIterator;
use vecs::{SIMD_SIZE, Packable, Packed};
use std::marker::PhantomData;

pub trait PackedIterator : Sized {
    type Vector;
    type Scalar;

    #[inline(always)]
    fn width(&self) -> usize;

    #[inline(always)]
    fn scalar_len(&self) -> usize;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Vector>;

    #[inline(always)]
    fn next_scalar(&mut self) -> Option<Self::Scalar>;

    #[inline(always)]
    fn uneven_map<A, B, F, G>(self, vectorfn: F, scalarfn: G) -> UnevenMap<Self, A, B, F, G>
        where F : Fn(Self::Vector) -> A, G : Fn(Self::Scalar) -> B, A : Packed<B>, B : Packable;
}

#[derive(Debug)]
pub struct PackedIter<'a, T : 'a + Packable> {
    pub position: usize,
    pub data: &'a [T],
}

#[derive(Debug)]
pub struct UnevenMap<T, A, B, F, G> {
    pub iter: T,
    pub vectorfn: F,
    pub scalarfn: G,
    pub __fn_outputs: PhantomData<(A, B)>
}

impl<'a, T> ExactSizeIterator for PackedIter<'a, T>
    where T : Packable {

    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len() / T::Vector::WIDTH
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
    type Vector = <T as Packable>::Vector;
    type Scalar = T;

    #[inline(always)]
    fn width(&self) -> usize {
        T::Vector::WIDTH
    }

    #[inline(always)]
    fn scalar_len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Vector> {
        if self.position + self.width() <= self.scalar_len() {
            let ret: Option<Self::Vector> = Some(Self::Vector::load(self.data, self.position));
            self.position += Self::Vector::WIDTH;
            ret
        } else {
            None
        }
    }

    #[inline(always)]
    fn next_scalar(&mut self) -> Option<Self::Scalar> {
        if self.position < self.scalar_len() {
            let ret: Option<Self::Scalar> = Some(self.data[self.position]);
            self.position += 1;
            ret
        } else {
            None
        }
    }

    #[inline(always)]
    fn uneven_map<A, B, F, G>(self, vectorfn: F, scalarfn: G) -> UnevenMap<Self, A, B, F, G>
        where F : Fn(Self::Vector) -> A, G : Fn(Self::Scalar) -> B, A : Packed<B>, B : Packable {
        UnevenMap {
            iter: self,
            vectorfn: vectorfn,
            scalarfn: scalarfn,
            __fn_outputs: PhantomData::<(A, B)>
        }
    }
}

impl<'a, T, A, B, F, G> PackedIterator for UnevenMap<T, A, B, F, G>
    where T : PackedIterator, F : Fn(T::Vector) -> A, G : Fn(T::Scalar) -> B, A : Packed<B>, B : Packable {
    type Vector = A;
    type Scalar = B;

    #[inline(always)]
    fn width(&self) -> usize {
        Self::Vector::WIDTH
    }

    #[inline(always)]
    fn scalar_len(&self) -> usize {
        self.iter.scalar_len()
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Vector> {
        self.iter.next().map(&self.vectorfn)
    }

    #[inline(always)]
    fn next_scalar(&mut self) -> Option<Self::Scalar> {
        self.iter.next_scalar().map(&self.scalarfn)
    }

    #[inline(always)]
    fn uneven_map<AA, BB, AF, BG>(self, vectorfn: AF, scalarfn: BG) -> UnevenMap<Self, AA, BB, AF, BG>
        where AF : Fn(Self::Vector) -> AA, BG : Fn(Self::Scalar) -> BB, AA : Packed<BB>, BB : Packable {
        UnevenMap {
            iter: self,
            vectorfn: vectorfn,
            scalarfn: scalarfn,
            __fn_outputs: PhantomData::<(AA, BB)>
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

pub trait FromPackedIterator<T> : FromIterator<T> where T : Packable {
    fn from_simd_iter<I, S>(iter: I) -> Self
        where I : ExactSizeIterator<Item = S>, S : Packed<T>;
}

impl<T> FromPackedIterator<T> for Vec<T> where T : Packable {
    #[inline(always)]
    fn from_simd_iter<I, S>(iter: I) -> Vec<T>
        where I : ExactSizeIterator<Item = S>, S : Packed<T> {
        let mut offset = 0;
        let mut ret = Vec::with_capacity(iter.len() * S::WIDTH);

        unsafe {
            ret.set_len(iter.len() * S::WIDTH);
            for vec in iter {
                let incr = vec.width();
                vec.store(ret.as_mut_slice(), offset);
                offset += incr;
            }
        }

        ret
    }
}

pub trait IntoScalar<T> where T : Packable {
    fn scalar_collect<C : FromPackedIterator<T>>(self) -> C;
}

impl<T, I, S> IntoScalar<T> for I where I : ExactSizeIterator<Item = S>, S : Packed<T>, T : Packable {
    fn scalar_collect<C : FromPackedIterator<T>>(self) -> C {
        FromPackedIterator::from_simd_iter(self)
    }
}
