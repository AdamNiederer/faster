use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::intrin::merge::*;
use crate::intrin::transmute::*;
use crate::intrin::destride::*;
use crate::std::mem::transmute;

impl Destride for u8x16 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn destride_two(self, other: Self) -> (Self, Self) {
        optimized!();
        unsafe {
            let a = _mm_shuffle_epi8(self.be_i8s(), Self::new(0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15).be_i8s());
            let b = _mm_shuffle_epi8(other.be_i8s(), Self::new(1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14).be_i8s());
            // Backwards merge of a and b (keeps elements at the same indices)
            let c = _mm_shuffle_epi8(b.merge_halves(a), Self::new(8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7).be_i8s());
            (a.merge_halves(b).be_u8s(), c.be_u8s())
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn destride_two(self, other: Self) -> (Self, Self) {
        fallback!();
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        fallback!();
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12)
    }
}

impl Destride for u8x32 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn destride_two(self, other: Self) -> (Self, Self) {
        optimized!();
        unsafe {
            // In-lane destrided vectors
            let a = _mm256_shuffle_epi8(self.be_i8s(), Self::new(0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15).be_i8s());
            let b = _mm256_shuffle_epi8(other.be_i8s(), Self::new(1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14).be_i8s());
            // Cross-lane destrided vectors
            let aa = _mm256_permute4x64_epi64(a.be_i64s(), 0xD8).be_u8s();
            let bb = _mm256_permute4x64_epi64(b.be_i64s(), 0xD8).be_u8s();
            // Backwards merge of aa and bb (keeps elements at the same indices)
            let c = _mm256_permute4x64_epi64(aa.merge_halves(bb).be_i64s(), 0x4E).be_u8s();
            (aa.merge_halves(bb), c)
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn destride_two(self, other: Self) -> (Self, Self) {
        fallback!();
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        fallback!();
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12, 16, 20, 24, 28)
    }
}

impl Destride for i8x16 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn destride_two(self, other: Self) -> (Self, Self) {
        optimized!();
        unsafe {
            let a = _mm_shuffle_epi8(transmute(self), transmute(Self::new(0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15)));
            let b = _mm_shuffle_epi8(transmute(other), transmute(Self::new(1, 3, 5, 7, 9, 11, 13, 15, 0, 2, 4, 6, 8, 10, 12, 14)));
            // Backwards merge of a and b (keeps elements at the same indices)
            let c = _mm_shuffle_epi8(b.merge_halves(a), transmute(Self::new(8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7)));
            (a.be_i8s().merge_halves(b.be_i8s()), c.be_i8s())
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn destride_two(self, other: Self) -> (Self, Self) {
        fallback!();
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        fallback!();
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12)
    }
}

impl Destride for i8x32 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn destride_two(self, other: Self) -> (Self, Self) {
        optimized!();
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

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn destride_two(self, other: Self) -> (Self, Self) {
        fallback!();
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        fallback!();
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12, 16, 20, 24, 28)
    }
}

macro_rules! impl_destride {
    ($t:ty, $($two:expr, $four:expr),*) => {
        impl Destride for $t {
            #[inline(always)]
            fn destride_two(self, other: Self) -> (Self, Self) {
                fallback!();
                destride_two_polyfill!(self, other, $($two, $four),*)
            }

            #[inline(always)]
            fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
                fallback!();
                destride_four_polyfill!(self, b, c, d, $($two),*)
            }
        }
    }
}

impl_destride!(u16x16, 0, 2, 4, 6, 8, 10, 12, 14);
impl_destride!(u16x8, 0, 2, 4, 6);
impl_destride!(i16x16, 0, 2, 4, 6, 8, 10, 12, 14);
impl_destride!(i16x8, 0, 2, 4, 6);

impl_destride!(u32x8, 0, 2, 4, 6);
impl_destride!(u32x4, 0, 2);
impl_destride!(i32x8, 0, 2, 4, 6);
impl_destride!(i32x4, 0, 2);
