use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::vec_patterns::*;
use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::intrin::transmute::*;
use crate::intrin::merge::*;
use crate::std::mem::transmute;


// TODO: The AVX-512 version of this macro doesn't work; impl when stdsimd gets
// around to it (and when I have some hardware to test it on).
impl_packed_merge!(u8x16, u8x16, u8, _mm_blendv_epi8, "sse4.1", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(u8x32, u8x32, u8, _mm256_blendv_epi8, "avx2", (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
impl_packed_merge!(u8x64, u8x64, u8, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), (32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);

impl_packed_merge!(u16x8, u16x8, u16, _mm_blendv_epi8, "sse4.1", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(u16x16, u16x16, u16, _mm256_blendv_epi8, "avx2", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(u16x32, u16x32, u16, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);

impl_packed_merge!(u32x4, u32x4, u32, _mm_blendv_epi8, "sse4.1", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(u32x8, u32x8, u32, _mm256_blendv_epi8, "avx2", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(u32x16, u32x16, u32, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

impl_packed_merge!(u64x2, u64x2, u64, _mm_blendv_epi8, "sse4.1", (0), (1), 0, 1);
impl_packed_merge!(u64x4, u64x4, u64, _mm256_blendv_epi8, "avx2", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(u64x8, u64x8, u64, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);

impl_packed_merge!(i8x16, u8x16, u8, _mm_blendv_epi8, "sse4.1", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(i8x32, u8x32, u8, _mm256_blendv_epi8, "avx2", (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
impl_packed_merge!(i8x64, u8x64, u8, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), (32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);

impl_packed_merge!(i16x8, u16x8, u16, _mm_blendv_epi8, "sse4.1", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(i16x16, u16x16, u16, _mm256_blendv_epi8, "avx2", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(i16x32, u16x32, u16, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);

impl_packed_merge!(i32x4, u32x4, u32, _mm_blendv_epi8, "sse4.1", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(i32x8, u32x8, u32, _mm256_blendv_epi8, "avx2", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(i32x16, u32x16, u32, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

impl_packed_merge!(i64x2, u64x2, u64, _mm_blendv_epi8, "sse4.1", (0), (1), 0, 1);
impl_packed_merge!(i64x4, u64x4, u64, _mm256_blendv_epi8, "avx2", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(i64x8, u64x8, u64, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);

impl_packed_merge!(f32x4, u32x4, u32, _mm_blendv_epi8, "sse4.1", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(f32x8, u32x8, u32, _mm256_blendv_epi8, "avx2", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(f32x16, u32x16, u32, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

impl_packed_merge!(f64x2, u64x2, u64, _mm_blendv_epi8, "sse4.1", (0), (1), 0, 1);
impl_packed_merge!(f64x4, u64x4, u64, _mm256_blendv_epi8, "avx2", (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(f64x8, u64x8, u64, _mm512_mask_mov_epi8, "avx512-butnotyet", (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);

mod tests {
    use crate::prelude::*;
    use crate::arch::current::vecs::*;

    test_packed_merge!(
        (u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2),
        (merge_u8x64, merge_u8x32, merge_u8x16, merge_i8x64, merge_i8x32, merge_i8x16, merge_u16x32, merge_u16x16, merge_u16x8, merge_i16x32, merge_i16x16, merge_i16x8, merge_u32x16, merge_u32x8, merge_u32x4, merge_i32x16, merge_i32x8, merge_i32x4, merge_f32x16, merge_f32x8, merge_f32x4, merge_u64x8, merge_u64x4, merge_u64x2, merge_i64x8, merge_i64x4, merge_i64x2, merge_f64x8, merge_f64x4, merge_f64x2));
}
