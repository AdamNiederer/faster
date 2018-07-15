use crate::arch::current::vecs::*;
use crate::intrin::transmute::*;
use crate::std::mem::transmute;

pub trait Reendianize : Sized + Copy {
    /// Return a vector containing elements of `self` with switched endianness.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u32s(0xDEADBEEF).swap_bytes(), u32s(0xEFBEADDE));
    /// # }
    /// ```
    fn swap_bytes(&self) -> Self;

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn to_be(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn to_be(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn to_le(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn to_le(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn from_be(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn from_be(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn from_le(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn from_le(&self) -> Self {
        *self
    }
}


macro_rules! impl_packed_swap_bytes {
    ($vec:tt, $uvec:tt, $feat:expr, $mmfn:tt, ($($c:expr),*), ($($a:expr, $b:expr),*)) => {
        impl Reendianize for $vec {
            #[cfg(not(target_feature = $feat))]
            #[inline(always)]
            fn swap_bytes(&self) -> Self {
                $vec::new($(self.extract($a).swap_bytes(),
                            self.extract($b).swap_bytes()),*)
            }

            #[cfg(target_feature = $feat)]
            #[inline(always)]
            fn swap_bytes(&self) -> Self {
                unsafe {
                    transmute($mmfn(self.be_i8s(), $uvec::new($($c),*).be_i8s()))
                }
            }
        }
    }
}
