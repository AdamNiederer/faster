#![feature(test)]

extern crate faster;

#[cfg(test)]
mod tests {
    use faster::*;

    #[test]
    #[cfg(not(feature = "no-std"))]
    fn zipped_stride_iters() {
        let matrices = [1i8, 2, 3, 4, 5, 6, 7, 8, 9][..].iter().cycle().take(9 * 100).map(|i| i.clone()).collect::<Vec<_>>();
        let determinants = (&matrices[..]).stripe_nine(tuplify!(9, i8s(0))).zip()
            .simd_map(|(a, b, c, d, e, f, g, h, i)| {
                assert_eq!(a.extract((a.width() - 1) as u32), 1);
                assert_eq!(b.extract((b.width() - 1) as u32), 2);
                assert_eq!(c.extract((c.width() - 1) as u32), 3);
                assert_eq!(d.extract((d.width() - 1) as u32), 4);
                assert_eq!(e.extract((e.width() - 1) as u32), 5);
                assert_eq!(f.extract((f.width() - 1) as u32), 6);
                assert_eq!(g.extract((g.width() - 1) as u32), 7);
                assert_eq!(h.extract((h.width() - 1) as u32), 8);
                assert_eq!(i.extract((i.width() - 1) as u32), 9);
                (a * e * i) + (b * f * g) + (c * d * h) - (c * e * g) - (b * d * i) - (a * f * h)
            }).scalar_collect();
        assert!(determinants.iter().fold(true, |acc, x| acc && x == &0));

        let matrices = [1i16, 0, 0, 0, 5, 4, 2, 3, 0][..].iter().cycle().take(9 * 100).map(|i| i.clone()).collect::<Vec<_>>();
        let determinants = (&matrices[..]).stripe_nine(tuplify!(9, i16s(0))).zip()
            .simd_map(|(a, b, c, d, e, f, g, h, i)| {
                (a * e * i) + (b * f * g) + (c * d * h) - (c * e * g) - (b * d * i) - (a * f * h)
            }).scalar_collect();
        assert!(determinants.iter().fold(true, |acc, x| { println!("{:?}", x); acc && x == &-12 }));
    }

    #[test]
    #[cfg(not(feature = "no-std"))]
    fn zipped_heterogeneous_iters() {
        let to_stripe = [1i8, 2, 3, 4, 5, 6, 7, 8][..].iter().cycle().take(512).map(|i| i.clone()).collect::<Vec<_>>();
        let (a, b) = to_stripe.stripe_two(tuplify!(2, i8s(0)));
        let standard_iter_a = vec!(3i8; 256).into_simd_iter(i8s(0));
        let standard_iter_b = vec!(7i8; 256).into_simd_iter(i8s(0));

        let a_times_three = (a, standard_iter_a).zip()
            .simd_map(|(s, c)| s * c)
            .scalar_collect();

        let b_times_three = (b, standard_iter_b).zip()
            .simd_map(|(s, c)| s * c)
            .scalar_collect();

        let a_times_three_check = to_stripe.chunks(2).map(|c| c[0] * 3);
        let b_times_three_check = to_stripe.chunks(2).map(|c| c[1] * 7);

        assert!(a_times_three_check.zip(a_times_three)
                .fold(true, |acc, (a, b)| acc && a == b));

        assert!(b_times_three_check.zip(b_times_three)
                .fold(true, |acc, (a, b)| acc && a == b));
    }
}
