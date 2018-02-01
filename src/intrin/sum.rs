use vecs::*;
use stdsimd::vendor::*;
use core_or_std::ops::Add;
use intrin::upcast::Upcast;
use intrin::cmp::PackedCmp;
use intrin::abs::PackedAbs;
use intrin::transmute::PackedTransmute;

pub trait PackedSum : Packed {
    /// Return a scalar equivalent to the sum of all elements of this vector.
    fn sum(&self) -> Self::Scalar;
}

pub trait PackedUpcastSum : Packed {
    /// Return a scalar equivalent to the sum of all elements of this vector,
    /// but collect the result in an i64 rather than the vector's type.
    fn sum_upcast(&self) -> i64;
}

// TODO: Specialization
// impl<T> PackedSum for T where T : Packed, T::Scalar : Add<T::Scalar, Output = T::Scalar>, T::Scalar : From<i8> {
//     #[inline(always)]
//     default fn sum(&self) -> Self::Scalar {
//         self.scalar_reduce(Self::Scalar::from(0i8), |acc, s| acc + s)
//     }
// }

// TODO: Specialization
// impl<T> PackedUpcastSum for T where T : Packed, T::Scalar : Add<i64, Output = i64>, i64 : From<T::Scalar> {
//     #[inline(always)]
//     default fn sum_upcast(&self) -> i64 {
//         self.scalar_reduce(0i64, |acc, s| acc + i64::from(s))
//     }
// }

#[cfg(target_feature = "sse2")]
impl PackedSum for i8x16 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        let pos = unsafe { _mm_sad_epu8(self.max(Self::splat(0)).be_u8s(), u8x16::splat(0)).be_u16s() };
        let neg = unsafe { _mm_sad_epu8(self.min(Self::splat(0)).abs().be_u8s(), u8x16::splat(0)).be_u16s() };
        pos.extract(0).overflowing_sub(neg.extract(0)).0
            .overflowing_add(pos.extract(4).overflowing_sub(neg.extract(4)).0).0 as i8
    }
}

#[cfg(target_feature = "sse2")]
impl PackedUpcastSum for i8x16 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        let pos = unsafe { _mm_sad_epu8(self.max(Self::splat(0)).be_u8s(), u8x16::splat(0)).be_u16s() };
        let neg = unsafe { _mm_sad_epu8(self.min(Self::splat(0)).abs().be_u8s(), u8x16::splat(0)).be_u16s() };
        pos.extract(0).overflowing_sub(neg.extract(0)).0
            .overflowing_add(pos.extract(4).overflowing_sub(neg.extract(4)).0).0 as i8 as i64
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSum for i8x32 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        let pos = unsafe { _mm256_sad_epu8(self.max(Self::splat(0)).be_u8s(), u8x32::splat(0)).be_u16s() };
        let neg = unsafe { _mm256_sad_epu8(self.min(Self::splat(0)).abs().be_u8s(), u8x32::splat(0)).be_u16s() };
        pos.extract(0).overflowing_sub(neg.extract(0)).0
            .overflowing_add(pos.extract(4).overflowing_sub(neg.extract(4)).0).0
            .overflowing_add(pos.extract(8).overflowing_sub(neg.extract(8)).0).0
            .overflowing_add(pos.extract(12).overflowing_sub(neg.extract(12)).0).0 as i8
    }
}

#[cfg(target_feature = "avx2")]
impl PackedUpcastSum for i8x32 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        let pos = unsafe { _mm256_sad_epu8(self.max(Self::splat(0)).be_u8s(), u8x32::splat(0)).be_u16s() };
        let neg = unsafe { _mm256_sad_epu8(self.min(Self::splat(0)).abs().be_u8s(), u8x32::splat(0)).be_u16s() };
        pos.extract(0).overflowing_sub(neg.extract(0)).0
            .overflowing_add(pos.extract(4).overflowing_sub(neg.extract(4)).0).0
            .overflowing_add(pos.extract(8).overflowing_sub(neg.extract(8)).0).0
            .overflowing_add(pos.extract(12).overflowing_sub(neg.extract(12)).0).0 as i8 as i64
    }
}

#[cfg(target_feature = "sse2")]
impl PackedSum for u8x16 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        let x = unsafe { _mm_sad_epu8(*self, Self::splat(0)).be_u16s() };
        (x.extract(0) + x.extract(4)) as u8
    }
}

#[cfg(target_feature = "sse2")]
impl PackedUpcastSum for u8x16 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        let x = unsafe { _mm_sad_epu8(*self, Self::splat(0)).be_u16s() };
        (x.extract(0) + x.extract(4)) as i64
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSum for u8x32 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        let x = unsafe { _mm256_sad_epu8(*self, Self::splat(0)).be_u16s() };
        (x.extract(0) + x.extract(4) + x.extract(8) + x.extract(12)) as u8
    }
}

#[cfg(target_feature = "avx2")]
impl PackedUpcastSum for u8x32 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        let x = unsafe { _mm256_sad_epu8(*self, Self::splat(0)).be_u16s() };
        (x.extract(0) + x.extract(4) + x.extract(8) + x.extract(12)) as i64
    }
}

#[cfg(target_feature = "ssse3")]
impl PackedSum for i16x8 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        let x =  unsafe {
            _mm_hadd_epi16(
                _mm_hadd_epi16(
                    _mm_hadd_epi16(*self, Self::splat(0)), Self::splat(0)), Self::splat(0))
        };
        x.extract(0)
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSum for i16x16 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        let x =  unsafe {
            _mm256_hadd_epi16(
                _mm256_hadd_epi16(
                    _mm256_hadd_epi16(*self, Self::splat(0)), Self::splat(0)), Self::splat(0))
        };
        x.extract(0) + x.extract(8)
    }
}

#[cfg(target_feature = "avx2")]
impl PackedUpcastSum for i16x16 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        unsafe {
            let (a, b) = self.upcast();
            let x =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(a.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            let y =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(b.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            (x.extract(0) + x.extract(4) + y.extract(0) + y.extract(4)) as i64
        }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSum for u16x16 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        unsafe {
            let (a, b) = self.upcast();
            let x =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(a.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            let y =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(b.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            (x.extract(0) + x.extract(4) + y.extract(0) + y.extract(4)) as u16
        }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedUpcastSum for u16x16 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        unsafe {
            let (a, b) = self.upcast();
            let x =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(a.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            let y =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(b.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            (x.extract(0) + x.extract(4) + y.extract(0) + y.extract(4)) as i64
        }
    }
}

#[cfg(target_feature = "avx2")]
impl PackedSum for i32x8 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        let x = unsafe {
            _mm256_hadd_epi32(
                _mm256_hadd_epi32(*self, Self::splat(0)), Self::splat(0))
        };
        x.extract(0) + x.extract(4)
    }
}

macro_rules! impl_packed_sum {
    ($($vec:tt),*) => {
        $(
            impl PackedSum for $vec {
                #[inline(always)]
                default fn sum(&self) -> Self::Scalar {
                    self.scalar_reduce(0 as Self::Scalar, |acc, s| acc + s)
                }
            }
        )*
    }
}

macro_rules! impl_packed_upcast_sum {
    ($($vec:tt),*) => {
        $(
            impl PackedUpcastSum for $vec {
                #[inline(always)]
                default fn sum_upcast(&self) -> i64 {
                    self.scalar_reduce(0i64, |acc, s| acc + (s as i64))
                }
            }
        )*
    }
}

impl_packed_sum!(u8x64, i8x64, u16x32, u16x8, i16x32, u32x16, u32x8, u32x4, i32x16, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2);
impl_packed_upcast_sum!(u8x64, i8x64, u16x32, u16x8, i16x32, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2);

#[cfg(not(target_feature = "avx2"))]
impl_packed_sum!(i8x32, u8x32, i16x16, u16x16, i32x8);
#[cfg(not(target_feature = "avx2"))]
impl_packed_upcast_sum!(i8x32, u8x32, i16x16, u16x16);

#[cfg(not(target_feature = "sse2"))]
impl_packed_sum!(i8x16, u8x16);
#[cfg(not(target_feature = "sse2"))]
impl_packed_upcast_sum!(i8x16, u8x16);

#[cfg(not(target_feature = "ssse3"))]
impl_packed_sum!(i16x8);
#[cfg(not(target_feature = "ssse3"))]
impl_packed_upcast_sum!();

mod tests {
    use super::*;

    macro_rules! test_packed_sum_int {
        ($vec:tt, $el:tt, $name:ident) => {
            #[test]
            fn $name() {
                // Try not to overflow
                let mut i = $el::min_value() / 64 + 1;

                while i < $el::max_value() / 64 - 1 {
                    let v = $vec::splat(i);
                    assert_eq!(v.sum(),
                               v.scalar_reduce(0 as $el, |acc, v| acc + v));
                    assert_eq!(v.sum_upcast(),
                               v.scalar_reduce(0 as i64, |acc, v| acc + (v as i64)));
                    i += $el::max_value() / 20;
                }
            }
        };
    }

    test_packed_sum_int!(u8x64, u8, test_packed_sum_u8x64);
    test_packed_sum_int!(u8x32, u8, test_packed_sum_u8x32);
    test_packed_sum_int!(u8x16, u8, test_packed_sum_u8x16);
    test_packed_sum_int!(i8x64, i8, test_packed_sum_i8x64);
    test_packed_sum_int!(i8x32, i8, test_packed_sum_i8x32);
    test_packed_sum_int!(i8x16, i8, test_packed_sum_i8x16);
    test_packed_sum_int!(u16x32, u16, test_packed_sum_u16x32);
    test_packed_sum_int!(u16x16, u16, test_packed_sum_u16x16);
    test_packed_sum_int!(u16x8, u16, test_packed_sum_u16x8);
    test_packed_sum_int!(i16x32, i16, test_packed_sum_i16x32);
    test_packed_sum_int!(i16x16, i16, test_packed_sum_i16x16);
    test_packed_sum_int!(i16x8, i16, test_packed_sum_i16x8);
    test_packed_sum_int!(u32x16, u32, test_packed_sum_u32x16);
    test_packed_sum_int!(u32x8, u32, test_packed_sum_u32x8);
    test_packed_sum_int!(u32x4, u32, test_packed_sum_u32x4);
    test_packed_sum_int!(i32x16, i32, test_packed_sum_i32x16);
    test_packed_sum_int!(i32x8, i32, test_packed_sum_i32x8);
    test_packed_sum_int!(i32x4, i32, test_packed_sum_i32x4);
    test_packed_sum_int!(u64x8, u64, test_packed_sum_u64x8);
    test_packed_sum_int!(u64x4, u64, test_packed_sum_u64x4);
    test_packed_sum_int!(u64x2, u64, test_packed_sum_u64x2);
    test_packed_sum_int!(i64x8, i64, test_packed_sum_i64x8);
    test_packed_sum_int!(i64x4, i64, test_packed_sum_i64x4);
    test_packed_sum_int!(i64x2, i64, test_packed_sum_i64x2);

    // test_packed_sum!(f32x16, f32, test_packed_sum_f32x16);
    // test_packed_sum!(f32x8, f32, test_packed_sum_f32x8);
    // test_packed_sum!(f32x4, f32, test_packed_sum_f32x4);

    // test_packed_sum!(f64x8, f64, test_packed_sum_f64x8);
    // test_packed_sum!(f64x4, f64, test_packed_sum_f64x4);
    // test_packed_sum!(f64x2, f64, test_packed_sum_f64x2);
}
