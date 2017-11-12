use vecs::{Packable, Packed};
use either::{Either, Left, Right};

pub trait PackedIterator : Sized + ExactSizeIterator {
    type Scalar : Packable;
    type Vector : Packed<Self::Scalar>;

    #[inline(always)]
    fn width(&self) -> usize;

    #[inline(always)]
    fn scalar_len(&self) -> usize;

    #[inline(always)]
    fn scalar_position(&self) -> usize;

    #[inline(always)]
    fn next_vector(&mut self) -> Option<Self::Vector>;
}

pub trait UnevenPackedIterator : PackedIterator {
    #[inline(always)]
    fn next(&mut self) -> Option<Either<<Self as PackedIterator>::Vector, <Self as PackedIterator>::Scalar>> {
        self.next_vector().map_or(
            self.next_scalar().map_or(None, |a| Some(Right(a))),
            |a| Some(Left(a)))
    }

    #[inline(always)]
    fn next_scalar(&mut self) -> Option<Self::Scalar>;

    #[inline(always)]
    fn uneven_map<A, B, F, G>(self, vectorfn: F, scalarfn: G) -> UnevenMap<Self, F, G>
        where F : Fn(Self::Vector) -> A, G : Fn(Self::Scalar) -> B, A : Packed<B>, B : Packable;
}

#[derive(Debug)]
pub struct PackedIter<'a, T : 'a + Packable> {
    pub position: usize,
    pub data: &'a [T],
}

#[derive(Debug)]
pub struct UnevenMap<T, F, G> {
    pub iter: T,
    pub vectorfn: F,
    pub scalarfn: G,
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
    fn scalar_position(&self) -> usize {
        self.position
    }

    #[inline(always)]
    fn next_vector(&mut self) -> Option<Self::Vector> {
        if self.position + self.width() <= self.scalar_len() {
            let ret: Option<Self::Vector> = Some(Self::Vector::load(self.data, self.position));
            self.position += Self::Vector::WIDTH;
            ret
        } else {
            None
        }
    }
}

impl<'a, T> UnevenPackedIterator for PackedIter<'a, T> where T : Packable {
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
    fn uneven_map<A, B, F, G>(self, vectorfn: F, scalarfn: G) -> UnevenMap<Self, F, G>
        where F : Fn(Self::Vector) -> A, G : Fn(Self::Scalar) -> B {
        UnevenMap {
            iter: self,
            vectorfn: vectorfn,
            scalarfn: scalarfn,
        }
    }
}


impl<T, A, B, F, G> Iterator for UnevenMap<T, F, G>
    where Self : UnevenPackedIterator, T : PackedIterator, F : Fn(<T as PackedIterator>::Vector) -> A, G : Fn(<T as PackedIterator>::Scalar) -> B, A : Packed<B>, B : Packable {
    type Item = Either<<Self as PackedIterator>::Vector, <Self as PackedIterator>::Scalar>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: WTF types?
        // UnevenPackedIterator::next(self)
        None
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.scalar_len() - self.iter.scalar_position() * self.width()) / self.width();
        (remaining, Some(remaining))
    }
}


impl<'a, T, F, G> ExactSizeIterator for UnevenMap<T, F, G>
    where Self : UnevenPackedIterator, T : PackedIterator {
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, A, B, T, F, G> PackedIterator for UnevenMap<T, F, G>
    where T : UnevenPackedIterator, F : Fn(T::Vector) -> A, G : Fn(T::Scalar) -> B, A : Packed<B>, B : Packable {
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
    fn scalar_position(&self) -> usize {
        self.iter.scalar_len()
    }

    #[inline(always)]
    fn next_vector(&mut self) -> Option<Self::Vector> {
        self.iter.next_vector().map(&self.vectorfn)
    }
}

impl<'a, A, B, T, F, G> UnevenPackedIterator for UnevenMap<T, F, G>
    where T : UnevenPackedIterator,
F : Fn(T::Vector) -> A,
G : Fn(T::Scalar) -> B,
A : Packed<B>,
B : Packable  {

    #[inline(always)]
    fn next_scalar(&mut self) -> Option<Self::Scalar> {
        self.iter.next_scalar().map(&self.scalarfn)
    }

    #[inline(always)]
    fn uneven_map<AA, BB, AF, BG>(self, vectorfn: AF, scalarfn: BG) -> UnevenMap<Self, AF, BG>
        where AF : Fn(Self::Vector) -> AA, BG : Fn(Self::Scalar) -> BB, AA : Packed<BB>, BB : Packable {
        UnevenMap {
            iter: self,
            vectorfn: vectorfn,
            scalarfn: scalarfn,
        }
    }
}

impl<'a, T> Iterator for PackedIter<'a, T> where T : Packable {
    type Item = <PackedIter<'a, T> as PackedIterator>::Vector;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        PackedIterator::next_vector(self)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.scalar_len() - self.position * self.width()) / self.width();
        (remaining, Some(remaining))
    }
}

pub trait IntoScalar<T, S>
    where T : Packable, Self : ExactSizeIterator<Item = S>, S : Packed<T> {
    fn scalar_collect(self) -> Vec<T>;
    fn scalar_fill<'a>(self, fill: &'a mut [T]) -> &'a mut [T];
}

impl<T, S, I> IntoScalar<T, S> for I
    where I : ExactSizeIterator<Item = S>, T : Packable, S : Packed<T> {

    #[inline(always)]
    fn scalar_collect(self) -> Vec<T> {
        let mut offset = 0;
        let mut ret = Vec::with_capacity(self.len() * S::WIDTH);

        unsafe {
            ret.set_len(self.len() * S::WIDTH);
            for vec in self {
                let incr = vec.width();
                vec.store(ret.as_mut_slice(), offset);
                offset += incr;
            }
        }
        ret
    }

    #[inline(always)]
    fn scalar_fill<'a>(self, fill: &'a mut [T]) -> &'a mut [T] {
        let mut offset = 0;

        for vec in self {
            let incr = vec.width();
            vec.store(fill, offset);
            offset += incr;
        }
        fill
    }

}
