use vecs::*;
use vec_patterns::*;
use stdsimd::vendor::*;
use intrin::transmute::*;
use core_or_std::mem::transmute;

pub trait PackedMerge {
    fn merge_halves(&self, other: Self) -> Self;
    fn merge_interleaved(&self, other: Self) -> Self;
    fn merge_offset(&self, other: Self, offset: u8) -> Self;
}

macro_rules! impl_packed_merge {
    ($vec:ty, ($($a:expr),*), ($($b:expr),*), $($na:expr, $nb:expr),*) => {
        #[cfg(not(target_feature = "avx2"))]
        impl PackedMerge for $vec {

            #[inline(always)]
            fn merge_halves(&self, other: Self) -> Self {
                Self::new($(self.extract($a)),*,
                          $(other.extract($b)),*)
            }

            #[inline(always)]
            fn merge_interleaved(&self, other: Self) -> Self {
                Self::new($(self.extract($na), other.extract($nb)),*)
            }

            #[inline(always)]
            fn merge_offset(&self, other: Self, offset: u8) -> Self {
                assert!(offset < Self::WIDTH as u8);
                let mut ret = self.clone();
                for i in (offset as u32)..(Self::WIDTH as u32) {
                    ret = ret.replace(i, other.extract(i));
                }
                ret
            }
        }

        #[cfg(target_feature = "avx2")]
        impl PackedMerge for $vec {

            #[inline(always)]
            fn merge_halves(&self, other: Self) -> Self {
                unsafe {
                    transmute(_mm256_blendv_epi8(
                        self.be_i8s(), other.be_i8s(),
                        transmute(Self::halfs(0x00, 0xFF))))
                }

            }

            #[inline(always)]
            fn merge_interleaved(&self, other: Self) -> Self {
                unsafe {
                    transmute(_mm256_blendv_epi8(
                        self.be_i8s(), other.be_i8s(),
                        transmute(Self::interleave(0x00, 0xFF))))
                }
            }

            #[inline(always)]
            fn merge_offset(&self, other: Self, offset: u8) -> Self {
                unsafe {
                    transmute(_mm256_blendv_epi8(
                        self.be_i8s(), other.be_i8s(),
                        transmute(Self::partition(0x00, 0xFF, offset))))
                }
            }
        }
    }
}

impl_packed_merge!(u8x16, (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(u8x32, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
impl_packed_merge!(u8x64, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), (32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);

impl_packed_merge!(u16x8, (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(u16x16, (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(u16x32, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);

impl_packed_merge!(u32x4, (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(u32x8, (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(u32x16, (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

impl_packed_merge!(u64x2, (0), (1), 0, 1);
impl_packed_merge!(u64x4, (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(u64x8, (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);

impl_packed_merge!(i8x16, (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(i8x32, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
impl_packed_merge!(i8x64, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), (32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);

impl_packed_merge!(i16x8, (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(i16x16, (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
impl_packed_merge!(i16x32, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);

impl_packed_merge!(i32x4, (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(i32x8, (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(i32x16, (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

impl_packed_merge!(i64x2, (0), (1), 0, 1);
impl_packed_merge!(i64x4, (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(i64x8, (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);

impl_packed_merge!(f32x4, (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(f32x8, (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);
impl_packed_merge!(f32x16, (0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

impl_packed_merge!(f64x2, (0), (1), 0, 1);
impl_packed_merge!(f64x4, (0, 1), (2, 3), 0, 1, 2, 3);
impl_packed_merge!(f64x8, (0, 1, 2, 3), (4, 5, 6, 7), 0, 1, 2, 3, 4, 5, 6, 7);

mod tests {
    use vecs::*;
    use intrin::*;

    macro_rules! test_packed_merge {
        (($($vec:tt),*), ($($fn:ident),*)) => {
            $(
                #[test]
                fn $fn() {
                    let asc = 1i32 as <$vec as Packed>::Scalar;
                    let bsc = 2i32 as <$vec as Packed>::Scalar;
                    let a = $vec::splat(asc);
                    let b = $vec::splat(bsc);
                    assert_eq!(a.merge_interleaved(b), $vec::interleave(asc, bsc));
                    assert_eq!(b.merge_interleaved(a), $vec::interleave(bsc, asc));

                    assert_eq!(a.merge_halves(b), $vec::halfs(asc, bsc));
                    assert_eq!(b.merge_halves(a), $vec::halfs(bsc, asc));

                    for i in 0..$vec::WIDTH as u8 {
                        assert_eq!(a.merge_offset(b, i), $vec::partition(asc, bsc, i));
                        assert_eq!(b.merge_offset(a, i), $vec::partition(bsc, asc, i));
                    }
                }
            )*
        }
    }
    test_packed_merge!(
        (u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2),
        (merge_u8x64, merge_u8x32, merge_u8x16, merge_i8x64, merge_i8x32, merge_i8x16, merge_u16x32, merge_u16x16, merge_u16x8, merge_i16x32, merge_i16x16, merge_i16x8, merge_u32x16, merge_u32x8, merge_u32x4, merge_i32x16, merge_i32x8, merge_i32x4, merge_f32x16, merge_f32x8, merge_f32x4, merge_u64x8, merge_u64x4, merge_u64x2, merge_i64x8, merge_i64x4, merge_i64x2, merge_f64x8, merge_f64x4, merge_f64x2));


}
