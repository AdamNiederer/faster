pub use crate::vecs::*;
pub use crate::std::simd::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};



impl_packed!(u8, u8s, u8x64, 1, 64, ["avx512"], ["avx1024"]);
impl_packed!(u8, u8s, u8x32, 1, 32, ["avx2"], ["avx512"]);
impl_packed!(u8, u8s, u8x16, 1, 16, [], ["avx2"]);
impl_packed!(i8, i8s, i8x64, 1, 64, ["avx512"], ["avx1024"]);
impl_packed!(i8, i8s, i8x32, 1, 32, ["avx2"], ["avx512"]);
impl_packed!(i8, i8s, i8x16, 1, 16, [], ["avx2"]);
impl_packed!(u16, u16s, u16x32, 2, 32, ["avx512"], ["avx1024"]);
impl_packed!(u16, u16s, u16x16, 2, 16, ["avx2"], ["avx512"]);
impl_packed!(u16, u16s, u16x8, 2, 8, [], ["avx2"]);
impl_packed!(i16, i16s, i16x32, 2, 32, ["avx512"], ["avx1024"]);
impl_packed!(i16, i16s, i16x16, 2, 16, ["avx2"], ["avx512"]);
impl_packed!(i16, i16s, i16x8, 2, 8, [], ["avx2"]);
impl_packed!(u32, u32s, u32x16, 4, 16, ["avx512"], ["avx1024"]);
impl_packed!(u32, u32s, u32x8, 4, 8, ["avx2"], ["avx512"]);
impl_packed!(u32, u32s, u32x4, 4, 4, [], ["avx2"]);
impl_packed!(i32, i32s, i32x16, 4, 16, ["avx512"], ["avx1024"]);
impl_packed!(i32, i32s, i32x8, 4, 8, ["avx2"], ["avx512"]);
impl_packed!(i32, i32s, i32x4, 4, 4, [], ["avx2"]);
impl_packed!(f32, f32s, f32x16, 4, 16, ["avx512"], ["avx1024"]);
impl_packed!(f32, f32s, f32x8, 4, 8, ["avx2"], ["avx512"]);
impl_packed!(f32, f32s, f32x4, 4, 4, [], ["avx2"]);
impl_packed!(u64, u64s, u64x8, 8, 8, ["avx512"], ["avx1024"]);
impl_packed!(u64, u64s, u64x4, 8, 4, ["avx2"], ["avx512"]);
impl_packed!(u64, u64s, u64x2, 8, 2, [], ["avx2"]);
impl_packed!(i64, i64s, i64x8, 8, 8, ["avx512"], ["avx1024"]);
impl_packed!(i64, i64s, i64x4, 8, 4, ["avx2"], ["avx512"]);
impl_packed!(i64, i64s, i64x2, 8, 2, [], ["avx2"]);
impl_packed!(f64, f64s, f64x8, 8, 8, ["avx512"], ["avx1024"]);
impl_packed!(f64, f64s, f64x4, 8, 4, ["avx2"], ["avx512"]);
impl_packed!(f64, f64s, f64x2, 8, 2, [], ["avx2"]);

#[cfg(test)]
mod tests {
    use super::Packed;
    use super::*;

    macro_rules! test_product {
        (($($el:tt),*), ($($vec:tt),*), ($($fn:tt),*), ($($sum:tt),*)) => (
            $(
                #[test]
                fn $fn() {
                    assert_eq!($vec::splat(1i8 as $el).product(), $sum as $el);
                }
            )*
        )
    }

    // TODO: Do we need better test cases for this?
    test_product!((u8, u8, u8, i8, i8, i8, u16, u16, u16, i16, i16, i16, u32, u32, u32, i32, i32, i32, f32, f32, f32, u64, u64, u64, i64, i64, i64, f64, f64, f64),
                  (u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2),
                  (scalar_product_u8x64, scalar_product_u8x32, scalar_product_u8x16, scalar_product_i8x64, scalar_product_i8x32, scalar_product_i8x16, scalar_product_u16x32, scalar_product_u16x16, scalar_product_u16x8, scalar_product_i16x32, scalar_product_i16x16, scalar_product_i16x8, scalar_product_u32x16, scalar_product_u32x8, scalar_product_u32x4, scalar_product_i32x16, scalar_product_i32x8, scalar_product_i32x4, scalar_product_f32x16, scalar_product_f32x8, scalar_product_f32x4, scalar_product_u64x8, scalar_product_u64x4, scalar_product_u64x2, scalar_product_i64x8, scalar_product_i64x4, scalar_product_i64x2, scalar_product_f64x8, scalar_product_f64x4, scalar_product_f64x2),
                  (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1));
}
