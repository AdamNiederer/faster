use vecs::*;
use stdsimd::vendor::*;
use intrin::transmute::*;
use core_or_std::mem::transmute;

// TODO: Does this actually provide any performance benefit over swap_bytes?
//
// pub trait Reendianize {
//     fn reendianize(&self) -> Self;
// }

// impl Reendianize for u64 {
//     #[inline(always)]
//     #[cfg(any(target_arch = "x86", target_arch="x86_64"))]
//     fn reendianize(&self) -> Self {
//         unsafe { transmute(_bswap64(transmute(*self))) }
//     }

//     #[inline(always)]
//     #[cfg(not(any(target_arch = "x86", target_arch="x86_64")))]
//     fn reendianize(&self) -> Self {
//         ((self << 56) & 0xFF00000000000000u64) |
//         ((self << 40) & 0x00FF000000000000u64) |
//         ((self << 24) & 0x0000FF0000000000u64) |
//         ((self << 8) & 0x000000FF00000000u64) |
//         ((self >> 8) & 0x00000000FF000000u64) |
//         ((self >> 24) & 0x0000000000FF0000u64) |
//         ((self >> 40) & 0x000000000000FF00u64) |
//         ((self >> 56) & 0x00000000000000FFu64)
//     }
// }

// impl Reendianize for u32 {
//     #[inline(always)]
//     #[cfg(any(target_arch = "x86", target_arch="x86_64"))]
//     fn reendianize(&self) -> Self {
//         unsafe { transmute(_bswap(transmute(*self))) }
//     }

//     #[inline(always)]
//     #[cfg(not(any(target_arch = "x86", target_arch="x86_64")))]
//     fn reendianize(&self) -> Self {
//         ((self << 24) & 0xFF000000u32) |
//         ((self << 8) & 0x00FF0000u32) |
//         ((self >> 8) & 0x0000FF00u32) |
//         ((self >> 24) & 0x000000FFu32)
//     }
// }

// impl Reendianize for u16 {
//     #[inline(always)]
//     fn reendianize(&self) -> Self {
//         ((self << 8) & 0xFF00u16) |
//         ((self >> 8) & 0x00FFu16)
//     }
// }

// impl Reendianize for u8 {
//     #[inline(always)]
//     fn reendianize(&self) -> Self {
//         *self
//     }
// }

// impl Reendianize for i8 {
//     #[inline(always)]
//     fn reendianize(&self) -> Self {
//         unsafe { transmute(transmute::<Self, u8>(*self).reendianize()) }
//     }
// }

// impl Reendianize for i16 {
//     #[inline(always)]
//     fn reendianize(&self) -> Self {
//         unsafe { transmute(transmute::<Self, u16>(*self).reendianize()) }
//     }
// }

// impl Reendianize for i32 {
//     #[inline(always)]
//     fn reendianize(&self) -> Self {
//         unsafe { transmute(transmute::<Self, u32>(*self).reendianize()) }
//     }
// }

// impl Reendianize for i64 {
//     #[inline(always)]
//     fn reendianize(&self) -> Self {
//         unsafe { transmute(transmute::<Self, u64>(*self).reendianize()) }
//     }
// }

pub trait PackedSwizzle {
    fn flip(&self) -> Self;
}

pub trait PackedReendianize {
    fn reendianize(&self) -> Self;
}

macro_rules! impl_packed_swizzle {
    ($vec:tt, $uvec:tt, $feat:expr, $mmfn:tt, ($($c:expr),*), ($($a:expr, $b:expr),*)) => {
        impl PackedSwizzle for $vec {
            #[cfg(not(target_feature = $feat))]
            fn flip(&self) -> Self {
                $vec::new($(self.extract($b), self.extract($a)),*)
            }

            #[cfg(target_feature = $feat)]
            fn flip(&self) -> Self {
                unsafe {
                    transmute($mmfn(self.be_u8s(), $uvec::new($($c),*).be_u8s()))
                }
            }
        }
    }
}

macro_rules! impl_packed_reendianize {
    ($vec:tt, $uvec:tt, $feat:expr, $mmfn:tt, ($($c:expr),*), ($($a:expr, $b:expr),*)) => {
        impl PackedReendianize for $vec {
            #[cfg(not(target_feature = $feat))]
            fn reendianize(&self) -> Self {
                $vec::new($(self.extract($a).swap_bytes(),
                            self.extract($b).swap_bytes()),*)
            }

            #[cfg(target_feature = $feat)]
            fn reendianize(&self) -> Self {
                unsafe {
                    transmute($mmfn(self.be_u8s(), $uvec::new($($c),*).be_u8s()))
                }
            }
        }
    }
}

impl_packed_swizzle!(u8x64, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30, 33, 32, 35, 34, 37, 36, 39, 38, 41, 40, 43, 42, 45, 44, 47, 46, 49, 48, 51, 50, 53, 52, 55, 54, 57, 56, 59, 58, 61, 60, 63, 62),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63));
impl_packed_swizzle!(u8x32, u8x32, "avx2", _mm256_shuffle_epi8,
                     (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_swizzle!(u8x16, u8x16, "ssse3", _mm_shuffle_epi8,
                     (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swizzle!(i8x64, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30, 33, 32, 35, 34, 37, 36, 39, 38, 41, 40, 43, 42, 45, 44, 47, 46, 49, 48, 51, 50, 53, 52, 55, 54, 57, 56, 59, 58, 61, 60, 63, 62),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63));
impl_packed_swizzle!(i8x32, u8x32, "avx2", _mm256_shuffle_epi8,
                     (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_swizzle!(i8x16, u8x16, "ssse3", _mm_shuffle_epi8,
                     (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swizzle!(u16x32, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13, 18, 19, 16, 17, 22, 23, 20, 21, 26, 27, 24, 25, 30, 31, 28, 29, 34, 35, 32, 33, 38, 39, 36, 37, 42, 43, 40, 41, 46, 47, 44, 45, 50, 51, 48, 49, 54, 55, 52, 53, 58, 59, 56, 57, 62, 63, 60, 61),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_swizzle!(u16x16, u8x32, "avx2", _mm256_shuffle_epi8,
                     (2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13, 18, 19, 16, 17, 22, 23, 20, 21, 26, 27, 24, 25, 30, 31, 28, 29),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swizzle!(u16x8, u8x16, "ssse3", _mm_shuffle_epi8,
                     (2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13),
                     (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swizzle!(i16x32, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13, 18, 19, 16, 17, 22, 23, 20, 21, 26, 27, 24, 25, 30, 31, 28, 29, 34, 35, 32, 33, 38, 39, 36, 37, 42, 43, 40, 41, 46, 47, 44, 45, 50, 51, 48, 49, 54, 55, 52, 53, 58, 59, 56, 57, 62, 63, 60, 61),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_swizzle!(i16x16, u8x32, "avx2", _mm256_shuffle_epi8,
                     (2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13, 18, 19, 16, 17, 22, 23, 20, 21, 26, 27, 24, 25, 30, 31, 28, 29),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swizzle!(i16x8, u8x16, "ssse3", _mm_shuffle_epi8,
                     (2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13),
                     (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swizzle!(u32x16, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11, 20, 21, 22, 23, 16, 17, 18, 19, 28, 29, 30, 31, 24, 25, 26, 27, 36, 37, 38, 39, 32, 33, 34, 35, 44, 45, 46, 47, 40, 41, 42, 43, 52, 53, 54, 55, 48, 49, 50, 51, 60, 61, 62, 63, 56, 57, 58, 59),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swizzle!(u32x8, u8x32, "avx2", _mm256_shuffle_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11, 20, 21, 22, 23, 16, 17, 18, 19, 28, 29, 30, 31, 24, 25, 26, 27),
                     (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swizzle!(u32x4, u8x16, "ssse3", _mm_shuffle_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11),
                     (0, 1, 2, 3));
impl_packed_swizzle!(i32x16, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11, 20, 21, 22, 23, 16, 17, 18, 19, 28, 29, 30, 31, 24, 25, 26, 27, 36, 37, 38, 39, 32, 33, 34, 35, 44, 45, 46, 47, 40, 41, 42, 43, 52, 53, 54, 55, 48, 49, 50, 51, 60, 61, 62, 63, 56, 57, 58, 59),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swizzle!(i32x8, u8x32, "avx2", _mm256_shuffle_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11, 20, 21, 22, 23, 16, 17, 18, 19, 28, 29, 30, 31, 24, 25, 26, 27),
                     (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swizzle!(i32x4, u8x16, "ssse3", _mm_shuffle_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11),
                     (0, 1, 2, 3));
impl_packed_swizzle!(f32x16, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11, 20, 21, 22, 23, 16, 17, 18, 19, 28, 29, 30, 31, 24, 25, 26, 27, 36, 37, 38, 39, 32, 33, 34, 35, 44, 45, 46, 47, 40, 41, 42, 43, 52, 53, 54, 55, 48, 49, 50, 51, 60, 61, 62, 63, 56, 57, 58, 59),
                     (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swizzle!(f32x8, u8x32, "avx2", _mm256_shuffle_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11, 20, 21, 22, 23, 16, 17, 18, 19, 28, 29, 30, 31, 24, 25, 26, 27),
                     (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swizzle!(f32x4, u8x16, "ssse3", _mm_shuffle_epi8,
                     (4, 5, 6, 7, 0, 1, 2, 3, 12, 13, 14, 15, 8, 9, 10, 11),
                     (0, 1, 2, 3));
impl_packed_swizzle!(u64x8, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 31, 16, 17, 18, 19, 20, 21, 22, 23, 40, 41, 42, 43, 44, 45, 46, 47, 32, 33, 34, 35, 36, 37, 38, 39, 56, 57, 58, 59, 60, 61, 62, 63, 48, 49, 50, 51, 52, 53, 54, 55),
                     (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swizzle!(u64x4, u8x32, "avx2", _mm256_shuffle_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 31, 16, 17, 18, 19, 20, 21, 22, 23),
                     (0, 1, 2, 3));
impl_packed_swizzle!(u64x2, u8x16, "ssse3", _mm_shuffle_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7),
                     (0, 1));
impl_packed_swizzle!(i64x8, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 31, 16, 17, 18, 19, 20, 21, 22, 23, 40, 41, 42, 43, 44, 45, 46, 47, 32, 33, 34, 35, 36, 37, 38, 39, 56, 57, 58, 59, 60, 61, 62, 63, 48, 49, 50, 51, 52, 53, 54, 55),
                     (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swizzle!(i64x4, u8x32, "avx2", _mm256_shuffle_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 31, 16, 17, 18, 19, 20, 21, 22, 23),
                     (0, 1, 2, 3));
impl_packed_swizzle!(i64x2, u8x16, "ssse3", _mm_shuffle_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7),
                     (0, 1));
impl_packed_swizzle!(f64x8, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 31, 16, 17, 18, 19, 20, 21, 22, 23, 40, 41, 42, 43, 44, 45, 46, 47, 32, 33, 34, 35, 36, 37, 38, 39, 56, 57, 58, 59, 60, 61, 62, 63, 48, 49, 50, 51, 52, 53, 54, 55),
                     (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swizzle!(f64x4, u8x32, "avx2", _mm256_shuffle_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 31, 16, 17, 18, 19, 20, 21, 22, 23),
                     (0, 1, 2, 3));
impl_packed_swizzle!(f64x2, u8x16, "ssse3", _mm_shuffle_epi8,
                     (8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7),
                     (0, 1));

impl_packed_reendianize!(u8x64, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63));
impl_packed_reendianize!(u8x32, u8x32, "avx2", _mm256_shuffle_epi8,
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_reendianize!(u8x16, u8x16, "ssse3", _mm_shuffle_epi8,
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_reendianize!(i8x64, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63));
impl_packed_reendianize!(i8x32, u8x32, "avx2", _mm256_shuffle_epi8,
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_reendianize!(i8x16, u8x16, "ssse3", _mm_shuffle_epi8,
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_reendianize!(u16x32, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                         (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30, 33, 32, 35, 34, 37, 36, 39, 38, 41, 40, 43, 42, 45, 44, 47, 46, 49, 48, 51, 50, 53, 52, 55, 54, 57, 56, 59, 58, 61, 60, 63, 62),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_reendianize!(u16x16, u8x32, "avx2", _mm256_shuffle_epi8,
                         (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_reendianize!(u16x8, u8x16, "ssse3", _mm_shuffle_epi8,
                         (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14),
                         (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_reendianize!(i16x32, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                         (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30, 33, 32, 35, 34, 37, 36, 39, 38, 41, 40, 43, 42, 45, 44, 47, 46, 49, 48, 51, 50, 53, 52, 55, 54, 57, 56, 59, 58, 61, 60, 63, 62),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_reendianize!(i16x16, u8x32, "avx2", _mm256_shuffle_epi8,
                         (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_reendianize!(i16x8, u8x16, "ssse3", _mm_shuffle_epi8,
                         (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14),
                         (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_reendianize!(u32x16, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                         (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28, 35, 34, 33, 32, 39, 38, 37, 36, 43, 42, 41, 40, 47, 46, 45, 44, 51, 50, 49, 48, 55, 54, 53, 52, 59, 58, 57, 56, 63, 62, 61, 60),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_reendianize!(u32x8, u8x32, "avx2", _mm256_shuffle_epi8,
                         (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28),
                         (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_reendianize!(u32x4, u8x16, "ssse3", _mm_shuffle_epi8,
                         (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12),
                         (0, 1, 2, 3));
impl_packed_reendianize!(i32x16, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                         (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28, 35, 34, 33, 32, 39, 38, 37, 36, 43, 42, 41, 40, 47, 46, 45, 44, 51, 50, 49, 48, 55, 54, 53, 52, 59, 58, 57, 56, 63, 62, 61, 60),
                         (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_reendianize!(i32x8, u8x32, "avx2", _mm256_shuffle_epi8,
                         (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28),
                         (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_reendianize!(i32x4, u8x16, "ssse3", _mm_shuffle_epi8,
                         (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12),
                         (0, 1, 2, 3));
impl_packed_reendianize!(u64x8, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                         (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24, 39, 38, 37, 36, 35, 34, 33, 32, 47, 46, 45, 44, 43, 42, 41, 40, 55, 54, 53, 52, 51, 50, 49, 48, 63, 62, 61, 60, 59, 58, 57, 56),
                         (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_reendianize!(u64x4, u8x32, "avx2", _mm256_shuffle_epi8,
                         (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24),
                         (0, 1, 2, 3));
impl_packed_reendianize!(u64x2, u8x16, "ssse3", _mm_shuffle_epi8,
                         (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8),
                         (0, 1));
impl_packed_reendianize!(i64x8, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                         (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24, 39, 38, 37, 36, 35, 34, 33, 32, 47, 46, 45, 44, 43, 42, 41, 40, 55, 54, 53, 52, 51, 50, 49, 48, 63, 62, 61, 60, 59, 58, 57, 56),
                         (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_reendianize!(i64x4, u8x32, "avx2", _mm256_shuffle_epi8,
                         (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24),
                         (0, 1, 2, 3));
impl_packed_reendianize!(i64x2, u8x16, "ssse3", _mm_shuffle_epi8,
                         (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8),
                         (0, 1));

mod tests {
    use super::*;

    macro_rules! test_packed_swizzle {
        (($($vec:tt),*), ($($fn:tt),*)) => {
            $(
                #[test]
                fn $fn() {
                    let a = $vec::interleave(0u8 as <$vec as Packed>::Scalar,
                                             1u8 as <$vec as Packed>::Scalar);
                    let b = $vec::interleave(1u8 as <$vec as Packed>::Scalar,
                                             0u8 as <$vec as Packed>::Scalar);
                    assert_eq!(a.flip(), b);
                }
            )*
        }
    }

    macro_rules! test_packed_reendianize {
        (($($vec:tt),*), ($($fn:tt),*)) => {
            $(
                #[test]
                fn $fn() {
                    let a = $vec::interleave(33u8 as <$vec as Packed>::Scalar,
                                             92u8 as <$vec as Packed>::Scalar);
                    let b = $vec::interleave((33u8 as <$vec as Packed>::Scalar).swap_bytes(),
                                             (92u8 as <$vec as Packed>::Scalar).swap_bytes());
                    assert_eq!(a.reendianize(), b);
                }
            )*
        }
    }

    test_packed_swizzle!((u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2),
                         (flip_u8x64, flip_u8x32, flip_u8x16, flip_i8x64, flip_i8x32, flip_i8x16, flip_u16x32, flip_u16x16, flip_u16x8, flip_i16x32, flip_i16x16, flip_i16x8, flip_u32x16, flip_u32x8, flip_u32x4, flip_i32x16, flip_i32x8, flip_i32x4, flip_f32x16, flip_f32x8, flip_f32x4, flip_u64x8, flip_u64x4, flip_u64x2, flip_i64x8, flip_i64x4, flip_i64x2, flip_f64x8, flip_f64x4, flip_f64x2));

    test_packed_reendianize!((u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2),
                             (reendianize_u8x64, reendianize_u8x32, reendianize_u8x16, reendianize_i8x64, reendianize_i8x32, reendianize_i8x16, reendianize_u16x32, reendianize_u16x16, reendianize_u16x8, reendianize_i16x32, reendianize_i16x16, reendianize_i16x8, reendianize_u32x16, reendianize_u32x8, reendianize_u32x4, reendianize_i32x16, reendianize_i32x8, reendianize_i32x4, reendianize_u64x8, reendianize_u64x4, reendianize_u64x2, reendianize_i64x8, reendianize_i64x4, reendianize_i64x2));
}
