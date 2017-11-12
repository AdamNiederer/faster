use stdsimd::vendor::*;
use stdsimd::simd::*;
use std::mem::transmute;

pub trait PackedTransmute {
    type i8s;
    type u8s;
    type i16s;
    type u16s;
    type i32s;
    type u32s;
    type f32s;
    type i64s;
    type u64s;
    type f64s;

    fn be_i8s(&self) -> Self::i8s;
    fn be_u8s(&self) -> Self::u8s;
    fn be_i16s(&self) -> Self::i16s;
    fn be_u16s(&self) -> Self::u16s;
    fn be_i32s(&self) -> Self::i32s;
    fn be_u32s(&self) -> Self::u32s;
    // TODO: Remove possibility of signalling NaNs
    unsafe fn be_f32s_unchecked(&self) -> Self::f32s;
    fn be_i64s(&self) -> Self::i64s;
    fn be_u64s(&self) -> Self::u64s;
    // TODO: Remove possibility of signalling NaNs
    unsafe fn be_f64s_unchecked(&self) -> Self::f64s;
}

macro_rules! impl_packed_transmute {
    ($($t:ty,)* ... $u8s:ty, $i8s:ty, $u16s:ty, $i16s:ty, $u32s:ty, $i32s:ty,
     $f32s:ty, $u64s:ty, $i64s:ty, $f64s:ty, $feat:expr, $nfeat:expr) => (
        $(
            impl PackedTransmute for $t {
                type i8s = $i8s;
                type u8s = $u8s;
                type i16s = $i16s;
                type u16s = $u16s;
                type i32s = $i32s;
                type u32s = $u32s;
                type f32s = $f32s;
                type i64s = $i64s;
                type u64s = $u64s;
                type f64s = $f64s;

                #[inline(always)]
                fn be_i8s(&self) -> Self::i8s {
                    unsafe { transmute::<Self, Self::i8s>(*self) }
                }
                #[inline(always)]
                fn be_u8s(&self) -> Self::u8s {
                    unsafe { transmute::<Self, Self::u8s>(*self) }
                }
                #[inline(always)]
                fn be_i16s(&self) -> Self::i16s {
                    unsafe { transmute::<Self, Self::i16s>(*self) }
                }
                #[inline(always)]
                fn be_u16s(&self) -> Self::u16s {
                    unsafe { transmute::<Self, Self::u16s>(*self) }
                }
                #[inline(always)]
                fn be_i32s(&self) -> Self::i32s {
                    unsafe { transmute::<Self, Self::i32s>(*self) }
                }
                #[inline(always)]
                fn be_u32s(&self) -> Self::u32s {
                    unsafe { transmute::<Self, Self::u32s>(*self) }
                }
                #[inline(always)]
                unsafe fn be_f32s_unchecked(&self) -> Self::f32s {
                    transmute::<Self, Self::f32s>(*self)
                }
                #[inline(always)]
                fn be_i64s(&self) -> Self::i64s {
                    unsafe { transmute::<Self, Self::i64s>(*self) }
                }
                #[inline(always)]
                fn be_u64s(&self) -> Self::u64s {
                    unsafe { transmute::<Self, Self::u64s>(*self) }
                }
                #[inline(always)]
                unsafe fn be_f64s_unchecked(&self) -> Self::f64s {
                    transmute::<Self, Self::f64s>(*self)
                }
            }
        )*
    );
}

impl_packed_transmute!(u8x32, i8x32, u16x16, i16x16, u32x8, i32x8, f32x8,
                       u64x4, i64x4, f64x4, ...
                       u8x32, i8x32, u16x16, i16x16, u32x8, i32x8,
                       f32x8, u64x4, i64x4, f64x4,
                       "avx", "avx512");
impl_packed_transmute!(u8x64, i8x64, u16x32, i16x32, u32x16, i32x16, f32x16,
                       u64x8, i64x8, f64x8, ...
                       u8x64, i8x64, u16x32, i16x32, u32x16, i32x16,
                       f32x16, u64x8, i64x8, f64x8,
                       "avx512", "avx1024");
impl_packed_transmute!(u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, f32x4,
                       u64x2, i64x2, f64x2, ...
                       u8x16, i8x16, u16x8, i16x8, u32x4, i32x4,
                       f32x4, u64x2, i64x2, f64x2,
                       "sse", "avx");
