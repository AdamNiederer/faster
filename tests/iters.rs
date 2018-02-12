#![feature(test)]

extern crate faster;

#[cfg(test)]
mod tests {
    use faster::*;

    #[test]
    #[cfg(not(feature = "no-std"))]
    fn zipped_stride_iters() {
        let mut vec = vec![0f32, 1f32, 2f32, 3f32];
        
        let simd = vec.as_mut_slice()
            .simd_iter_mut(f32s(0.0))
            .simd_for_each(|mut x| *x /= f32s(2f32));
        
        let scl = vec.iter_mut()
            .for_each(|x| *x /= 2f32);
        
        assert_eq!(simd, scl);
    }
}
