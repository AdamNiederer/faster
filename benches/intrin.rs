#![feature(test, stdsimd)]

#[cfg(test)] extern crate test;
extern crate faster;

const ARRAY_F32: &[f32] = &[-123.456f32; 1024];

macro_rules! bench_intrin_1 {
    ($simd_name:ident, $simd_fn:expr, $scalar_name:ident, $scalar_fn:expr) => {
        #[bench]
        #[cfg(feature = "std")]
        fn $scalar_name(b: &mut Bencher) {
            b.iter(|| { black_box(
               crate::ARRAY_F32.iter().map(|v| { $scalar_fn(*v) }).collect::<Vec<f32>>()
            )})
        }

        #[bench]
        #[cfg(feature = "std")]
        fn $simd_name(b: &mut Bencher) {
            b.iter(|| { black_box(
                crate::ARRAY_F32.simd_iter(f32s(0.0)).simd_map(|v| { $simd_fn(v) }).scalar_collect()
            )});
        }
    }
}

macro_rules! bench_intrin_2 {
    ($simd_name:ident, $simd_fn:ident, $scalar_name:ident, $scalar_fn:ident) => {
        #[bench]
        #[cfg(feature = "std")]
        fn $scalar_name(b: &mut Bencher) {
            b.iter(|| { black_box(
                crate::ARRAY_F32.iter().map(|v| { v.$scalar_fn(*v) }).collect::<Vec<f32>>()
            )})
        }

        #[bench]
        #[cfg(feature = "std")]
        fn $simd_name(b: &mut Bencher) {
            b.iter(|| { black_box(
                crate::ARRAY_F32.simd_iter(f32s(0.0)).simd_map(|v| {v.$simd_fn(v) }).scalar_collect()
            )});
        }
    }
}


#[cfg(test)]
mod intrin {
    use faster::prelude::*;
    use test::{Bencher, black_box};

    bench_intrin_1!(abs_simd, |x: f32s| x.abs(), abs_scala, |x: f32| x.abs());
    bench_intrin_1!(ceil_simd, |x: f32s| x.ceil(), ceil_scala, |x: f32| x.ceil());
    bench_intrin_1!(floor_simd, |x: f32s| x.floor(), floor_scala, |x: f32| x.floor());
    bench_intrin_2!(min_simd, min, min_scala, min);
    bench_intrin_2!(max_simd, max, max_scala, max);
    bench_intrin_1!(recip_simd, |x: f32s| x.recip(), recip_scala, |x: f32| 1.0f32 / x);
    bench_intrin_1!(round_simd, |x: f32s| x.round(), round_scala, |x: f32| x.round());
    bench_intrin_1!(sqrt_simd, |x: f32s| x.sqrt(), sqrt_scala, |x: f32| x.sqrt());
    bench_intrin_1!(trunc_simd, |x: f32s| x.trunc(), trunc_scala, |x: f32| x.trunc());
}
