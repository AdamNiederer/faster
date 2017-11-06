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

    #[bench]
    fn bench_work_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 128][..]).simd_iter()
                    .map(|v| { f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() -
                               f32s::splat(4.0) - f32s::splat(2.0) })
                    .scalar_collect::<Vec<f32>>());
        })
    }

    #[bench]
    fn bench_work_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 128][..]).iter()
                    .map(|v| { 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() -
                               4.0 - 2.0 })
                    .collect::<Vec<f32>>());
        });
    }
}
