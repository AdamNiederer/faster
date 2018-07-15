use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::vec_patterns::*;
use crate::intrin::transmute::*;
use crate::intrin::merge::*;
use crate::std::mem::transmute;


// Will produce fallback implementations only, so we get away with __undefined. 
impl_packed_merge!(u8x16, u8x16,  u8, __undefined, "undefined", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(u16x8, u16x8, u16, __undefined, "undefined", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(u32x4, u32x4, u32, __undefined, "undefined", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(u64x2, u64x2, u64, __undefined, "undefined", (0), (1), 0, 1);
impl_packed_merge!(i8x16, u8x16,  u8, __undefined, "undefined", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(i16x8, u16x8, u16, __undefined, "undefined", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(i32x4, u32x4, u32, __undefined, "undefined", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(i64x2, u64x2, u64, __undefined, "undefined", (0), (1), 0, 1);
impl_packed_merge!(f32x4, u32x4, u32, __undefined, "undefined", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(f64x2, u64x2, u64, __undefined, "undefined", (0), (1), 0, 1);


mod tests {
    use crate::prelude::*;
    use crate::arch::current::vecs::*;

    macro_rules! test_packed_merge {
        (($($vec:tt),*), ($($fn:ident),*)) => {
            $(
                #[test]
                fn $fn() {
                    let asc = 30i32 as <$vec as Packed>::Scalar;
                    let bsc = 5i32 as <$vec as Packed>::Scalar;
                    let a = $vec::splat(asc);
                    let b = $vec::splat(bsc);
                    assert_eq!(a.merge_interleaved(b), $vec::interleave(asc, bsc));
                    assert_eq!(b.merge_interleaved(a), $vec::interleave(bsc, asc));

                    assert_eq!(a.merge_halves(b), $vec::halfs(asc, bsc));
                    assert_eq!(b.merge_halves(a), $vec::halfs(bsc, asc));

                    for i in 0..$vec::WIDTH {
                        assert_eq!(a.merge_partitioned(b, i), $vec::partition(asc, bsc, i));
                        assert_eq!(b.merge_partitioned(a, i), $vec::partition(bsc, asc, i));
                    }
                }
            )*
        }
    }
    test_packed_merge!(
        (u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2),
        (merge_u8x64, merge_u8x32, merge_u8x16, merge_i8x64, merge_i8x32, merge_i8x16, merge_u16x32, merge_u16x16, merge_u16x8, merge_i16x32, merge_i16x16, merge_i16x8, merge_u32x16, merge_u32x8, merge_u32x4, merge_i32x16, merge_i32x8, merge_i32x4, merge_f32x16, merge_f32x8, merge_f32x4, merge_u64x8, merge_u64x4, merge_u64x2, merge_i64x8, merge_i64x4, merge_i64x2, merge_f64x8, merge_f64x4, merge_f64x2));
}
