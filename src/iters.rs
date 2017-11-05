use vecs::Packable;

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
