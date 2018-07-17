#![feature(stdsimd, test)]

#[cfg(test)] extern crate test;
#[macro_use] extern crate faster;

#[cfg(test)]
mod destride {
    use faster::prelude::*;
    use test::{Bencher, black_box};

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
