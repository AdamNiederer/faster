use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::merge::*;


// Will produce fallback implementations only, so we get away with __undefined. 
impl_packed_merge!(u8x16, u8x16,  u8, __undefined, "__undefined", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(u16x8, u16x8, u16, __undefined, "__undefined", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(u32x4, u32x4, u32, __undefined, "__undefined", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(u64x2, u64x2, u64, __undefined, "__undefined", (0), (1), 0, 1);
impl_packed_merge!(i8x16, u8x16,  u8, __undefined, "__undefined", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(i16x8, u16x8, u16, __undefined, "__undefined", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(i32x4, u32x4, u32, __undefined, "__undefined", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(i64x2, u64x2, u64, __undefined, "__undefined", (0), (1), 0, 1);
impl_packed_merge!(f32x4, u32x4, u32, __undefined, "__undefined", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(f64x2, u64x2, u64, __undefined, "__undefined", (0), (1), 0, 1);


mod tests {
    #![allow(unused_imports)]

    use crate::prelude::*;
    use crate::arch::current::vecs::*;

    // TODO: Which ones do we really need?
    test_packed_merge!(
        (u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, f32x4, u64x2, i64x2, f64x2),
        (merge_u8x16, merge_i8x16, merge_u16x8, merge_i16x8, merge_u32x4, merge_i32x4, merge_f32x4, merge_u64x2, merge_i64x2, merge_f64x2)
    );
}
