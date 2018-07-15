use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::endian::*;

impl_packed_swap_bytes!(u8x16, u8x16, "__undefined", __undefined,
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swap_bytes!(i8x16, u8x16, "__undefined", __undefined,
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swap_bytes!(u16x8, u8x16, "__undefined", __undefined,
                        (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14),
                        (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swap_bytes!(i16x8, u8x16, "__undefined", __undefined,
                        (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14),
                        (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swap_bytes!(u32x4, u8x16, "__undefined", __undefined,
                        (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12),
                        (0, 1, 2, 3));
impl_packed_swap_bytes!(i32x4, u8x16, "__undefined", __undefined,
                        (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12),
                        (0, 1, 2, 3));
impl_packed_swap_bytes!(u64x2, u8x16, "__undefined", __undefined,
                        (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8),
                        (0, 1));
impl_packed_swap_bytes!(i64x2, u8x16, "__undefined", __undefined,
                        (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8),
                        (0, 1));


mod tests {
    #![allow(unused_imports)]

    use crate::prelude::*;
    use crate::arch::current::vecs::*;

    test_packed_swap_bytes!((u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, u64x2, i64x2),
                             (swap_bytes_u8x16, swap_bytes_i8x16, swap_bytes_u16x8, swap_bytes_i16x8, swap_bytes_u32x4, swap_bytes_i32x4, swap_bytes_u64x2, swap_bytes_i64x2));
}
