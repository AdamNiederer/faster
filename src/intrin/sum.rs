use crate::vecs::*;
use crate::std::ops::Add;
use crate::intrin::upcast::Upcast;
use crate::intrin::cmp::Cmp;
use crate::intrin::abs::Abs;
use crate::intrin::transmute::Transmute;

pub trait Sum : Packed {
    /// Return a scalar equivalent to the sum of all elements of this vector.
    fn sum(&self) -> Self::Scalar;
}

pub trait UpcastSum :  {
    /// Return a scalar equivalent to the sum of all elements of this vector,
    /// but collect the result in an i64 rather than the vector's type.
    fn sum_upcast(&self) -> i64;
}


macro_rules! impl_packed_sum {
    ($($vec:tt),*) => {
        $(
            impl Sum for $vec {
                #[inline(always)]
                fn sum(&self) -> Self::Scalar {
                    self.scalar_reduce(0 as Self::Scalar, |acc, s| acc + s)
                }
            }
        )*
    }
}

macro_rules! impl_packed_upcast_sum {
    ($($vec:tt),*) => {
        $(
            impl UpcastSum for $vec {
                #[inline(always)]
                fn sum_upcast(&self) -> i64 {
                    self.scalar_reduce(0i64, |acc, s| acc + (s as i64))
                }
            }
        )*
    }
}
