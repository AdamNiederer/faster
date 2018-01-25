#![feature(test)]

extern crate faster;

#[cfg(test)]
mod tests {
    use faster::{IntoPackedRefIterator, IntoPackedZip, PackedZippedIterator, PackedIterator, Packed, f64s, i32s};


    /// Tests a number of simple kernel computations with int32 values. 
    #[test]
    fn zip_variable_kernel_i32() {
        
        for n in 0 .. 16 {

            let mut vec_of_1 = Vec::new();
            let mut vec_of_3 = Vec::new();
            
            for _ in 0 .. n {
                vec_of_1.push(1);
                vec_of_3.push(3);
            }

            let slice_1 = &vec_of_1[..];
            let slice_3 = &vec_of_3[..];

            // Should produce n times (1 - 3) * (1 - 3) == n * 4 for each element
            let sum: i32 = (slice_1.simd_iter(), slice_3.simd_iter()).zip()
                .simd_map((i32s::splat(0), i32s::splat(0)), |(a,b)| (a - b) * (a - b) )
                .sum();

            assert_eq!(sum, n * 4);

            
            // Same as above, but this time we reduce with simd_reduce
            let sum: i32 = (slice_1.simd_iter(), slice_3.simd_iter()).zip()
                .simd_map((i32s::splat(0), i32s::splat(0)), |(a,b)| (a - b) * (a - b) )
                .simd_reduce(i32s::splat(0), i32s::splat(0), |a, v| a + v )
                .sum();

            assert_eq!(sum, n * 4);
        }
     
    }

    /// Tests a number of simple kernel computations with f64 values.
    #[test]
    fn zip_variable_kernel_f64() {

        for n in 0 .. 16 {

            let mut vec_of_1 = Vec::new();
            let mut vec_of_3 = Vec::new();

            for _ in 0 .. n {
                vec_of_1.push(1.0);
                vec_of_3.push(3.0);
            }

            let slice_1 = &vec_of_1[..];
            let slice_3 = &vec_of_3[..];

            // Should produce n times (1 - 3) * (1 - 3) == n * 4 for each element
            let sum_scalar: f64 = (slice_1.simd_iter(), slice_3.simd_iter()).zip()
                .simd_map((f64s::splat(0.0), f64s::splat(0.0)), |(a,b)| (a - b) * (a - b) )
                .sum();


            // Same as above, but this time we reduce with simd_reduce
            let sum_simd: f64 = (slice_1.simd_iter(), slice_3.simd_iter()).zip()
                .simd_map((f64s::splat(0.0), f64s::splat(0.0)), |(a,b)| (a - b) * (a - b) )
                .simd_reduce(f64s::splat(0.0), f64s::splat(0.0), |a, v| a + v )
                .sum();

            // Same example as above, but instead of requiring hard number, we just want 
            // results to be the same (float math ...)
            assert_eq!(sum_scalar, sum_simd);
        }

    }
    

    /// Tests uneven vector lengths
    #[test]
    fn zip_uneven() {
        for n in 0 .. 16 {
            let mut vec_short = Vec::new();
            let mut vec_long = Vec::new();

            for _ in 0..n {
                vec_short.push(0);
                vec_long.push(1);
            }

            // Add one extra to long
            vec_long.push(1);

            let slice_long = &vec_long[..];
            let slice_short = &vec_short[..];

            let sum_scalar: i32 = (slice_long.simd_iter(), slice_short.simd_iter()).zip()
                .simd_map((i32s::splat(0), i32s::splat(1)), |(a, b)| (a - b) * (a - b) )
                .sum();

            let sum_simd: i32 = (slice_long.simd_iter(), slice_short.simd_iter()).zip()
                .simd_map((i32s::splat(0), i32s::splat(1)), |(a, b)| (a - b) * (a - b) )
                .simd_reduce(i32s::splat(0), i32s::splat(0), |a, v| a + v )
                .sum();

            assert_eq!(sum_scalar, sum_simd);
        }
    }

}
