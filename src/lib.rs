#![feature(cfg_target_feature)]
#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(inclusive_range))]

#[cfg(test)] extern crate test;

extern crate stdsimd;

mod vecs;
mod iters;
mod intrin;

pub use iters::*;
pub use vecs::{u8s, i8s, u16s, i16s, u32s, i32s, f32s, u64s, i64s, f64s};
pub use intrin::*;

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
