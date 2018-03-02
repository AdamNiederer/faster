#![feature(test)]

extern crate faster;

#[cfg(test)]
mod tests {
    use faster::*;

    #[test]
    #[cfg(not(feature = "no-std"))]
    fn in_place_mutation() {
        let test = |mut vec: Vec<f32>| {
            let mut scl = vec.clone();
            vec.simd_iter_mut(f32s(0.0))
                .simd_for_each(|x| *x /= f32s(2f32));

            scl.iter_mut()
                .for_each(|x| *x /= 2f32);

            assert_eq!(vec, scl);
        };

        let vec: Vec<f32> = (0..(f32s::WIDTH - 1)).map(|x| x as f32).collect();
        test(vec);

        let vec: Vec<f32> = (0..f32s::WIDTH).map(|x| x as f32).collect();
        test(vec);

        let vec: Vec<f32> = (0..(f32s::WIDTH + 1)).map(|x| x as f32).collect();
        test(vec);
    }

    #[test]
    fn simd_reduce() {
        let vec = [2u32; 129];
        let sum = vec.simd_iter(u32s(0u32)).simd_reduce(u32s(0u32), |acc, x| acc + x).sum();
        assert_eq!(sum, 2 * 129);
    }

}
