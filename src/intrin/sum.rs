use crate::std::ops::Add;
use crate::vecs::*;
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

macro_rules! test_packed_sum {
    ($vec:tt, $el:tt, $name:ident) => {
        #[test]
        fn $name() {
            for i in -100..100 {
                let v = $vec::splat(i as $el);
                assert_eq!(v.sum(),
                           v.scalar_reduce(0 as $el, |acc, v| acc + v));
                assert_eq!(v.sum_upcast(),
                           v.scalar_reduce(0 as i64, |acc, v| acc + (v as i64)));
            }
        }
    };
}

