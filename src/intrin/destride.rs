use vecs::*;
use stdsimd::vendor::*;
use intrin::{Merge, Transmute};
use core_or_std::mem::transmute;

pub trait Destride : Sized {
    fn destride(self, other: Self) -> (Self, Self);
}

macro_rules! destride_polyfill {
    ($self:expr, $other:expr, $($n:expr),*) => {
        (Self::new($($self.extract($n)),*,
                   $($other.extract($n)),*),
         Self::new($($self.extract($n + 1)),*,
                   $($other.extract($n + 1)),*))
    }
}

impl Destride for u8x16 {
    #[cfg(target_feature = "ssse3")]
    fn destride(self, other: Self) -> (Self, Self) {
        unsafe {
            let a = _mm_shuffle_epi8(self, Self::new(0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15));
            let b = _mm_shuffle_epi8(other, Self::new(1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14));
            // Backwards merge of a and b (keeps elements at the same indices)
            let c = _mm_shuffle_epi8(b.merge_halves(a), Self::new(8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7));
            (a.merge_halves(b), c)
        }
    }

    #[cfg(not(target_feature = "ssse3"))]
    fn destride(self, other: Self) -> (Self, Self) {
        destride_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14)
    }
}


impl Destride for u8x32 {
    #[cfg(target_feature = "avx2")]
    fn destride(self, other: Self) -> (Self, Self) {
        unsafe {
            // In-lane destrided vectors
            let a = _mm256_shuffle_epi8(self, Self::new(0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15));
            let b = _mm256_shuffle_epi8(other, Self::new(1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14));
            // Cross-lane destrided vectors
            let aa = _mm256_permute4x64_epi64(a.be_i64s(), 0xD8).be_u8s();
            let bb = _mm256_permute4x64_epi64(b.be_i64s(), 0xD8).be_u8s();
            // Backwards merge of aa and bb (keeps elements at the same indices)
            let c = _mm256_permute4x64_epi64(aa.merge_halves(bb).be_i64s(), 0x4E).be_u8s();
            (aa.merge_halves(bb), c)
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    fn destride(self, other: Self) -> (Self, Self) {
        destride_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30)
    }
}

impl Destride for i8x16 {
    #[cfg(target_feature = "ssse3")]
    fn destride(self, other: Self) -> (Self, Self) {
        unsafe {
            let a = _mm_shuffle_epi8(transmute(self), transmute(Self::new(0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15)));
            let b = _mm_shuffle_epi8(transmute(other), transmute(Self::new(1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14)));
            // Backwards merge of a and b (keeps elements at the same indices)
            let c = _mm_shuffle_epi8(b.merge_halves(a), transmute(Self::new(8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7)));
            (a.be_i8s().merge_halves(b.be_i8s()), c.be_i8s())
        }
    }

    #[cfg(not(target_feature = "ssse3"))]
    fn destride(self, other: Self) -> (Self, Self) {
        destride_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14)
    }
}

impl Destride for i8x32 {
    #[cfg(target_feature = "avx2")]
    fn destride(self, other: Self) -> (Self, Self) {
        unsafe {
            // In-lane destrided vectors
            let a = _mm256_shuffle_epi8(transmute(self), transmute(Self::new(0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15)));
            let b = _mm256_shuffle_epi8(transmute(other), transmute(Self::new(1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14)));
            // Cross-lane destrided vectors
            let aa = _mm256_permute4x64_epi64(a.be_i64s(), 0xD8).be_i8s();
            let bb = _mm256_permute4x64_epi64(b.be_i64s(), 0xD8).be_i8s();
            // Backwards merge of aa and bb (keeps elements at the same indices)
            let c = _mm256_permute4x64_epi64(aa.merge_halves(bb).be_i64s(), 0x4E).be_i8s();
            (aa.merge_halves(bb), c)
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    fn destride(self, other: Self) -> (Self, Self) {
        destride_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30)
    }
}
