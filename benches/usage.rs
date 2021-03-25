#![feature(stdsimd, test)]

#[cfg(test)]
extern crate test;
#[macro_use]
extern crate faster;

#[cfg(test)]
mod usage {
    use faster::prelude::*;
    use test::{black_box, Bencher};

    #[bench]
    #[cfg(feature = "std")]
    fn nop_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [0u8; 1024]
                    .simd_iter(u8s(0))
                    .simd_map(|v| v)
                    .scalar_collect(),
            )
        });
    }

    #[bench]
    #[cfg(feature = "std")]
    fn nop_scalar(b: &mut Bencher) {
        b.iter(|| black_box([0u8; 1024].iter().map(|e| *e).collect::<Vec<u8>>()));
    }

    #[bench]
    #[cfg(feature = "std")]
    fn map_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024]
                    .simd_iter(f32s(0.0))
                    .simd_map(|v| {
                        f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0)
                    })
                    .scalar_collect(),
            )
        })
    }

    #[bench]
    fn for_fill_simd(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).enumerate() {
                let ans = f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0);
                ans.store(&mut out, i * ans.width());
            }
            out
        })
    }

    #[bench]
    fn for_unrolled_simd(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).unroll(8).enumerate() {
                macro_rules! compute {
                    ($($idx:expr),*) => {
                        $(
                            let a = f32s(9.0) * v[$idx].abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0);
                            a.store(&mut out, (i + $idx) * a.width());
                        )*
                    }
                }
                compute!(0, 1, 2, 3, 4, 5, 6, 7);
            }
            out
        })
    }

    #[bench]
    fn high_latency(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            [-123.456f32; 1024]
                .simd_iter(f32s(0.0))
                .simd_map(|v| {
                    let (a, b) = (v * f32s(1.20)).upcast();
                    (a.sqrt() * f64s(3.141592653589793))
                        .saturating_downcast(b.sqrt() * f64s(3.141592653589793))
                })
                .scalar_fill(&mut out);
        });
    }

    #[bench]
    fn high_latency_for(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).enumerate() {
                let (a, b) = (v * f32s(1.20)).upcast();
                let ans = (a.sqrt() * f64s(3.141592653589793))
                    .saturating_downcast(b.sqrt() * f64s(3.141592653589793));
                ans.store(&mut out, i * ans.width());
            }
        })
    }

    #[bench]
    fn high_latency_unrolled(b: &mut Bencher) {
        let mut out = [0f32; 1024];
        b.iter(|| {
            for (i, v) in [-123.456f32; 1024]
                .simd_iter(f32s(0.0))
                .unroll(8)
                .enumerate()
            {
                macro_rules! compute {
                    ($($idx:expr),*) => {
                        $(
                            let (a, b) = (v[$idx] * f32s(1.20)).upcast();
                            let ans = (a.sqrt() * f64s(3.141592653589793)).saturating_downcast(
                                b.sqrt() * f64s(3.141592653589793));
                            ans.store(&mut out, (i + $idx) * ans.width());
                        )*
                    }
                }
                compute!(0, 1, 2, 3, 4, 5, 6, 7);
            }
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn for_simd(b: &mut Bencher) {
        b.iter(|| {
            let mut out = vec![0f32; 1024];
            for (i, v) in [-123.456f32; 1024].simd_iter(f32s(0.0)).enumerate() {
                let ans = f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0);
                ans.store(&mut out, i * ans.width());
            }
            out
        })
    }

    // #[bench]
    // fn map_eager_simd(b: &mut Bencher) {
    //     let mut into = [0f32; 1024];
    //     b.iter(|| {
    //         black_box(
    //             [-123.456f32; 1024].simd_iter()
    //                 .simd_map_into(&mut into, f32s(0.0), |v| {
    //                     f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()
    //                         - f32s(4.0) - f32s(2.0)
    //                 }));
    //     })
    // }

    #[bench]
    fn map_fill_simd(b: &mut Bencher) {
        let mut into = [0f32; 1024];
        b.iter(|| {
            black_box(
                [-123.456f32; 1024]
                    .simd_iter(f32s(0.0))
                    .simd_map(|v| {
                        f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0)
                    })
                    .scalar_fill(&mut into),
            );
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn map_uneven_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1025]
                    .simd_iter(f32s(0.0))
                    .simd_map(|v| {
                        f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() - f32s(4.0) - f32s(2.0)
                    })
                    .scalar_collect(),
            )
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn map_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024]
                    .iter()
                    .map(|v| 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() - 4.0 - 2.0)
                    .collect::<Vec<f32>>(),
            )
        });
    }

    #[bench]
    fn reduce_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024]
                    .simd_iter(f32s(0.0))
                    .simd_reduce(f32s(0.0), |a, v| {
                        a + f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()
                    })
                    .sum(),
            )
        })
    }

    #[bench]
    fn reduce_uneven_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1025]
                    .simd_iter(f32s(0.0))
                    .simd_reduce(f32s(0.0), |a, v| {
                        a + f32s(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()
                    })
                    .sum(),
            )
        })
    }

    #[bench]
    fn reduce_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box([-123.456f32; 1024].iter().fold(0.0, |a, v| {
                a + 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt()
            }))
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn determinant3_simd(b: &mut Bencher) {
        // TODO: Why is this so slow? Cache locality?
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1026][..])
                    .stride_nine(tuplify!(9, f32s(0.0)))
                    .zip()
                    .simd_map(|(a, b, c, d, e, f, g, h, i)| {
                        (a * e * i) + (b * f * g) + (c * d * h)
                            - (c * e * g)
                            - (b * d * i)
                            - (a * f * h)
                    })
                    .scalar_collect(),
            )
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn determinant3_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1026]
                    .chunks(9)
                    .map(|m| {
                        (m[0] * m[4] * m[8]) + (m[1] * m[5] * m[6]) + (m[2] * m[3] * m[7])
                            - (m[2] * m[4] * m[6])
                            - (m[1] * m[3] * m[8])
                            - (m[0] * m[5] * m[7])
                    })
                    .collect::<Vec<f32>>(),
            )
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn determinant2_simd(b: &mut Bencher) {
        // TODO: Why is this so slow? Cache locality?
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..])
                    .stride_four(tuplify!(4, f32s(0.0)))
                    .zip()
                    .simd_map(|(a, b, c, d)| a * d - b * c)
                    .scalar_collect(),
            )
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn determinant2_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024]
                    .chunks(4)
                    .map(|m| m[0] * m[3] - m[1] * m[2])
                    .collect::<Vec<f32>>(),
            )
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn zip_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123i32; 1024][..])
                    .stride_two(tuplify!(2, i32s(0)))
                    .zip()
                    .simd_map(|(a, b)| {
                        let (aa, ab): (i64s, i64s) = a.upcast();
                        let (ba, bb): (i64s, i64s) = b.upcast();
                        (aa.abs() + ba.abs()).saturating_downcast(ab.abs() + bb.abs())
                    })
                    .scalar_collect(),
            )
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn zip_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123i32; 1024]
                    .chunks(2)
                    .map(|a| ((a[0] as f64).abs() + (a[1] as f64).abs()) as f32)
                    .collect::<Vec<f32>>(),
            )
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn zip_nop_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 1024][..])
                    .stride_two(tuplify!(2, f32s(0.0)))
                    .zip()
                    .simd_map(|(a, b)| a + b)
                    .scalar_collect(),
            )
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn zip_nop_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                [-123.456f32; 1024]
                    .chunks(2)
                    .map(|a| a[0] + a[1])
                    .collect::<Vec<f32>>(),
            )
        })
    }

    #[bench]
    fn base100_enc_simd(b: &mut Bencher) {
        let mut out = [0u8; 4096];
        b.iter(|| {
            let mut i = 0;
            [123u8; 1024].simd_iter(u8s(0)).simd_do_each(|v| {
                let (a, b): (u16s, u16s) = v.upcast();
                let third = ((a + u16s(55)) / u16s(64) + u16s(143))
                    .saturating_downcast((b + u16s(55)) / u16s(64) + u16s(143));
                let fourth = ((v + u8s(55)) & u8s(0x3f)) + u8s(128);

                // Make some room for interleaving
                let (ta, tb) = third.upcast();
                let (fa, fb) = fourth.upcast();

                // Interleave third and fourth bytes
                let third_fourth_a = ta.swap_bytes().merge_interleaved(fa);
                let third_fourth_b = tb.swap_bytes().merge_interleaved(fb);

                // Make some more room for another interleaving
                let (tfa, tfb) = third_fourth_a.be_u16s().upcast();
                let (tfc, tfd) = third_fourth_b.be_u16s().upcast();

                // Interleave a constant 0xf09f with the third and fourth bytes,
                // and store into out buffer
                u32s(0xf09f0000)
                    .merge_interleaved(tfa)
                    .be_u8s()
                    .store(&mut out, i);
                u32s(0xf09f0000)
                    .merge_interleaved(tfb)
                    .be_u8s()
                    .store(&mut out, i + v.width());
                u32s(0xf09f0000)
                    .merge_interleaved(tfc)
                    .be_u8s()
                    .store(&mut out, i + v.width() * 2);
                u32s(0xf09f0000)
                    .merge_interleaved(tfd)
                    .be_u8s()
                    .store(&mut out, i + v.width() * 3);
                i += v.width() * 4;
            });
            out
        })
    }

    #[bench]
    fn base100_enc_scalar(b: &mut Bencher) {
        let mut out = [0u8; 4096];
        b.iter(|| {
            for (i, ch) in [123u8; 1024].iter().enumerate() {
                out[4 * i + 0] = 0xf0;
                out[4 * i + 1] = 0x9f;
                out[4 * i + 2] = ((((*ch as u16).wrapping_add(55)) >> 6) + 143) as u8;
                out[4 * i + 3] = (ch.wrapping_add(55) & 0x3f).wrapping_add(128);
            }
            out
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn stride_zip_naive(b: &mut Bencher) {
        let a = [0u8; 4096];
        b.iter(|| {
            (&a[..])
                .stride_two(tuplify!(2, u8s(0)))
                .zip()
                .simd_map(|(a, b)| a + b)
                .scalar_collect()
        })
    }

    #[bench]
    #[cfg(feature = "std")]
    fn stride_zip(b: &mut Bencher) {
        let a = [0u8; 4096];
        b.iter(|| {
            (&a[..])
                .simd_iter(u8s(0))
                .stride_zip()
                .simd_map(|(a, b)| a + b)
                .scalar_collect()
        })
    }
}
