#![feature(cfg_target_feature)]
#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(inclusive_range))]

#[cfg(test)] extern crate test;

extern crate stdsimd;
extern crate typenum;
use std::iter::FromIterator;

mod vecs;
use vecs::{Packed, Packable};
mod iters;
pub use iters::*;

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


#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};
    use stdsimd::simd::u8x32;

    #[bench]
    fn bench_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[0u8; 128][..]).simd_iter()
                    .map(|v| (u8x32::splat(9) * v + u8x32::splat(4) - u8x32::splat(2))
                         * u8x32::splat(20) - u8x32::splat(4))
                    .scalar_collect::<Vec<u8>>())
        });
    }

    #[bench]
    fn bench_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[0u8; 128][..]).iter()
                    .map(|e| (9 * e + 4 - 2) * 20 - 4)
                    .collect::<Vec<u8>>())
        });
    }

    #[bench]
    fn bench_nop_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[0u8; 128][..]).simd_iter().map(|e| e).scalar_collect::<Vec<u8>>())
        });
    }

    #[bench]
    fn bench_nop_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[0u8; 128][..]).iter().map(|e| *e).collect::<Vec<u8>>())
        });
    }

}
