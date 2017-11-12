use stdsimd::simd::*;
use iters::{IntoPackedRefIterator, IntoPackedRefMutIterator, PackedIter,
            IntoUnevenPackedRefIterator, IntoUnevenPackedRefMutIterator, UnevenPackedIter};

#[cfg(all(target_feature = "avx512"))]
pub const SIMD_SIZE: usize = 512;

#[cfg(all(target_feature = "avx2", not(target_feature="avx512")))]
pub const SIMD_SIZE: usize = 256;

#[cfg(all(target_feature = "sse", not(target_feature="avx2")))]
pub const SIMD_SIZE: usize = 128;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(target_feature = "sse")))]
compile_error!("Your CPU is ancient! We don't support MMX, sorry!");

#[cfg(all(not(target_arch = "x86"), not(target_arch = "x86_64")))]
compile_error!("Support for non-x86 platforms is forthcoming - See the stdsimd issues tracker for more details.");

// A SIMD vector containing T.
pub trait Packed<T : Packable> : Sized {
    const WIDTH: usize;
    const ELEMENT_SIZE: usize = <T as Packable>::SIZE;
    const WIDTH_BYTES: usize = Self::WIDTH * Self::ELEMENT_SIZE / 8;

    #[inline(always)]
    fn width(&self) -> usize {
        Self::WIDTH
    }

    #[inline(always)]
    fn width_bytes(&self) -> usize {
        Self::WIDTH_BYTES
    }

    #[inline(always)]
    fn element_size(&self) -> usize {
        Self::ELEMENT_SIZE
    }

    fn load(data: &[T], offset: usize) -> Self;
    fn store(self, data: &mut [T], offset: usize);
    fn splat(data: T) -> Self;
}

// A type which may be packed into a SIMD vector
pub trait Packable where Self : Sized + Copy {
    const SIZE: usize;
    type Vector : Packed<Self>;
}

macro_rules! impl_packed {
    ($el:ty, $pvec:tt, $vec:tt, $sz:expr, $feat:expr, $nfeat:expr) => (
        #[allow(non_camel_case_types)]
        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        pub type $pvec = $vec;

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl Packable for $el {
            const SIZE: usize = SIMD_SIZE / $sz;
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
            #[inline(always)]
            fn splat(data: $el) -> Self {
                $vec::splat(data)
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

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl<'a> IntoUnevenPackedRefIterator<'a> for &'a [$el] {
            type Iter = UnevenPackedIter<'a, $el>;

            #[inline(always)]
            fn uneven_simd_iter(&'a self) -> Self::Iter {
                UnevenPackedIter {
                    data: self,
                    position: 0,
                }
            }
        }

        #[cfg(all(target_feature = $feat, not(target_feature = $nfeat)))]
        impl<'a> IntoUnevenPackedRefMutIterator<'a> for &'a mut [$el] {
            type Iter = UnevenPackedIter<'a, $el>;

            #[inline(always)]
            fn uneven_simd_iter_mut(&'a mut self) -> Self::Iter {
                UnevenPackedIter {
                    data: self,
                    position: 0,
                }
            }
        }

    );
}

impl_packed!(u8, u8s, u8x64, 64, "avx512", "avx1024");
impl_packed!(u8, u8s, u8x32, 32, "avx2", "avx512");
impl_packed!(u8, u8s, u8x16, 16, "sse", "avx2");

impl_packed!(i8, i8s, i8x64, 64, "avx512", "avx1024");
impl_packed!(i8, i8s, i8x32, 32, "avx2", "avx512");
impl_packed!(i8, i8s, i8x16, 16, "sse", "avx2");

impl_packed!(u16, u16s, u16x32, 32, "avx512", "avx1024");
impl_packed!(u16, u16s, u16x16, 16, "avx2", "avx512");
impl_packed!(u16, u16s, u16x8, 8, "sse", "avx2");

impl_packed!(i16, i16s, i16x32, 32, "avx512", "avx1024");
impl_packed!(i16, i16s, i16x16, 16, "avx2", "avx512");
impl_packed!(i16, i16s, i16x8, 8, "sse", "avx2");

impl_packed!(u32, u32s, u32x16, 16, "avx512", "avx1024");
impl_packed!(u32, u32s, u32x8, 8, "avx2", "avx512");
impl_packed!(u32, u32s, u32x4, 4, "sse", "avx2");

impl_packed!(i32, i32s, i32x16, 16, "avx512", "avx1024");
impl_packed!(i32, i32s, i32x8, 8, "avx2", "avx512");
impl_packed!(i32, i32s, i32x4, 4, "sse", "avx2");

impl_packed!(f32, f32s, f32x16, 16, "avx512", "avx1024");
impl_packed!(f32, f32s, f32x8, 8, "avx2", "avx512");
impl_packed!(f32, f32s, f32x4, 4, "sse", "avx2");

impl_packed!(u64, u64s, u64x8, 8, "avx512", "avx1024");
impl_packed!(u64, u64s, u64x4, 4, "avx2", "avx512");
impl_packed!(u64, u64s, u64x2, 2, "sse", "avx2");

impl_packed!(i64, i64s, i64x8, 8, "avx512", "avx1024");
impl_packed!(i64, i64s, i64x4, 4, "avx2", "avx512");
impl_packed!(i64, i64s, i64x2, 2, "sse", "avx2");

impl_packed!(f64, f64s, f64x8, 8, "avx512", "avx1024");
impl_packed!(f64, f64s, f64x4, 4, "avx2", "avx512");
impl_packed!(f64, f64s, f64x2, 2, "sse", "avx2");
