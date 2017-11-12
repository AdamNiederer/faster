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
        Self::new(this.extract(0) + this.extract(1),
                  this.extract(2) + this.extract(3),
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
        Self::new(this.extract(0) + this.extract(1),
                  this.extract(2) + this.extract(3),
                  this.extract(4) + this.extract(5),
                  this.extract(6) + this.extract(7),
                  other.extract(0) + other.extract(1),
                  other.extract(2) + other.extract(3),
                  other.extract(4) + other.extract(5),
                  other.extract(6) + other.extract(7),
                  this.extract(8) + this.extract(9),
                  this.extract(10) + this.extract(11),
                  this.extract(12) + this.extract(13),
                  this.extract(14) + this.extract(15),
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
