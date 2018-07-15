use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::merge::*;
use crate::intrin::transmute::*;
use crate::intrin::destride::*;
use crate::std::mem::transmute;


impl Destride for u8x16 {
    #[inline(always)]
    fn destride_two(self, other: Self) -> (Self, Self) {
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12)
    }
}

impl Destride for u8x32 {
    #[inline(always)]
    fn destride_two(self, other: Self) -> (Self, Self) {
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12, 16, 20, 24, 28)
    }
}

impl Destride for i8x16 {
    #[inline(always)]
    fn destride_two(self, other: Self) -> (Self, Self) {
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12)
    }
}

impl Destride for i8x32 {
    #[inline(always)]
    fn destride_two(self, other: Self) -> (Self, Self) {
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12, 16, 20, 24, 28)
    }
}

macro_rules! impl_destride {
    ($t:ty, $($two:expr, $four:expr),*) => {
        impl Destride for $t {
            #[inline(always)]
            fn destride_two(self, other: Self) -> (Self, Self) {
                destride_two_polyfill!(self, other, $($two, $four),*)
            }

            #[inline(always)]
            fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use test::*;

    #[bench]
    #[cfg(feature = "std")]
    fn destride_two(b: &mut Bencher) {
        let a = [0u8; 4096];
        b.iter(|| {
            for v in a.simd_iter(u8s(0)).unroll(2) {
                let _ = black_box(v[0].destride_two(v[1]));
            }
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_four(b: &mut Bencher) {
        let a = [0u8; 4096];
        b.iter(|| {
            for v in a.simd_iter(u8s(0)).unroll(4) {
                let _ = black_box(v[0].destride_four(v[1], v[2], v[3]));
            }
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_two_16(b: &mut Bencher) {
        let a = [0u16; 4096];
        b.iter(|| {
            for v in a.simd_iter(u16s(0)).unroll(2) {
                let _ = black_box(v[0].destride_two(v[1]));
            }
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_four_16(b: &mut Bencher) {
        let a = [0u16; 4096];
        b.iter(|| {
            for v in a.simd_iter(u16s(0)).unroll(4) {
                let _ = v[0].destride_four(v[1], v[2], v[3]);
            }
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_two_32(b: &mut Bencher) {
        let a = [0u32; 4096];
        b.iter(|| {
            for v in a.simd_iter(u32s(0)).unroll(2) {
                let _ = black_box(v[0].destride_two(v[1]));
            }
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_four_32(b: &mut Bencher) {
        let a = [0u32; 4096];
        b.iter(|| {
            for v in a.simd_iter(u32s(0)).unroll(4) {
                let _ = v[0].destride_four(v[1], v[2], v[3]);
            }
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_four_naiive(b: &mut Bencher) {
        let a = [0u8; 4096];
        b.iter(|| {
             (&a[..]).stride_four(tuplify!(4, u8s(0))).zip()
                .simd_do_each(|x| { black_box(x); });
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_two_naiive(b: &mut Bencher) {
        let a = [0u8; 4096];
        b.iter(|| {
             (&a[..]).stride_two(tuplify!(2, u8s(0))).zip()
                .simd_do_each(|x| { black_box(x); });
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_four_naiive_16(b: &mut Bencher) {
        let a = [0u16; 4096];
        b.iter(|| {
             (&a[..]).stride_four(tuplify!(4, u16s(0))).zip()
                .simd_do_each(|x| { black_box(x); });
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_two_naiive_16(b: &mut Bencher) {
        let a = [0u16; 4096];
        b.iter(|| {
             (&a[..]).stride_two(tuplify!(2, u16s(0))).zip()
                .simd_do_each(|x| { black_box(x); });
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_four_naiive_32(b: &mut Bencher) {
        let a = [0u32; 4096];
        b.iter(|| {
             (&a[..]).stride_four(tuplify!(4, u32s(0))).zip()
                .simd_do_each(|x| { black_box(x); });
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn destride_two_naiive_32(b: &mut Bencher) {
        let a = [0u32; 4096];
        b.iter(|| {
             (&a[..]).stride_two(tuplify!(2, u32s(0))).zip()
                .simd_do_each(|x| { black_box(x); });
        })
    }
}
