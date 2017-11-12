use stdsimd::vendor::*;
use stdsimd::simd::*;

pub trait PackedHadd {
    fn hadd(&self, other: Self) -> Self;
}

impl PackedHadd for f32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_ps(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3))
    }
}

impl PackedHadd for f64x2 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_pd(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1))
    }
}

impl PackedHadd for f32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_ps(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3),
                  self.extract(4) + self.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(4) + other.extract(5),
                  other.extract(6) + other.extract(7))    }
}

impl PackedHadd for f64x4 {
    #[inline(always)]
    #[cfg(target_feature = "avx")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_pd(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  other.extract(0) + other.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(2) + other.extract(3))
    }
}

impl PackedHadd for i16x8 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi16(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  self.extract(4) + self.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3),
                  other.extract(4) + other.extract(5),
                  other.extract(6) + other.extract(7))
    }
}

impl PackedHadd for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "ssse3")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm_hadd_epi32(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "ssse3"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3))
    }
}

#[cfg(target_feature = "avx2")]
impl PackedHadd for i16x16 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi16(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  self.extract(4) + self.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3),
                  other.extract(4) + other.extract(5),
                  other.extract(6) + other.extract(7),
                  self.extract(8) + self.extract(9),
                  self.extract(10) + self.extract(11),
                  self.extract(12) + self.extract(13),
                  self.extract(14) + self.extract(15),
                  other.extract(8) + other.extract(9),
                  other.extract(10) + other.extract(11),
                  other.extract(12) + other.extract(13),
                  other.extract(14) + other.extract(15))
    }
}


impl PackedHadd for i32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn hadd(&self, other: Self) -> Self {
        unsafe { _mm256_hadd_epi32(*self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn hadd(&self, other: Self) -> Self {
        Self::new(self.extract(0) + self.extract(1),
                  self.extract(2) + self.extract(3),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3),
                  self.extract(4) + self.extract(5),
                  self.extract(6) + self.extract(7),
                  other.extract(4) + other.extract(5),
                  other.extract(6) + other.extract(7))
    }
}
