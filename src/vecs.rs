use stdsimd::simd::*;
// use typenum::{Sum, Prod, Quot, U2, U4, U8, U16, U32, U64, U128, U256, U512, Unsigned, UInt};
// use std::marker::PhantomData;
// use std::ops::Div;
use iters::{IntoPackedRefIterator, IntoPackedRefMutIterator, PackedIter, PackedIterator};

#[cfg(all(target_feature = "avx512"))]
const SIMD_SIZE: usize = 512;

#[cfg(all(target_feature = "avx2", not(target_feature="avx512")))]
const SIMD_SIZE: usize = 256;

#[cfg(all(target_feature = "sse2", not(target_feature="avx2")))]
const SIMD_SIZE: usize = 128;

pub trait Packed<T> {
    const WIDTH: usize;

    fn width(&self) -> usize {
        Self::WIDTH
    }

    fn load(data: &[T], offset: usize) -> Self;
    fn store(self, data: &mut [T], offset: usize);
}

// where UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>
//     : std::ops::Div<<T as vecs::Packable>::Size>
// Whether this type is able to be packed into a SIMD vector
pub trait Packable where Self : Sized {
    const SIZE: usize;
    type Vector : Packed<Self>;
}

impl<'a, T> PackedIterator for PackedIter<'a, T> where T : Packable {
    const WIDTH: usize = SIMD_SIZE / <T as Packable>::SIZE;
    type Vector = <T as Packable>::Vector;

    #[inline(always)]
    fn width(&self) -> usize {
        Self::WIDTH
    }

    #[inline(always)]
    fn scalar_len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Vector> {
        if self.position + self.width() > self.scalar_len() {
            None
        } else {
            let ret: Option<Self::Vector> = Some(Self::Vector::load(self.data, self.position));
            self.position += T::SIZE;
            ret
        }
    }
}

// where typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UInt<typenum::UTerm, typenum::B1>, typenum::B0>, typenum::B0>, typenum::B0>, typenum::B0>, typenum::B0>, typenum::B0>, typenum::B0>, typenum::B0>:
// std::ops::Div<<T as vecs::Packable>::Size> bound


macro_rules! impl_into_packed {
    ($el:ty, $vec:tt, $sz:expr, $feat:expr, $nfeat:expr) => (
        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl Packable for $el {
            const SIZE: usize = $sz;
            type Vector = $vec;
        }

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl Packed<$el> for $vec {
            const WIDTH: usize = $sz;

            #[inline(always)]
            fn load(data: &[$el], offset: usize) -> $vec {
                $vec::load(data, offset)
            }
            #[inline(always)]
            fn store(self, data: &mut [$el], offset: usize) {
                $vec::store(self, data, offset);
            }
        }


        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl<'a> IntoPackedRefIterator<'a> for &'a [$el] {
            type Iter = PackedIter<'a, $el>;

            #[inline(always)]
            fn simd_iter(&'a self) -> Self::Iter {
                PackedIter {
                    data: self,
                    position: 0,
                }
            }
        }

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl<'a> IntoPackedRefMutIterator<'a> for &'a mut [$el] {
            type Iter = PackedIter<'a, $el>;

            #[inline(always)]
            fn simd_iter_mut(&'a mut self) -> Self::Iter {
                PackedIter {
                    data: self,
                    position: 0,
                }
            }
        }
    );
}

impl_into_packed!(u8, u8x64, 64, "avx512", "avx1024");
impl_into_packed!(u8, u8x32, 32, "avx2", "avx512");
impl_into_packed!(u8, u8x16, 16, "sse2", "avx2");

impl_into_packed!(i8, i8x64, 64, "avx512", "avx1024");
impl_into_packed!(i8, i8x32, 32, "avx2", "avx512");
impl_into_packed!(i8, i8x16, 16, "sse2", "avx2");

impl_into_packed!(u16, u16x32, 32, "avx512", "avx1024");
impl_into_packed!(u16, u16x16, 16, "avx2", "avx512");
impl_into_packed!(u16, u16x8, 8, "sse2", "avx2");

impl_into_packed!(i16, i16x32, 32, "avx512", "avx1024");
impl_into_packed!(i16, i16x16, 16, "avx2", "avx512");
impl_into_packed!(i16, i16x8, 8, "sse2", "avx2");

impl_into_packed!(u32, u32x16, 16, "avx512", "avx1024");
impl_into_packed!(u32, u32x8, 8, "avx2", "avx512");
impl_into_packed!(u32, u32x4, 4, "sse2", "avx2");

impl_into_packed!(i32, i32x16, 16, "avx512", "avx1024");
impl_into_packed!(i32, i32x8, 8, "avx2", "avx512");
impl_into_packed!(i32, i32x4, 4, "sse2", "avx2");

impl_into_packed!(f32, f32x16, 16, "avx512", "avx1024");
impl_into_packed!(f32, f32x8, 8, "avx2", "avx512");
impl_into_packed!(f32, f32x4, 4, "sse2", "avx2");

impl_into_packed!(u64, u64x8, 8, "avx512", "avx1024");
impl_into_packed!(u64, u64x4, 4, "avx2", "avx512");
impl_into_packed!(u64, u64x2, 2, "sse2", "avx2");

impl_into_packed!(i64, i64x8, 8, "avx512", "avx1024");
impl_into_packed!(i64, i64x4, 4, "avx2", "avx512");
impl_into_packed!(i64, i64x2, 2, "sse2", "avx2");

impl_into_packed!(f64, f64x8, 8, "avx512", "avx1024");
impl_into_packed!(f64, f64x4, 4, "avx2", "avx512");
impl_into_packed!(f64, f64x2, 2, "sse2", "avx2");
