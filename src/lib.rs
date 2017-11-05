#![feature(cfg_target_feature)]
#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(inclusive_range))]

#[cfg(test)] extern crate test;

extern crate stdsimd;
extern crate typenum;
use std::iter::FromIterator;
use typenum::Unsigned;

mod vecs;
use vecs::Packed;
mod iters;
pub use iters::*;

pub trait FromPackedIterator<T> : FromIterator<T> {
    fn from_simd_iter<I, S, N>(simd_iter: I) -> Self
        where I : ExactSizeIterator<Item = S>, N : Unsigned, S : Packed<T, S, N>;
}

impl<T> FromPackedIterator<T> for Vec<T> {
    #[inline(always)]
    fn from_simd_iter<I, S, N>(iter: I) -> Vec<T>
        where I : ExactSizeIterator<Item = S>, N : Unsigned, S : Packed<T, S, N> {
        let mut offset = 0;
        let mut ret = Vec::with_capacity(iter.len() * N::to_usize());

        unsafe {
            ret.set_len(iter.len() * N::to_usize());
            for vec in iter {
                let incr = vec.width();
                vec.store(ret.as_mut_slice(), offset);
                offset += incr;
            }
        }

        ret
    }
}

pub trait IntoScalar<T, S, N> {
    fn scalar_collect<C>(self) -> C
        where Self : ExactSizeIterator<Item = S>, N : Unsigned, S : Packed<T, S, N>, C : FromPackedIterator<T>;
}

impl<T, S, N, F> IntoScalar<T, S, N> for F where F : Iterator<Item = S>, S : Packed<T, S, N>, N : Unsigned {
    fn scalar_collect<C>(self) -> C
        where Self : ExactSizeIterator<Item = S>, N : Unsigned, S : Packed<T, S, N>, C : FromPackedIterator<T>{
        FromPackedIterator::from_simd_iter(self)
    }

}

impl<'a, T, S, N> Iterator for PackedIter<'a, T, S, N>
    where N : Unsigned, S : Packed<T, S, N> {
    type Item = S;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.position + N::to_usize() > self.data.len() {
            None
        } else {
            let ret: Option<Self::Item> = Some(Self::Item::load(self.data, self.position));
            self.position += N::to_usize();
            ret
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.data.len() - self.position * N::to_usize()) / N::to_usize();
        (remaining, Some(remaining))
    }
}

impl<'a, T, S, N> ExactSizeIterator for PackedIter<'a, T, S, N>
    where N : Unsigned, S : Packed<T, S, N> {
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
