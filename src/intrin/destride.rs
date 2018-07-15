

pub trait Destride : Sized {
    fn destride_two(self, other: Self) -> (Self, Self);
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self);
}



// TODO: LLVM actually autovectorizes our polyfills, but we should still have an
// explicit implementation for everything

macro_rules! destride_two_polyfill {
    ($self:expr, $other:expr, $($n:expr),*) => {
        (Self::new($($self.extract($n)),*,
                   $($other.extract($n)),*),
         Self::new($($self.extract($n + 1)),*,
                   $($other.extract($n + 1)),*))
    }
}

macro_rules! destride_four_polyfill {
    ($self:expr, $b:expr, $c:expr, $d:expr, $($n:expr),*) => {
        (Self::new($($self.extract($n)),*,
                   $($b.extract($n)),*,
                   $($c.extract($n)),*,
                   $($d.extract($n)),*),
         Self::new($($self.extract($n + 1)),*,
                   $($b.extract($n + 1)),*,
                   $($c.extract($n + 1)),*,
                   $($d.extract($n + 1)),*),
         Self::new($($self.extract($n + 2)),*,
                   $($b.extract($n + 2)),*,
                   $($c.extract($n + 2)),*,
                   $($d.extract($n + 2)),*),
         Self::new($($self.extract($n + 3)),*,
                   $($b.extract($n + 3)),*,
                   $($c.extract($n + 3)),*,
                   $($d.extract($n + 3)),*))
    }
}



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

