use stdsimd::simd::*;
use typenum::{U2, U4, U8, U16, U32, U64, Unsigned};
use std::marker::PhantomData;
use iters::{IntoPackedRefIterator, IntoPackedRefMutIterator, PackedIter};

pub trait Packed<T, S, N: Unsigned> {
    fn width(&self) -> usize {
        N::to_usize()
    }
    fn load(data: &[T], offset: usize) -> S;
    fn store(self, data: &mut [T], offset: usize);
}

macro_rules! impl_packed {
    ($el:ty, $vec:tt, $sz:ty) => (
        impl Packed<$el, $vec, $sz> for $vec {
            #[inline(always)]
            fn load(data: &[$el], offset: usize) -> $vec {
                $vec::load(data, offset)
            }
            #[inline(always)]
            fn store(self, data: &mut [$el], offset: usize) {
                $vec::store(self, data, offset);
            }
        }
    );
}

impl_packed!(u8, u8x64, U64);
impl_packed!(u8, u8x32, U32);
impl_packed!(u8, u8x16, U16);

impl_packed!(i8, i8x64, U64);
impl_packed!(i8, i8x32, U32);
impl_packed!(i8, i8x16, U16);

impl_packed!(u16, u16x32, U32);
impl_packed!(u16, u16x16, U16);
impl_packed!(u16, u16x8, U8);

impl_packed!(i16, i16x32, U32);
impl_packed!(i16, i16x16, U16);
impl_packed!(i16, i16x8, U8);

impl_packed!(u32, u32x16, U16);
impl_packed!(u32, u32x8, U8);
impl_packed!(u32, u32x4, U4);

impl_packed!(i32, i32x16, U16);
impl_packed!(i32, i32x8, U8);
impl_packed!(i32, i32x4, U4);

impl_packed!(f32, f32x16, U16);
impl_packed!(f32, f32x8, U8);
impl_packed!(f32, f32x4, U4);

impl_packed!(u64, u64x8, U8);
impl_packed!(u64, u64x4, U4);
impl_packed!(u64, u64x2, U2);

impl_packed!(i64, i64x8, U8);
impl_packed!(i64, i64x4, U4);
impl_packed!(i64, i64x2, U2);

impl_packed!(f64, f64x8, U8);
impl_packed!(f64, f64x4, U4);
impl_packed!(f64, f64x2, U2);

macro_rules! impl_into_packed {
    ($el:ty, $vec:ty, $sz:ty, $feat:expr, $nfeat:expr) => (
        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl<'a> IntoPackedRefIterator<'a> for &'a [$el] {
            type Item = $vec;
            type Iter = PackedIter<'a, $el, Self::Item, $sz>;

            #[inline(always)]
            fn simd_iter(&'a self) -> Self::Iter {
                PackedIter {
                    data: self,
                    position: 0,
                    __simd_data: PhantomData::<($vec, $sz)>
                }
            }
        }

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl<'a> IntoPackedRefMutIterator<'a> for &'a mut [$el] {
            type Item = $vec;
            type Iter = PackedIter<'a, $el, Self::Item, $sz>;

            #[inline(always)]
            fn simd_iter_mut(&'a mut self) -> Self::Iter {
                PackedIter {
                    data: self,
                    position: 0,
                    __simd_data: PhantomData::<($vec, $sz)>
                }
            }
        }
    );
}

impl_into_packed!(u8, u8x64, U64, "avx512", "avx1024");
impl_into_packed!(u8, u8x32, U32, "avx2", "avx512");
impl_into_packed!(u8, u8x16, U16, "sse2", "avx2");

impl_into_packed!(i8, i8x64, U64, "avx512", "avx1024");
impl_into_packed!(i8, i8x32, U32, "avx2", "avx512");
impl_into_packed!(i8, i8x16, U16, "sse2", "avx2");

impl_into_packed!(u16, u16x32, U32, "avx512", "avx1024");
impl_into_packed!(u16, u16x16, U16, "avx2", "avx512");
impl_into_packed!(u16, u16x8, U8, "sse2", "avx2");

impl_into_packed!(i16, i16x32, U32, "avx512", "avx1024");
impl_into_packed!(i16, i16x16, U16, "avx2", "avx512");
impl_into_packed!(i16, i16x8, U8, "sse2", "avx2");

impl_into_packed!(u32, u32x16, U16, "avx512", "avx1024");
impl_into_packed!(u32, u32x8, U8, "avx2", "avx512");
impl_into_packed!(u32, u32x4, U4, "sse2", "avx2");

impl_into_packed!(i32, i32x16, U16, "avx512", "avx1024");
impl_into_packed!(i32, i32x8, U8, "avx2", "avx512");
impl_into_packed!(i32, i32x4, U4, "sse2", "avx2");

impl_into_packed!(f32, f32x16, U16, "avx512", "avx1024");
impl_into_packed!(f32, f32x8, U8, "avx2", "avx512");
impl_into_packed!(f32, f32x4, U4, "sse2", "avx2");

impl_into_packed!(u64, u64x8, U8, "avx512", "avx1024");
impl_into_packed!(u64, u64x4, U4, "avx2", "avx512");
impl_into_packed!(u64, u64x2, U2, "sse2", "avx2");

impl_into_packed!(i64, i64x8, U8, "avx512", "avx1024");
impl_into_packed!(i64, i64x4, U4, "avx2", "avx512");
impl_into_packed!(i64, i64x2, U2, "sse2", "avx2");

impl_into_packed!(f64, f64x8, U8, "avx512", "avx1024");
impl_into_packed!(f64, f64x4, U4, "avx2", "avx512");
impl_into_packed!(f64, f64x2, U2, "sse2", "avx2");
